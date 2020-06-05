use crate::app::consts::*;
use crate::app::effect::RecapturedFighter;
use crate::app::game::{EventQueue, EventType};
use crate::app::util::{CollBox, Collidable};
use crate::framework::types::Vec2I;
use crate::framework::RendererTrait;
use crate::util::math::{clamp, round_up, ANGLE, ONE};
use crate::util::pad::{Pad, PAD_A, PAD_L, PAD_R};

const DEFAULT_LEFT_SHIP: u32 = 3;

#[derive(PartialEq)]
enum State {
    Normal,
    Dead,
    Capturing,
    Captured,
    CaptureCompleted,
    MoveHomePos,
}

pub struct Player {
    pos: Vec2I,
    state: State,
    dual: bool,
    angle: i32,
    capture_pos: Vec2I,
    recaptured_fighter: Option<RecapturedFighter>,
    left_ship: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: &Vec2I::new(WIDTH / 2, HEIGHT - 16 - 8) * ONE,
            state: State::Normal,
            dual: false,
            angle: 0,
            capture_pos: Vec2I::new(0, 0),
            recaptured_fighter: None,
            left_ship: DEFAULT_LEFT_SHIP,
        }
    }

    pub fn restart(&mut self) {
        self.state = State::Normal;
        self.pos = &Vec2I::new(WIDTH / 2, HEIGHT - 16 - 8) * ONE;
    }

    pub fn update(&mut self, pad: &Pad, event_queue: &mut EventQueue) {
        match self.state {
            State::Normal => {
                self.update_normal(pad, event_queue);
            }
            State::Dead => {
            }
            State::Capturing => {
                self.update_capture(pad, event_queue);
            }
            State::Captured | State::CaptureCompleted => {}
            State::MoveHomePos => {
                let x = (WIDTH / 2 - 8) * ONE;
                let speed = 1 * ONE;
                self.pos.x += clamp(x - self.pos.x, -speed, speed);
                if self.pos.x == x {
                    if self.recaptured_fighter.as_ref().unwrap().done() {
                        self.dual = true;
                        self.state = State::Normal;
                        self.recaptured_fighter = None;
                        event_queue.push(EventType::RecaptureEnded);
                    }
                }
            }
        }

        if let Some(recaptured_fighter) = &mut self.recaptured_fighter {
            recaptured_fighter.update(self.state != State::Dead, event_queue);
            if self.state == State::Dead && recaptured_fighter.done() {
                self.pos.x = WIDTH / 2 * ONE;
                self.state = State::Normal;
                self.recaptured_fighter = None;
                event_queue.push(EventType::RecaptureEnded);
            }
        }
    }

    pub fn update_normal(&mut self, pad: &Pad, event_queue: &mut EventQueue) {
        if pad.is_pressed(PAD_L) {
            self.pos.x -= 2 * ONE;
            if self.pos.x < 8 * ONE {
                self.pos.x = 8 * ONE;
            }
        }
        if pad.is_pressed(PAD_R) {
            self.pos.x += 2 * ONE;

            let right = if self.dual { (WIDTH - 8 - 16) * ONE } else { (WIDTH - 8) * ONE };
            if self.pos.x > right {
                self.pos.x = right;
            }
        }
        self.fire_bullet(pad, event_queue);
    }

    pub fn update_capture(&mut self, pad: &Pad, event_queue: &mut EventQueue) {
        const D: i32 = 1 * ONE;
        let d = &self.capture_pos - &self.pos;
        self.pos.x += clamp(d.x, -D, D);
        self.pos.y += clamp(d.y, -D, D);
        self.angle += ANGLE * ONE / 32;

        self.fire_bullet(pad, event_queue);

        if d.x == 0 && d.y == 0 {
            self.state = State::Captured;
            self.angle = 0;
        }
    }

    fn fire_bullet(&mut self, pad: &Pad, event_queue: &mut EventQueue) {
        if pad.is_trigger(PAD_A) {
            let pos = &self.pos + &Vec2I::new(0, 2 * ONE);
            event_queue.push(EventType::MyShot(pos, self.dual, self.angle));
        }
    }

    pub fn draw<R>(&self, renderer: &mut R) -> Result<(), String>
        where R: RendererTrait
    {
        match self.state {
            State::Normal | State::MoveHomePos => {
                let pos = self.pos();
                renderer.draw_sprite("rustacean", &(&pos + &Vec2I::new(-8, -8)))?;
                if self.dual {
                    renderer.draw_sprite("rustacean", &(&pos + &Vec2I::new(-8 + 16, -8)))?;
                }
            }
            State::Capturing => {
                let pos = self.pos();
                let angle = calc_display_angle(self.angle);
                //renderer.draw_sprite_rot("rustacean", &(&pos + &Vec2I::new(-8, -8)), angle, Some(Vec2I::new(7, 10)))?;
                renderer.draw_sprite_rot("rustacean", &(&pos + &Vec2I::new(-8, -8)), angle, None)?;
            }
            State::Captured => {
                let pos = self.pos();
                renderer.draw_sprite("rustacean_captured", &(&pos + &Vec2I::new(-8, -8)))?;
            }
            State::CaptureCompleted | State::Dead => {}
        }

        if let Some(recaptured_fighter) = &self.recaptured_fighter {
            recaptured_fighter.draw(renderer)?;
        }

        if self.left_ship > 0 {
            for i in 0..self.left_ship - 1 {
                renderer.draw_sprite("rustacean", &Vec2I::new(i as i32 * 16, HEIGHT - 16))?;
            }
        }

        Ok(())
    }

    pub fn active(&self) -> bool {
        self.state == State::Normal
    }

    pub fn pos(&self) -> Vec2I {
        round_up(&self.pos)
    }

    pub fn get_raw_pos(&self) -> &Vec2I {
        &self.pos
    }

    pub fn dual_pos(&self) -> Option<Vec2I> {
        if self.dual {
            Some(&self.pos() + &Vec2I::new(16, 0))
        } else {
            None
        }
    }

    pub fn dual_collbox(&self) -> Option<CollBox> {
        if self.dual && self.state == State::Normal {
            Some(CollBox {
                top_left: &self.pos() + &Vec2I::new(8, -8),
                size: Vec2I::new(16, 16),
            })
        } else {
            None
        }
    }

    pub fn crash(&mut self, pos: &Vec2I) -> bool {
        if self.dual {
            if pos.x < self.pos.x + (16 / 2 * ONE) {
                // Abandan left ship.
                self.pos.x += 16 * ONE;
            }
            self.dual = false;
            false
        } else {
            self.state = State::Dead;
            true
        }
    }

    pub fn decrement_and_restart(&mut self) -> bool {
        self.left_ship -= 1;
        if self.left_ship == 0 {
            false
        } else {
            self.restart();
            true
        }
    }

    pub fn start_capture(&mut self, capture_pos: &Vec2I) {
        self.state = State::Capturing;
        self.capture_pos = *capture_pos;
        self.angle = 0;
    }

    pub fn complete_capture(&mut self) {
        self.state = State::CaptureCompleted;
    }

    pub fn start_recapture_effect(&mut self, pos: &Vec2I) {
        self.recaptured_fighter = Some(RecapturedFighter::new(pos));
    }

    pub fn start_move_home_pos(&mut self) {
        if self.state != State::Dead {
            self.state = State::MoveHomePos;
        }
    }
}

impl Collidable for Player {
    fn get_collbox(&self) -> Option<CollBox> {
        if self.state == State::Normal {
            Some(CollBox {
                top_left: &self.pos() - &Vec2I::new(8, 8),
                size: Vec2I::new(16, 16),
            })
        } else {
            None
        }
    }
}

fn calc_display_angle(angle: i32) -> f64 {
    // Quantize.
    let div = 16;
    let angle = (angle + ANGLE * ONE / div / 2) & (ANGLE * ONE - (ANGLE * ONE / div));
    let angle: f64 = (angle as f64) * (360.0 / ((ANGLE * ONE) as f64));

    angle
}
