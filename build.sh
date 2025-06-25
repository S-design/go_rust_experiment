#!/bin/bash

set -e
echo "building the rust library"
cd rust_lib
cargo build --release
echo "done and dusted"