#!/usr/bin/env bash

set -e

echo "Building app:" >&2
if cd app && yarn -s build && cd ..; then
  echo "Done" >&2
  echo >&2
else
  exit $?
fi

echo "Running server:" >&2
cargo run --release
