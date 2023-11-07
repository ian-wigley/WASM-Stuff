# This is a small collection of Rust projects for playing with Rust & Wasm in a browser

Uploaded to git as a personal helper guide ... for future reference !

### <ins>Initialise a Rust Wasm project</ins>
``` shell
cargo new --lib colour_bars
cd colour_bars
code .
```
Update the Cargo.toml
``` shell
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.86"
```

### <ins>Reading wasm files via the instantiateStreaming method</ins>

index.html:
``` html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8" />
  <title>Image Colour Bar - From Rust</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">

  <script type="module">

    const importObj = {};
    async function init() {
      const { instance } = await WebAssembly.instantiateStreaming(
        fetch('./pkg/colour_bars_bg.wasm'), importObj);

      const width = 600;
      const height = 600;

      const canvas = document.getElementById("demo-canvas");
      canvas.width = width;
      canvas.height = height;

      const buffer_address = instance.exports.BUFFER.value;
      const image = new ImageData(
          new Uint8ClampedArray(
              instance.exports.memory.buffer,
              buffer_address,
              4 * width * height,
          ),
          width,
      );

      const ctx = canvas.getContext("2d");

      const render = () => {
        instance.exports.go();
        ctx.putImageData(image, 0, 0);
        requestAnimationFrame(render);
      };

      render();
      instance.exports.add(10, 20);
    }

    init();
  </script>
</head>

<body>
  <canvas id="demo-canvas"></canvas>
</body>

</html>
```


### <ins> The WASM-Pack way
index.html:
``` html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8" />
  <title>Image Colour Bar - From Rust</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>

<body>
  <script type="module" src="./index.js"></script>
</body>

</html>
```
index.js:
``` javascript

// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "./pkg/colour_bars.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await init("./pkg/colour_bars_bg.wasm");

  // Call the Add function export from wasm, save the result
  const addResult = helloWorld.add(24, 12);

  // Set the result onto the body
  document.body.textContent = `Hello World! addResult: ${addResult}`;
};

runWasm();
```

### <ins> The Fetch-Compile-Instantiate-Run way
index.html:
``` html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8" />
  <title>Image Colour Bar - From Rust</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <script type="module">
    const importObj = {};
    fetch('./pkg/colour_bars_bg.wasm')
      .then(res => {
        if (res.ok)
          return res.arrayBuffer();
        throw new Error(`Unable to fetch WASM.`);
      })
      .then(bytes => {
        return WebAssembly.compile(bytes);
      })
      .then(module => {
        return WebAssembly.instantiate(module, importObj);
      })
      .then(instance => {
        let result = instance.exports.add(10, 20);
        console.log(result);
      });
  </script>
</head>
</html>
```

> **Tip:** Add ```const importObj = {};``` to avoid promise errors.

### <ins>Compile the wasm file</ins>
using wasm-pack:
``` shell
wasm-pack build --taget web
```
create a script:
``` shell
#!/bin/bash
set -euo pipefail
TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/colour_bars.wasm
cargo build --target $TARGET --release
wasm-strip $BINARY
mkdir -p web/
wasm-opt -o web/colour_bars.wasm -Oz $BINARY
ls -lh web/colour_bars.wasm
```

> **Tip:** To keep the file size to a minimum define a panic handler :
``` rust
#![no_std]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
```