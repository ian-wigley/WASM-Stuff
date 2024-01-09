use std::f64::consts::PI;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone)]
pub struct Letter {
    pub(crate) ascii: u8,
    pub(crate) screen_position_x: i32,
    pub(crate) screen_position_y: f64,
    pub(crate) x: i32,
    pub(crate) y: i32,
    count: i32,
}

impl Letter {
    pub(crate) fn new(
        x: i32,
        y: i32,
        ascii: u8,
        screen_position_x: i32,
        screen_position_y: f64,
        next_row: bool,
        index: i32,
    ) -> Letter {
        const WIDTH: i32 = 62;
        const HEIGHT: i32 = 66;
        let mut _x = 0;
        let mut _y = 0;
        if next_row {
            _x = x * WIDTH;
            _y = y * HEIGHT;
        } else {
            _x = x;
            _y = y;
        }

        Letter {
            ascii,
            screen_position_x,
            screen_position_y,
            x: _x,
            y: _y,
            count: 0,
        }
    }

    pub(crate) fn update(&mut self) {
        self.screen_position_x -= 1;
        self.count += 1 % 360;
        let radians = self.count as f64 * (PI / 180.0);
        let sinus = f64::cos(radians) * 57.5;
        self.screen_position_y = 200.0 + sinus;
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn draw_letters(x: i32, y: i32, screen_position_x: i32, screen_position_y: f64);
}