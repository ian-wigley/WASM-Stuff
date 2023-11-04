#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/colour_bars.wasm

cargo build --target $TARGET --release
wasm-strip $BINARY
mkdir -p web/
wasm-opt -o web/colour_bars.wasm -Oz $BINARY
ls -lh web/colour_bars.wasm
