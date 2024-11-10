#!/bin/sh
if [ $# -lt 1 ]; then exit 1; fi
mkdir -p $(pwd)/testcases/$1
cargo compete test --manifest-path $(pwd)/Cargo.toml $1
