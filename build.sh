#!/bin/bash
if [ ! -d tests/data ] || [ $(ls tests/data|wc -l) -lt 99 ]; then
    echo Building test data...
    mkdir -p tests/data
    cd data
    python3 ./gen_tests.py
    cd ..
else
    echo Using cached test data...
    ls tests/data
    echo
fi
cargo build --release --verbose
cargo test --release --verbose
