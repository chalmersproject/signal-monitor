# chalmersproject/signal-monitor

_Monitoring and administration for Chalmers Signals_

## Development

### Setup

_This project uses [Devcontainers](https://code.visualstudio.com/docs/remote/containers)
to standardize a development environment._

1. Ensure you have the following tools:

    - [Docker](https://docs.docker.com/get-docker/)
    - [VS Code](https://code.visualstudio.com)
        - Extension: [Remote Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

2. Open the desired workspace
   (i.e. [`./server/signal-monitor-server.code-worskpace`](./server/signal-monitor-server.code-workspace)) in VS Code:

    ```bash
    code ./server/signal-monitor-server.code-workspace
    ```

3. Open the command palette in VS Code (`ctrl+p` or `cmd+p`), and select:
    ```
    Remote-Containers: Open Workspace in Container...
    ```
    The container should have all development tools pre-installed and
    pre-configured, and install any dependencies on startup.

### Workflow

#### [`./server`](./server/)

```bash
./scripts/run.sh       # Run (development) with live-reloading
./scripts/run-prod.sh  # Run (production)
./scripts/lint.sh      # Check code style and report problems
./scripts/lint-fix.sh  # Auto-fix code style and problems

psql  # Connect to development Postgres

sqlx migrate add -r <description>  # Add a migration
sqlx migrate run # Run pending migrations
sqlx database reset # Reset database

sea-orm generate entity -o src/entities


```
