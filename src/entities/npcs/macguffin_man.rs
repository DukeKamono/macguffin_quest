use crate::sprites::*;
use ggez::graphics::{Image, Rect};
use ggez::nalgebra as na;
use ggez::*;
use std::collections::HashMap;
use std::time::Duration;

use super::super::{Animations, CollideEntity, Direction, DrawableEntity};
use crate::ui::FloatingText;

/// The MacguffinMan struct
pub struct MacguffinMan {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    floating_text: Vec<FloatingText>,
    pub cooldown: Duration,
    pub sprite: HashMap<(Animations, Direction), AnimatedSprite>,
    pub animation: (Animations, Direction),
    pub direction: Direction,
}

/// Functions for the MacguffinMan
impl MacguffinMan {
    /// News up a MacguffinMan
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> MacguffinMan {
        let floating_text = Vec::new();

        let mut sprite = HashMap::new();
        let sheet = Image::new(ctx, "/macguffin-man.png").unwrap();
        let builder = AnimatedBuilder::new(&sheet);

        sprite.insert(
            (Animations::Stand, Direction::Down),
            builder
                .create_animated(Rect::new(0f32, 128f32, 64f32, 64f32), 1usize)
                .unwrap(),
        );

        MacguffinMan {
            x: xpos,
            y: ypos,
            hp: 10.0,
            atk: 3.0,
            def: 1.0,
            floating_text,
            cooldown: Duration::new(1u64, 0u32),
            sprite,
            animation: (Animations::Stand, Direction::Down),
            direction: Direction::Down,
        }
    }

    /// Keeps current floating text and resets the cooldown if needed for the text.
    pub fn update(&mut self, delta: Duration) {
        self.floating_text.retain(|t| t.live());
        self.floating_text.iter_mut().for_each(|t| t.update(delta));

        // cooldown
        if self.talk_cooldown() {
            self.cooldown += delta;
        }
    }

    /// Shows the floating text if the cooldown is done.
    pub fn talk(&mut self, ctx: &mut Context, text: String) {
        if !self.talk_cooldown() {
            self.cooldown = Duration::new(0u64, 0u32);
            self.floating_text
                .push(FloatingText::new(ctx, self.x, self.y, text, "White"));
        }
    }

    /// Check to see if the cooldown if less than 1000 milliseconds
    fn talk_cooldown(&self) -> bool {
        self.cooldown < Duration::from_millis(1000u64)
    }
}

/// The draw trait for the MacguffinMan
impl DrawableEntity for MacguffinMan {
    /// Draws the MacguffinMan and the floating text
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, self.sprite.get(&self.animation).unwrap(), dp)?;

        self.floating_text.iter().for_each(|t| t.draw(ctx));

        Ok(())
    }
}

/// Colldie trait for the MacguffinMan
impl CollideEntity for MacguffinMan {
    /// Find the hitbox for the MacguffinMan to check for collision.
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
