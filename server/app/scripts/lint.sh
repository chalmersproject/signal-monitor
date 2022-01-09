#!/usr/bin/env bash

set -euo pipefail

echo "Checking types..." >&2
tsc --noEmit

echo "Checking formatting..."
prettier --check --loglevel warn .

echo "Checking code..."
eslint .
