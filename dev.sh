#!/usr/bin/env bash

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

. ./sh/pid.sh

set -ex

if ! hash watchexec 2>/dev/null; then
cargo install watchexec-cli
fi

cargo build || true

RUST_BACKTRACE=1 exec watchexec \
  --shell=none \
  -w src \
  -w demo \
  -c -r --exts rs,toml,coffee,mjs \
  --ignore target/ \
  -- ./run.sh
