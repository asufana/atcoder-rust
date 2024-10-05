#!/bin/sh
if [ $# -lt 1 ]; then exit 1; fi
cargo compete submit --manifest-path $(pwd)/Cargo.toml $1 --no-test
