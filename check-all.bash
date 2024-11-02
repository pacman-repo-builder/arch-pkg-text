#! /bin/bash
set -o errexit -o pipefail -o nounset
cd "$(dirname "$0")"

just fmt

for task in doc clippy check test; do
  just $task "$@"
  just $task --no-default-features "$@"
  just $task --all-features "$@"
  just $task --features std "$@"
  just $task --features parking_lot "$@"
done
