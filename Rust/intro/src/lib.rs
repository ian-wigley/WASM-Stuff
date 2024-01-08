mod stars;
mod letter;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use crate::letter::Letter;
use crate::stars::Star;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref MAIN: Mutex<Main> = {
        let main = Main {
            stars: Vec::new(),
            letters: Vec::new(),
            scrolling_text: Vec::new(),
            width: 0.0,
            height: 0.0,
            count: 0
        };
        Mutex::new(main)
    };
}

pub struct Main {
    stars: Vec<Star>,
    letters: Vec<Letter>,
    scrolling_text: Vec<Letter>,
    width: f64,
    height: f64,
    count: usize
}

impl Main {
    fn create(&mut self, num: i32) {
        for _ in 0..num {
            self.stars
                .push(Star::new(self.width, self.height, String::from("white")));
        }
        self.create_letters_from_bitmap();
        self.populate_scroll();
    }

    fn create_letters_from_bitmap(&mut self) {
        let mut start_ascii_code: u8 = 97;
        let mut screen_position_x = 100;
        let screen_position_y = 0.0;
        let index = 0;
        for i in 0..7 {
            for j in 0..4 {
                let ascii = start_ascii_code;
                let next_row = true;
                self.letters
                    .push(Letter::new(j, i, ascii, screen_position_x, screen_position_y, next_row, index));
                screen_position_x += 30;
                start_ascii_code += 1;
            }
        }
    }

    fn populate_scroll(&mut self) {
        let mut screen_position_x = 800;
        let screen_position_y = 200.0;
        let mut index = 0;
        let scroll_text = "{{{{welcome|to|a|funky|scroller{{{{{{{{{";
        for i in 0..scroll_text.len() {
            for j in 0..self.letters.len() {
                if scroll_text.as_bytes()[i].eq_ignore_ascii_case(&self.letters[j].ascii) {
                    let trip = false;
                    if index < 14 * 10 {
                        index += 10;
                    } else {
                        index = 0;
                    }
                    self.scrolling_text.push(Letter::new(self.letters[j].x, self.letters[j].y, self.letters[j].ascii, screen_position_x, screen_position_y, trip, index));
                    screen_position_x += 30;
                    self.count+=1;
                }
            }
        }
    }

    fn update(&mut self) {
        let rng = rand::thread_rng();
        for star in self.stars.iter_mut() {
            star.update(rng.clone());
        }
        let last_letters: Letter = self.scrolling_text.last().unwrap().clone();
        let mut spacing = 30;
        for letter in self.scrolling_text.iter_mut(){
            if last_letters.screen_position_x < 0 {
                letter.screen_position_x = self.width as i32 + spacing;
                spacing += 30;
            }
            letter.update();
        }
    }

    fn draw(&mut self) {
        for star in self.stars.iter() {
            stars::draw_stars(star.x, star.y, &star.colour, 1);
        }
        for text in self.scrolling_text.iter_mut(){
            letter::draw_letters(text.x, text.y, text.screen_position_x, text.screen_position_y);
        }
    }
}

#[wasm_bindgen]
pub fn create_objects(w: f64, h: f64, num: i32) {
    MAIN.lock().unwrap().width = w;
    MAIN.lock().unwrap().height = h;
    MAIN.lock().unwrap().create(num);
}

#[wasm_bindgen]
pub fn render_objects() {
    MAIN.lock().unwrap().update();
    MAIN.lock().unwrap().draw();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        MAIN.lock().unwrap().create_letters_from_bitmap();
        let result = 1;
        MAIN.lock().unwrap().populate_scroll();
        assert_eq!(result, 1);
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }

    #[test]
    fn it_stinks() {
        MAIN.lock().unwrap().create_letters_from_bitmap();
        let result = 1;
        MAIN.lock().unwrap().populate_scroll();
        MAIN.lock().unwrap().update();
        assert_eq!(result, 1);
        let _a = 3;

        MAIN.lock().unwrap().update();
        let _b = 3;
        assert_eq!(result, 1);
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }

}