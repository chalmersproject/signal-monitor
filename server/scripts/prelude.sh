set -euo pipefail

assert_installed() {
  if ! command -v $1 > /dev/null; then
    echo "Missing program '$1' (requires installation)" >&2
    exit 1
  fi
}

assert_installed volta
assert_installed cargo
