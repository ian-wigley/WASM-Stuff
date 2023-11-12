// Grow memory by 37 pages (128Kb)
memory.grow(37);

export function go(): void {

  let start: i32 = 20 * 600 * 4;
  let x: i32 = 0;
  for (let y: i32 = 0; y < 10; y++) {
    for (x = 0; x < 600 * 4; x += 4) {
      store<u8>(start + x, 255);
      store<u8>(start + x + 1, 0);
      store<u8>(start + x + 2, 0);
      store<u8>(start + x + 3, 255);
    }
    start += x;
  }
}

// npm run asbuild
// npm run start