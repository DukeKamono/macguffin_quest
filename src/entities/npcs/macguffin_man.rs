use ggez::nalgebra as na;
use ggez::*;
use std::time::Duration;

use super::super::{CollideEntity, DrawableEntity};
use crate::ui::FloatingText;

pub struct MacguffinMan {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
    floating_text: Vec<FloatingText>,
    pub cooldown: Duration,
}

impl MacguffinMan {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> MacguffinMan {
        let img = graphics::Image::new(ctx, "/blob.png").unwrap();
        let hb = img.dimensions();
        let floating_text = Vec::new();

        MacguffinMan {
            x: xpos,
            y: ypos,
            hp: 10.0,
            atk: 3.0,
            def: 1.0,
            sprite: img,
            hitbox: hb,
            floating_text,
			cooldown: Duration::new(1u64, 0u32),
		}
    }

	pub fn update(&mut self, delta: Duration) {
        self.floating_text.retain(|t| t.live());
        self.floating_text.iter_mut().for_each(|t| t.update(delta));

        // cooldown
        if self.talk_cooldown() {
            self.cooldown += delta;
        }
    }
	
	pub fn talk(&mut self, ctx: &mut Context, text: String) {
		if !self.talk_cooldown() {
            self.cooldown = Duration::new(0u64, 0u32);
			self.floating_text.push(FloatingText::new(ctx, self.x, self.y, text));
		}
	}
	
    fn talk_cooldown(&self) -> bool {
        self.cooldown < Duration::from_millis(1000u64)
    }
}

impl DrawableEntity for MacguffinMan {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.sprite, dp)?;

        self.floating_text.iter().for_each(|t| t.draw(ctx));

        Ok(())
    }
}

impl CollideEntity for MacguffinMan {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.hitbox;
        r.x = self.x;
        r.y = self.y;
        r
    }
}