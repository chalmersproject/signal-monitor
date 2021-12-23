#!/bin/sh

. ./scripts/prelude.sh

if ! cd app && yarn -s install && cd ..; then
  echo "Failed to install frontend" >&2
  exit $?
fi

if ! command -v cargo-watch > /dev/null; then
  echo "Installing cargo-watch..." >&2
  if cargo install cargo-watch; then
    echo "Done" >&2
    echo >&2
  else
    exit $?
  fi
fi

cargo watch -q -i app -x run
