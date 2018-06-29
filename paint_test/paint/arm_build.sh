#!/bin/sh

#set -ex

# Compile our wasm module
# cargo build --target aarch64-linux-android --release;
cargo build --target arm-linux-androideabi --release;

#cp  ./target/arm-linux-androideabi/release/libmain.so ../TestJNA/app/libs/armeabi/libmain.so;
#cp  ./target/arm-linux-androideabi/release/libmain.so ../TestJNA/app/src/main/jniLibs/armeabi/libmain.so;
# cp  ./target/aarch64-linux-android/release/libpaint.so ../../TestJNA/app/libs/arm64-v8a/libpaint.so;



