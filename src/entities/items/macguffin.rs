use ggez::nalgebra as na;
use ggez::*;
use ggez::graphics::{Image, Rect};
use crate::sprites::*;
use std::collections::HashMap;
use rand::prelude::*;

use super::super::{CollideEntity, DrawableEntity, Direction, Animations};

pub struct Macguffin {
    pub x: f32,
    pub y: f32,
	pub sprite: HashMap<(Animations, Direction), AnimatedSprite>,
	pub animation: (Animations, Direction),
	pub direction: Direction,
}

impl Macguffin {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> Macguffin {
        let mut sprite = HashMap::new();
        let sheet = Image::new(ctx, "/items.png").unwrap();
        let builder = AnimatedBuilder::new(&sheet);
		
		let mut rng = thread_rng();
		let random_x_item = rng.gen_range(0, 5) as f32 * 64.0;
		let random_y_item = rng.gen_range(0, 4) as f32 * 64.0;
		
		sprite.insert(
            (Animations::Stand, Direction::Down),
            builder.create_animated(Rect::new(random_x_item, random_y_item, 64f32, 64f32), 1usize).unwrap()
        );

        Macguffin {
            x: xpos,
            y: ypos,
			sprite: sprite,
			animation: (Animations::Stand, Direction::Down),
			direction: Direction::Down,
		}
    }
}

impl DrawableEntity for Macguffin {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, self.sprite.get(&self.animation).unwrap(), dp)?;

        Ok(())
	}
}

impl CollideEntity for Macguffin {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.sprite.get(&self.animation).unwrap().dimensions().unwrap();
        r.x = self.x;
        r.y = self.y;
        r
    }
}