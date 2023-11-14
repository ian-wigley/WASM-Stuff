set %TARGET=wasm32-unknown-unknown
set %BINARY=target/TARGET/release/colour_bars.wasm

cargo build --target TARGET --release
wasm-strip BINARY
mkdir -p web/
wasm-opt -o web/colour_bars.wasm -Oz BINARY