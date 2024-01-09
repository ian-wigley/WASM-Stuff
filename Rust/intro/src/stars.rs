use rand::Rng;
use rand::rngs::ThreadRng;
use wasm_bindgen::prelude::wasm_bindgen;

pub struct Star {
    pub(crate) speed: f64,
    pub(crate) angle: f64,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) colour: String,
    width: f64,
    height: f64,
}

impl Star {
    pub(crate) fn new(x: f64, y: f64, colour: String) -> Star {
        let mut rng = rand::thread_rng();
        Star {
            speed: 0.0,
            angle: 360.0 * rng.gen::<f64>() - 2.0,
            x,
            y,
            colour,
            width: x,
            height: y,
        }
    }

    pub(crate) fn update(&mut self, mut rng: ThreadRng) {
        self.speed += 0.1;
        self.angle += 0.025;
        self.x = f64::cos(self.angle) * self.speed + self.x;
        self.y = f64::sin(self.angle) * self.speed + self.y;
        if self.x < 0.0 || self.x > self.width || self.y < 0.0 || self.y > self.height {
            self.angle = 360.0 * rng.gen::<f64>();
            self.speed = 0.01;
            self.x = self.width / 2.0;
            self.y = self.height / 2.0;
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn draw_stars(x: f64, y: f64, s: &str, size: i32);
}