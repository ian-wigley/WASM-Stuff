#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const NUM_COLOURS: usize = 16;

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
static COLOURS: [u32; NUM_COLOURS] = [
    0xff_11_11_11,
    0xff_33_33_33,
    0xff_55_55_55,
    0xff_77_77_77,
    0xff_99_99_99,
    0xff_bb_bb_bb,
    0xff_cc_cc_cc,
    0xff_ff_ff_ff,
    0xff_cc_cc_cc,
    0xff_bb_bb_bb,
    0xff_99_99_99,
    0xff_77_77_77,
    0xff_55_55_55,
    0xff_33_33_33,
    0xff_11_11_11,
    0xff_00_00_00,
];

static FRAME: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub unsafe extern "C" fn go() {
    render_frame_safe(&mut BUFFER, COLOURS)
}


fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT], colours: [u32; NUM_COLOURS]) {
    let _f = FRAME.fetch_add(1, Ordering::Relaxed);
    let mut count: usize = 0;
    let col = 0xFF_00_00_00;

    for y in 0..HEIGHT {
        if y >= 100 && y <= 100 + 14 {
            count += 1;
        }

        for x in 0..WIDTH {
            if y < 100 || y > 100 + 14 {
                buffer[y * WIDTH + x] = col;
            } else {
                buffer[y * WIDTH + x] = colours[count];
            }
        }
    }
}

#[no_mangle]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
