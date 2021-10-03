#!/bin/sh

VERSION=0.2.0
TARGET_DIR=./release/macos/chrust-$VERSION/
BUILD_TARGET=x86_64-apple-darwin

cargo build --target $BUILD_TARGET --release

mkdir -p $TARGET_DIR && cp ./target/$BUILD_TARGET/release/chrust $TARGET_DIR