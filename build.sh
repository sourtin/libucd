#!/bin/bash
if [ ! -d tests/data ]; then
    mkdir -p tests/data
    cd data
    ./gen_tests.py
    cd ..
fi
cargo build --release --verbose
cargo test --release --verbose
