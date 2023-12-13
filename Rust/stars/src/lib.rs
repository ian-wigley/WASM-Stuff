use rand::Rng;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref STARS: Mutex<Main> = {
        let p = Main {
            stars: Vec::new(),
            width: 0.0,
            height: 0.0,
        };
        Mutex::new(p)
    };
}

pub struct Star {
    speed: f64,
    angle: f64,
    x: f64,
    y: f64,
    colour: String,
}

impl Star {
    fn new(x: f64, y: f64, colour: String) -> Star {
        let mut rng = rand::thread_rng();
        Star {
            speed: 0.0,
            angle: 360.0 * rng.gen::<f64>() - 2.0,
            x: x,
            y: y,
            colour,
        }
    }
}

pub struct Main {
    stars: Vec<Star>,
    width: f64,
    height: f64,
}

impl Main {

    fn create(&mut self, num: i32) {
        for _ in 0..num {
            self.stars
                .push(Star::new(400.0, 400.0, String::from("grey")));
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        for p in self.stars.iter_mut() {
            p.speed += 0.01;
            p.angle -= 0.01;

            p.x = f64::cos(p.angle) * p.speed + p.x;
            p.y = f64::sin(p.angle) * p.speed + p.y;
            if p.x < 0.0 || p.x > 800.0 || p.y < 0.0 || p.y > 800.0 {
                p.angle = 360.0 * rng.gen::<f64>();
                p.speed = 0.01;
                p.x = 400.0;
                p.y = 400.0;
            }
        }
    }

    fn draw(&self) {
        for p in self.stars.iter() {
            draw_stuff(p.x, p.y, &p.colour, 1);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    fn draw_stuff(x: f64, y: f64, s: &str, size: i32);
}

#[wasm_bindgen]
pub fn create_stars(w: f64, h: f64, num: i32) {
    STARS.lock().unwrap().width = w;
    STARS.lock().unwrap().height = h;
    STARS.lock().unwrap().create(num);
}

#[wasm_bindgen]
pub fn render_stars() {
    STARS.lock().unwrap().update();
    STARS.lock().unwrap().draw();
}
