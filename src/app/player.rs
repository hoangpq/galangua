extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use super::super::util::pad::{Pad, PAD_L, PAD_R, PAD_A};

pub struct Player {
    x: i32,
    y: i32,
    mx: i32,
    my: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 240 / 2,
            y: 280,
            mx: 0,
            my: -1,
        }
    }

    pub fn update(&mut self, pad: &Pad) {
        if pad.is_pressed(PAD_L) {
            self.x -= 2;
            if self.x < 8 {
                self.x = 8;
            }
        }
        if pad.is_pressed(PAD_R) {
            self.x += 2;
            if self.x > 240 - 8 {
                self.x = 240 - 8;
            }
        }
        if pad.is_pressed(PAD_A) && self.my < 0 {
            self.mx = self.x;
            self.my = self.y;
        }
        if self.my >= 0 {
            self.my -= 8;
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
        canvas.copy(&texture, None, Some(Rect::new((self.x - 8) * 2, (self.y - 8) * 2, 16 * 2, 16 * 2)))?;
        if self.my >= 0 {
            canvas.copy(&texture, None, Some(Rect::new((self.mx - 1) * 2, (self.my - 3) * 2, 2 * 2, 6 * 2)))?;
        }

        Ok(())
    }
}