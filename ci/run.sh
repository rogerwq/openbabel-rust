#!/bin/sh

set -ex

cargo test --target $TARGET --no-run
 
if [ -z "$NO_RUN" ]; then
    cargo test --lib -- --test-threads=1
fi