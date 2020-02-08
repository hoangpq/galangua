extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use super::collision::{CollBox, Collidable};

pub struct MyShot {
    x: i32,
    y: i32,
}

impl MyShot {
    pub fn new(x: i32, y: i32) -> MyShot {
        MyShot {
            x,
            y,
        }
    }

    pub fn update(&mut self) -> bool {
        self.y -= 8;

        self.y >= 0
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
        canvas.copy(&texture,
                    Some(Rect::new(16, 0, 8, 8)),
                    Some(Rect::new((self.x - 4) * 2, (self.y - 4) * 2, 8 * 2, 8 * 2)))?;

        Ok(())
    }
}

impl Collidable for MyShot {
    fn get_collbox(&self) -> CollBox {
        CollBox {
            left: self.x - 1,
            top: self.y - 4,
            width: 1,
            height: 8,
        }
    }
}
