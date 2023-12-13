cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/stars.wasm --out-dir web --target web
wasm-opt -Oz ./web/stars_bg.wasm -o ./web/stars.wasm