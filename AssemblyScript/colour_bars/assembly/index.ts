// Grow memory by 30 pages (1 page = 64KB)
memory.grow(30);

const numberOfValues = 31;
const greyColourBar = new Uint8Array(numberOfValues);
greyColourBar.set([
  0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
  0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00
]);

let startY: u32 = 16;
const canvasWidth: u32 = 600;
const numOfBytes: u32 = 4;

export function go(): void {
  let start: u32 = startY * canvasWidth * numOfBytes;
  let x: u32 = 0;
  for (let y: i32 = 0; y < numberOfValues; y++) {
    for (x = 0; x < canvasWidth * numOfBytes; x += numOfBytes) {
      store<u8>(start + x, greyColourBar[y]);
      store<u8>(start + x + 1, greyColourBar[y]);
      store<u8>(start + x + 2, greyColourBar[y]);
      store<u8>(start + x + 3, 255);
    }
    start += x;
  }
  if (startY < 640) {
    startY += 1;
  }
  else {
    startY = 16;
  }
}

// npm run asbuild
// npm run start