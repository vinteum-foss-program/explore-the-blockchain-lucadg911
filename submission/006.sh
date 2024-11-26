#!/bin/bash
cd rust_programs
cargo build --release
cd target/release
./rust_programs 006