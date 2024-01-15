#!/bin/bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/intro.wasm --out-dir web --target web
wasm-opt -Oz ./web/intro_bg.wasm -o ./web/intro.wasm