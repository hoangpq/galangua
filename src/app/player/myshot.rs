use crate::app::consts::{WIDTH, HEIGHT};
use crate::app::util::{CollBox, Collidable};
use crate::framework::types::Vec2I;
use crate::framework::RendererTrait;
use crate::util::math::{calc_velocity, round_up, ANGLE, ONE};

pub struct MyShot {
    pos: Vec2I,
    dual: bool,
    angle: i32,
}

impl MyShot {
    pub fn new(pos: &Vec2I, dual: bool, angle: i32) -> Self {
        Self {
            pos: *pos,
            dual,
            angle,
        }
    }

    pub fn update(&mut self) -> bool {
        let speed = 8 * ONE;
        let margin = 4;
        let top = -margin * ONE;
        if self.angle == 0 {
            self.pos.y -= speed;
            self.pos.y > top
        } else {
            let left = -margin * ONE;
            let right = (WIDTH + margin) * ONE;
            let bottom = (HEIGHT + margin) * ONE;
            self.pos += calc_velocity(self.angle, speed);
            self.pos.y > top && self.pos.x > left && self.pos.x < right && self.pos.y < bottom
        }
    }

    pub fn draw<R>(&self, renderer: &mut R) -> Result<(), String>
        where R: RendererTrait
    {
        let pos = self.pos();
        if self.angle == 0 {
            renderer.draw_sprite("myshot", &(&pos + &Vec2I::new(-2, -4)))?;
            if self.dual {
                renderer.draw_sprite("myshot", &(&pos + &Vec2I::new(-2 + 16, -4)))?;
            }
        } else {
            assert!(!self.dual);
            renderer.draw_sprite_rot("myshot", &(&pos + &Vec2I::new(-2, -4)), calc_display_angle(self.angle), None)?;
        }

        Ok(())
    }

    pub fn get_collbox_for_dual(&self) -> Option<CollBox> {
        if self.dual {
            Some(CollBox {
                top_left: &self.pos() + &Vec2I::new(-1 + 16, -4),
                size: Vec2I::new(1, 8),
            })
        } else {
            None
        }
    }

    fn pos(&self) -> Vec2I {
        round_up(&self.pos)
    }
}

impl Collidable for MyShot {
    fn get_collbox(&self) -> Option<CollBox> {
        Some(CollBox {
            top_left: &self.pos() - &Vec2I::new(1, 4),
            size: Vec2I::new(1, 8),
        })
    }
}

fn calc_display_angle(angle: i32) -> f64 {
    // Quantize.
    let div = 16;
    let angle = (angle + ANGLE * ONE / div / 2) & (ANGLE * ONE - (ANGLE * ONE / div));
    let angle: f64 = (angle as f64) * (360.0 / ((ANGLE * ONE) as f64));

    angle
}
