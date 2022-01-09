#!/usr/bin/env bash

set -e

echo "Installing app:" >&2
if cd app && yarn -s install && cd ..; then
  echo "Done" >&2
  echo >&2
else
  exit $?
fi

echo "Installing server:" >&2
if cargo fetch; then
  echo "Done" >&2
else
  exit $?
fi

