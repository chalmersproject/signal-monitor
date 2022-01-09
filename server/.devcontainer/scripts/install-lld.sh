#!/usr/bin/env bash

set -e

sudo apt-get -y install lld

echo <<EOF > /usr/local/cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
EOF

sudo apt-get -y clean
