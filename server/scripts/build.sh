#!/bin/sh

. ./scripts/prelude.sh

echo "Building frontend:" >&2
if cd app && yarn -s build && cd ..; then
  echo "Done" >&2
  echo >&2
else
  exit $?
fi

echo "Building backend:" >&2
if cargo build --release; then
  echo "Done"
else
  exit $?
fi

