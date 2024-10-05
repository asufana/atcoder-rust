#!/bin/sh
if [ $# -lt 1 ]; then exit 1; fi
cargo compete test --manifest-path $(pwd)/Cargo.toml $1
