#!/usr/bin/env zsh

source /etc/zsh/zshrc /usr/local/cargo/env

set -e

cargo install cargo-edit --features vendored-openssl
cargo install cargo-watch
cargo install cargo-cache
cargo install sqlx-cli --no-default-features --features rustls,postgres
cargo install sea-orm-cli --no-default-features --features runtime-async-std-rustls && ln -s "$(which sea-orm-cli)" /usr/bin/sea-orm
cargo cache -a
