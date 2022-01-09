#!/usr/bin/env bash

set -e

echo "Fixing app:" >&2
if cd app && yarn -s install && yarn -s lint && cd ..; then
  echo "Done" >&2
  echo >&2
else
  exit $?
fi

echo "Fixing server:" >&2
if cargo clippy --no-deps; then
  echo "Done" >&2
else
  exit $?
fi

