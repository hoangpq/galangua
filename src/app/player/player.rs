extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use super::super::consts::*;
use super::super::game::EventQueue;
use super::super::util::{CollBox, Collidable};
use super::super::super::util::pad::{Pad, PAD_L, PAD_R, PAD_A};
use super::super::super::util::types::Vec2I;

#[derive(PartialEq)]
enum State {
    Normal,
    Dual,
    Dead,
}

pub struct Player {
    pos: Vec2I,
    state: State,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vec2I::new((WIDTH / 2) * ONE, (HEIGHT - 16 - 8) * ONE),
            state: State::Dual,  // State::Normal,
        }
    }

    pub fn update(&mut self, pad: &Pad, event_queue: &mut EventQueue) {
        match self.state {
            State::Normal | State::Dual => {
                // Through.
            },
            State::Dead => {
                return;
            },
        }

        if pad.is_pressed(PAD_L) {
            self.pos.x -= 2 * ONE;
            if self.pos.x < 8 * ONE {
                self.pos.x = 8 * ONE;
            }
        }
        if pad.is_pressed(PAD_R) {
            self.pos.x += 2 * ONE;

            let right = if self.dual() { (WIDTH - 8 - 16) * ONE } else { (WIDTH - 8) * ONE };
            if self.pos.x > right {
                self.pos.x = right;
            }
        }
        if pad.is_trigger(PAD_A) {
            event_queue.spawn_myshot(Vec2I::new(self.pos.x, self.pos.y - 8 * ONE), self.dual());
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
        match self.state {
            State::Normal | State::Dual => {
                // Through.
            },
            State::Dead => {
                return Ok(());
            },
        }

        let pos = self.pos();
        canvas.copy(&texture,
                    Some(Rect::new(0, 0, 16, 16)),
                    Some(Rect::new((pos.x - 8) * 2, (pos.y - 8) * 2, 16 * 2, 16 * 2)))?;
        if self.dual() {
            canvas.copy(&texture,
                        Some(Rect::new(0, 0, 16, 16)),
                        Some(Rect::new((pos.x + 8) * 2, (pos.y - 8) * 2, 16 * 2, 16 * 2)))?;
        }

        Ok(())
    }

    fn dual(&self) -> bool {
        self.state == State::Dual
    }

    pub fn dead(&self) -> bool {
        self.state == State::Dead
    }

    pub fn pos(&self) -> Vec2I {
        Vec2I::new((self.pos.x + ONE / 2) / ONE, (self.pos.y + ONE / 2) / ONE)
    }

    pub fn dual_pos(&self) -> Option<Vec2I> {
        if self.dual() {
            let pos = self.pos();
            Some(Vec2I::new(pos.x + 16, pos.y))
        } else {
            None
        }
    }

    pub fn dual_collbox(&self) -> Option<CollBox> {
        if self.dual() {
            let pos = self.pos();
            Some(CollBox {
                top_left: Vec2I::new(pos.x + 8, pos.y - 8),
                size: Vec2I::new(16, 16),
            })
        } else {
            None
        }
    }

    pub fn crash(&mut self, pos: &Vec2I) -> bool {
        if self.dual() {
            if pos.x < self.pos.x + 16 / 2 {
                // Abandan left ship.
                self.pos.x += 16;
            }
            self.state = State::Normal;
            false
        } else {
            self.state = State::Dead;
            true
        }
    }
}

impl Collidable for Player {
    fn get_collbox(&self) -> CollBox {
        let pos = self.pos();
        CollBox {
            top_left: Vec2I::new(pos.x - 8, pos.y - 8),
            size: Vec2I::new(16, 16),
        }
    }
}
