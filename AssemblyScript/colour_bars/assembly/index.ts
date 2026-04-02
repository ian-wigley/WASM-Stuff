// Grow memory by 30 pages (1 page = 64KB)
memory.grow(30);

const numberOfColourValues = 31;
const greyColourBar = new Uint8Array(numberOfColourValues);
greyColourBar.set([
  0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
  0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00
]);

// const letterA = new Uint8Array(16);
// letterA.set([
//   0xff, 0x00, 0x00, 0x00,
//   0xff, 0xff, 0x00, 0x00,
//   0xff, 0x00, 0xff, 0x00,
//   0xff, 0xff, 0x00, 0xff
// ]);

let yScreenPosition: u32 = 16;
const canvasWidth: u32 = 600;
const numOfBytes: u32 = 4;
const bites: u32 = 54 * numOfBytes / 2;
let xx: u32 = 0;

export function go(): void {
  let start: u32 = yScreenPosition * canvasWidth * numOfBytes;
  let x: u32 = 0;
  for (let y: i32 = 0; y < numberOfColourValues; y++) {
    for (x = 0; x < canvasWidth * numOfBytes; x += numOfBytes) {
      store<u8>(start + x, greyColourBar[y]);
      store<u8>(start + x + 1, greyColourBar[y]);
      store<u8>(start + x + 2, greyColourBar[y]);
      store<u8>(start + x + 3, 255);
    }
    start += x;
  }
  if (yScreenPosition < 640) {
    yScreenPosition += 1;
  }
  else {
    yScreenPosition = 16;
  }

  let count = 0;
  let yPosition: u32 = 250 * canvasWidth * numOfBytes;
  for (x = xx; x < bites + xx; x += numOfBytes) {
    store<u8>(yPosition + x, greyColourBar[count]);
    store<u8>(yPosition + x + 1, greyColourBar[count]);
    store<u8>(yPosition + x + 2, greyColourBar[count]);
    store<u8>(yPosition + x + 3, 255);
    count += 1;
  }

  // Move the bar across the screen
  if (xx < canvasWidth * numOfBytes) {
    xx += 8;
  }
  else {
    for (x = 2000; x < 3000; x += numOfBytes) {
      store<u8>(yPosition + x, 0);
      store<u8>(yPosition + x + 1, 0);
      store<u8>(yPosition + x + 2, 0);
      store<u8>(yPosition + x + 3, 255);
    }
    xx = 0;
  }

  // for (let i: u32 = 0; i < 8; i += numOfBytes) {
  //   store<u8>(start + i, letterA[i]);
  //   store<u8>(start + i + 1, letterA[i + 1]);
  //   store<u8>(start + i + 2, letterA[i + 2]);
  //   store<u8>(start + i + 3, letterA[i + 3]);
  // }

}

//npx asc assembly/index.ts \
//  -o build/module.wasm \
//  --debug \
//  --sourceMap

// npm run asbuild
// asc assembly/index.ts --target debug --sourceMap
// npx serve
// Open http://localhost:3000 in Chrome

// npm run asbuild:debug
// npm run start


////////////////////////////////////////////
////////////////////////////////////////////
//
// npx serve
// Open http://localhost:3000 in Chrome
//
////////////////////////////////////////////
////////////////////////////////////////////