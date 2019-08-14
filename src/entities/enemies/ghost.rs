use crate::entities::enemies::ai::*;
use crate::entities::enemies::enemiesstruct::Enemy;
use crate::entities::environment::level::Level;
use crate::entities::player::playerstruct::Player;
use crate::sprites::*;
use ggez::graphics::{Image, Rect};
use ggez::nalgebra as na;
use ggez::*;
use std::collections::HashMap;
use std::time::Duration;

use super::super::{Animations, CollideEntity, Direction, DrawableEntity};
use crate::entities::enemies::sight::*;
use crate::ui::FloatingText;

/// Everything associated with the ghost enemy
pub struct Ghost {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    floating_text: Vec<FloatingText>,
    pub invulnerable: Duration,
    pub line_of_sight: LineOfSight,
    pub ai_type: AITypes,
    pub sprite: HashMap<(Animations, Direction), AnimatedSprite>,
    pub animation: (Animations, Direction),
    pub direction: Direction,
}

/// The functions used by the Ghost struct
impl Ghost {
	/// Sets up a new Ghost struct and returns it.
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32, ai_type: AITypes) -> Ghost {
        let mut sprite = HashMap::new();
        let sheet = Image::new(ctx, "/ghost.png").unwrap();
        let builder = AnimatedBuilder::new(&sheet);

        // walking
        sprite.insert(
            (Animations::Walking, Direction::Up),
            builder
                .create_animated(Rect::new(0f32, 0f32, 64f32, 64f32), 8usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Walking, Direction::Left),
            builder
                .create_animated(Rect::new(0f32, 0f32, 64f32, 64f32), 8usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Walking, Direction::Down),
            builder
                .create_animated(Rect::new(0f32, 64f32, 64f32, 64f32), 8usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Walking, Direction::Right),
            builder
                .create_animated(Rect::new(0f32, 64f32, 64f32, 64f32), 8usize)
                .unwrap(),
        );

        let floating_text = Vec::new();

        Ghost {
            x: xpos,
            y: ypos,
            hp: 15.0,
            atk: 3.0,
            def: 0.0,
            floating_text,
            invulnerable: Duration::new(1u64, 0u32),
            line_of_sight: LineOfSight::new(xpos, ypos),
            ai_type,
            sprite,
            animation: (Animations::Walking, Direction::Down),
            direction: Direction::Down,
        }
    }

	/// Whenever an enemy needs to take damage, we check to see if it is invulnerable
	/// or not, then checks to see if the defence is high enough to reduce hp or not.
	/// If the enemy has 0 or less hp give the player experience.
    pub fn take_dmg(&mut self, ctx: &mut Context, player: &mut Player) {
        let true_dmg = player.stats.atk - self.def;
        if !self.invulnerable() {
            if true_dmg > 0.0 {
                self.hp -= true_dmg;
                self.invulnerable = Duration::new(0u64, 0u32);
                self.floating_text.push(FloatingText::new(
                    ctx,
                    self.x,
                    self.y,
                    true_dmg.to_string(),
                    "Red",
                ));
            } else {
                self.floating_text.push(FloatingText::new(
                    ctx,
                    self.x,
                    self.y,
                    "Blocked".to_string(),
                    "Blue",
                ));
            }
        }

        if self.hp <= 0.0 {
            player.stats.check_for_level_up(5);
        }
    }

    /// Returns if the enemy should be able to take damage (time is 1/4 sec)
    fn invulnerable(&self) -> bool {
        self.invulnerable < Duration::from_millis(250u64)
    }
}

/// Attempts to draw the enemy
impl DrawableEntity for Ghost {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, self.sprite.get(&self.animation).unwrap(), dp)?;

        self.floating_text.iter().for_each(|t| t.draw(ctx));

        Ok(())
    }
}

/// Determines where the bounds of the enemy is for collision
impl CollideEntity for Ghost {
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

/// Functions associated with the Enemy trait implemented for the Ghost struct
impl Enemy for Ghost {
	/// Every update check for floating text, if we can be hit again, and if I have touched the player's attack box.
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, _level: &Level) {
        self.floating_text.retain(|t| t.live());
        self.floating_text.iter_mut().for_each(|t| t.update(delta));

        // cool down invulnerable of Ghost
        if self.invulnerable() {
            self.invulnerable += delta;
        }

        // Player's atk_box hits me
        if let Some(atk) = &player.atk_box {
            if self.collision(atk) {
                self.take_dmg(ctx, player);
            }
        }
    }

	/// Checks to see if the enemy is still alive or not.
    fn islive(&self) -> bool {
        self.hp > 0.0
    }

	/// Returns what type of AI the enemy is set at.
    fn get_aitype(&mut self) -> &AITypes {
        &self.ai_type
    }

	/// Describes how this enemy will chase the player.
    fn chase_player(
        &mut self,
        ctx: &mut Context,
        delta: Duration,
        player: &mut Player,
        _level: &Level,
    ) {
        // Charge towards player.
        if self.x >= player.x {
            self.x -= 1.0;
        }
        if self.x <= player.x {
            self.x += 1.0;
        }

        if self.y >= player.y {
            self.y -= 1.0;
        }
        if self.y <= player.y {
            self.y += 1.0;
        }

        // Which way am I facing?
        if self.x > player.x && self.y > player.y {
            self.animation = (Animations::Walking, Direction::Left);
            self.direction = Direction::Left;
        } else if self.x < player.x && self.y < player.y {
            self.animation = (Animations::Walking, Direction::Right);
            self.direction = Direction::Right;
        } else if self.y > player.y && self.x < player.x {
            self.animation = (Animations::Walking, Direction::Up);
            self.direction = Direction::Up;
        } else {
            self.animation = (Animations::Walking, Direction::Down);
            self.direction = Direction::Down;
        }

        self.sprite.get_mut(&self.animation).unwrap().animate(delta);

        // I touched the player.
        if self.collision(player) {
            // need attack animation
            player.take_dmg(ctx, self.atk);
        }
    }

	/// If the player comes within sight of the enemy then chase the player.
    fn chase_player_sight(
        &mut self,
        ctx: &mut Context,
        delta: Duration,
        player: &mut Player,
        level: &Level,
    ) {
        self.line_of_sight
            .update(self.x - 100.0, self.y - 100.0, 200.0, 200.0);

        if self.line_of_sight.collision(player) {
            // && !self.line_of_sight.collision(level) {
            self.chase_player(ctx, delta, player, level);
        }
    }

	/// Determines with this enemy spawns enemies.
    fn spawn(&self) -> bool {
        false
    }
}
