#!/bin/bash
mkdir -p tests/data
cd data
./gen_tests.py
cd ..
cargo build --release --verbose
cargo test --release --verbose
