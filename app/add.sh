#!/bin/sh
if [ $# -lt 1 ]; then exit 1; fi
cargo compete new $1
cargo member include $1