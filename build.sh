#!/usr/bin/env bash
echo PWD=$PWD
set -e
set -x
cargo build --release
#cargo install diesel_cli --no-default-features --features postgres
#diesel database setup
echo "Done."
