#!/bin/sh

#set -ex

# Compile our wasm module
cargo build --target aarch64-linux-android --release;

cp  ./target/aarch64-linux-android/release/libmain.so ../TestJNA/app/libs/arm64-v8a/libmain.so
