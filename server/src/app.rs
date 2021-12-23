use super::*;

use tokio::fs::canonicalize;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader, Lines};
use tokio::process::Child as ChildProcess;
use tokio::process::Command;
use tokio::sync::oneshot::channel as oneshot;
use tokio::sync::oneshot::Sender as OneshotSender;

use std::net::SocketAddr;
use std::process::Stdio;

use config::Environment;

#[derive(Debug, Default)]
pub struct Server {
    env: Environment,
}

impl Server {
    pub fn new(env: Environment) -> Self {
        Server { env }
    }

    pub async fn serve(self, addr: &SocketAddr) -> Result<()> {
        let Server { env } = self;
        let dir = canonicalize("./app")
            .await
            .context("failed to locate app directory")?;
        let mut app = Command::new("yarn")
            .arg("-s")
            .arg({
                use Environment::*;
                match env {
                    Development => "dev",
                    Production => "start",
                }
            })
            .arg("-p")
            .arg(addr.port().to_string())
            .env("NODE_ENV", env.to_string())
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Create ready notifier
        let (ready_tx, ready_rx) = oneshot::<()>();

        // Capture stdout
        let stdout = app
            .stdout
            .take()
            .expect("app did not have a handle to stdout");
        spawn(async move {
            let lines = BufReader::new(stdout).lines();
            if let Err(error) = capture_stdout(lines, ready_tx).await {
                error!(?error, "failed to read stdout");
            }
        });

        // Capture stderr
        let stderr = app
            .stderr
            .take()
            .expect("app did not have a handle to stderr");
        spawn(async move {
            let lines = BufReader::new(stderr).lines();
            if let Err(error) = capture_stderr(lines).await {
                error!(?error, "failed to read stderr");
            }
        });

        // Wait for app to be ready
        ready_rx.await.unwrap();
        Ok(())
    }
}

static IGNORE_LINE_PREFIXES: [&str; 1] = ["warn  - SWC minify beta enabled"];

async fn capture_stdout<R>(mut lines: Lines<R>, ready_tx: OneshotSender<()>) -> Result<()>
where
    R: AsyncBufRead,
    R: Unpin,
{
    // Wait for app to be ready
    let ready_line = match lines.next_line().await? {
        Some(line) => line,
        None => return Ok(()),
    };
    ensure!(
        ready_line.starts_with("ready - started server"),
        "expected ready line, received: {}",
        &ready_line
    );
    ready_tx.send(()).unwrap();

    // Capture output
    while let Some(line) = lines.next_line().await? {
        let should_ignore = IGNORE_LINE_PREFIXES
            .iter()
            .any(|prefix| line.starts_with(*prefix));
        if should_ignore {
            continue;
        }
        if line.starts_with("event - ") || line.starts_with("wait  - ") {
            debug!("{}", line);
            continue;
        }
        info!("{}", &line);
    }
    Ok(())
}

async fn capture_stderr<R>(mut lines: Lines<R>) -> Result<()>
where
    R: AsyncBufRead,
    R: Unpin,
{
    while let Some(line) = lines.next_line().await? {
        let should_ignore = IGNORE_LINE_PREFIXES
            .iter()
            .any(|prefix| line.starts_with(*prefix));
        if should_ignore {
            continue;
        }
        error!(target: "server::app", "{}", &line);
    }
    Ok(())
}
