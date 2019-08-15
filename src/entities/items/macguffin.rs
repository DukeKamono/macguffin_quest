use crate::sprites::*;
use ggez::graphics::{Image, Rect};
use ggez::nalgebra as na;
use ggez::*;
use rand::prelude::*;
use std::collections::HashMap;

use super::super::{Animations, CollideEntity, Direction, DrawableEntity};

/// Struct for the Macguffin
pub struct Macguffin {
    pub x: f32,
    pub y: f32,
    pub sprite: HashMap<(Animations, Direction), AnimatedSprite>,
    pub animation: (Animations, Direction),
    pub direction: Direction,
}

/// Functions implemented for the Macguffin struct
impl Macguffin {
    /// News up a new Macguffin. The sprite will be a random locaiton on the items spritesheet
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> Macguffin {
        let mut sprite = HashMap::new();
        let sheet = Image::new(ctx, "/items.png").unwrap();
        let builder = AnimatedBuilder::new(&sheet);

        let mut rng = thread_rng();
        let random_x_item = rng.gen_range(0, 5) as f32 * 64.0;
        let random_y_item = rng.gen_range(0, 4) as f32 * 64.0;

        sprite.insert(
            (Animations::Stand, Direction::Down),
            builder
                .create_animated(
                    Rect::new(random_x_item, random_y_item, 64f32, 64f32),
                    1usize,
                )
                .unwrap(),
        );

        Macguffin {
            x: xpos,
            y: ypos,
            sprite,
            animation: (Animations::Stand, Direction::Down),
            direction: Direction::Down,
        }
    }
}

/// The draw trait for the Macguffin
impl DrawableEntity for Macguffin {
    /// Draws up the Macguffin
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, self.sprite.get(&self.animation).unwrap(), dp)?;

        Ok(())
    }
}

/// The collide trait for the Macguffin
impl CollideEntity for Macguffin {
    /// Function to determine the hitbox to collide with the Macguffin
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self
            .sprite
            .get(&self.animation)
            .unwrap()
            .dimensions()
            .unwrap();
        r.x = self.x;
        r.y = self.y;
        r
    }
}
