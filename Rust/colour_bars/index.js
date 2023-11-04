import init from "./pkg/colour_bars.js";

const runWasm = async () => {

  const helloWorld = await init("./pkg/colour_bars_bg.wasm");
  const width = 600;
  const height = 600;

  document.body.innerHTML = "<canvas id='canvas' width=800 height=800>";
  const canvas = document.getElementById("canvas");

  const buffer_address = helloWorld.BUFFER.value;
  const image = new ImageData(
    new Uint8ClampedArray(
      helloWorld.memory.buffer,
      buffer_address,
      4 * width * height,
    ),
    width,
  );

  const ctx = canvas.getContext("2d");

  const render = () => {
    helloWorld.go();
    ctx.putImageData(image, 0, 0);
    requestAnimationFrame(render);
  };

  render();
};

runWasm();