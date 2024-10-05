#!/bin/sh
if [ $# -lt 1 ]; then exit 1; fi
watch "../test.sh $1"
