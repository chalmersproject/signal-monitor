FROM mcr.microsoft.com/vscode/devcontainers/base:debian

# Install tools
COPY ./scripts/ ./scripts/
RUN ./scripts/install-git.sh system
RUN ./scripts/install-docker.sh
RUN ./scripts/install-postgres.sh 14
RUN ./scripts/install-node.sh 16
RUN ./scripts/install-rust.sh 1.57
RUN ./scripts/install-lld.sh
RUN ./scripts/install-rust-tools.sh

# Persist caches
VOLUME ["/root/.vscode-server/extensions"]
VOLUME ["/usr/local/cargo/git", "/usr/local/cargo/registry"]
VOLUME ["/usr/local/share/.cache/yarn/v6"]

# Customize shell
RUN echo "set -o vi" >> ~/.zshrc && \
    chsh -s /bin/zsh
