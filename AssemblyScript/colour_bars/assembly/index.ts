// Grow memory by 37 pages (128Kb)
let pointer = memory.grow(37);
memory.fill(pointer, 255, 37);

const numberOfValues = 31;
const colours = new Uint8Array(numberOfValues);
colours.set([
  0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
  0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00
]);

export function go(): void {
  let start: u32 = 50 * 600 * 4;
  let x: u32 = 0;
  for (let y: i32 = 0; y < numberOfValues; y++) {
    for (x = 0; x < 600 * 4; x += 4) {
      store<u8>(start + x, colours[y]);
      store<u8>(start + x + 1, colours[y]);
      store<u8>(start + x + 2, colours[y]);
      store<u8>(start + x + 3, 255);
    }
    start += x;
  }
}

// npm run asbuild
// npm run start