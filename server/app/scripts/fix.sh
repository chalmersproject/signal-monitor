#!/bin/sh

set -euo pipefail

echo "Fixing formatting..." >&2
prettier --write --loglevel warn .

echo "Fixing code..." >&2
eslint --fix --quiet .
