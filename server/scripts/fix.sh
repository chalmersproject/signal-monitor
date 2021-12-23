#!/bin/sh

. ./scripts/prelude.sh

echo "Fixing frontend:" >&2
if cd app && yarn -s install && yarn -s lint && cd ..; then
  echo "Done" >&2
  echo >&2
else
  exit $?
fi

echo "Fixing backend:" >&2
if cargo clippy --no-deps; then
  echo "Done" >&2
else
  exit $?
fi

