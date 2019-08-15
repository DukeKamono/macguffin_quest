use crate::sprites::*;
use ggez::graphics::{Image, Rect};
use ggez::nalgebra as na;
use ggez::*;
use std::collections::HashMap;

use super::super::{Animations, CollideEntity, Direction, DrawableEntity};

/// The Potions struct
pub struct Potions {
    pub x: f32,
    pub y: f32,
    pub used: bool,
    pub sprite: HashMap<(Animations, Direction), AnimatedSprite>,
    pub animation: (Animations, Direction),
    pub direction: Direction,
}

/// Functions for the Potions struct
impl Potions {
    /// News up a Potion
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> Potions {
        let mut sprite = HashMap::new();
        let sheet = Image::new(ctx, "/items.png").unwrap();
        let builder = AnimatedBuilder::new(&sheet);

        sprite.insert(
            (Animations::Stand, Direction::Down),
            builder
                .create_animated(Rect::new(64f32, 128f32, 64f32, 64f32), 1usize)
                .unwrap(),
        );

        Potions {
            x: xpos,
            y: ypos,
            used: false,
            sprite,
            animation: (Animations::Stand, Direction::Down),
            direction: Direction::Down,
        }
    }
}

/// The draw trait for the Potions
impl DrawableEntity for Potions {
    /// Draws the Potion
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, self.sprite.get(&self.animation).unwrap(), dp)?;

        Ok(())
    }
}

/// Collide trait for the Potions
impl CollideEntity for Potions {
    /// Finds where the hitbox is for the Potion
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
