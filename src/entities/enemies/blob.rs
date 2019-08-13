use crate::entities::enemies::ai::*;
use crate::entities::enemies::enemiesstruct::Enemy;
use crate::entities::environment::level::Level;
use crate::entities::player::playerstruct::Player;
use ggez::nalgebra as na;
use ggez::*;
use ggez::graphics::{Image, Rect};
use std::time::Duration;
use crate::sprites::*;
use std::collections::HashMap;

use super::super::{CollideEntity, DrawableEntity, Direction, Animations};
use crate::entities::enemies::sight::*;
use crate::ui::FloatingText;

pub struct Blob {
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

impl Blob {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32, ai_type: AITypes) -> Blob {
		let mut sprite = HashMap::new();
        let sheet = Image::new(ctx, "/gel.png").unwrap();
        let builder = AnimatedBuilder::new(&sheet);
		
		// standing
        sprite.insert(
            (Animations::Stand, Direction::Up),
            builder.create_animated(Rect::new(0f32, 0f32, 64f32, 64f32), 1usize).unwrap()
        );
        sprite.insert(
            (Animations::Stand, Direction::Left),
            builder.create_animated(Rect::new(0f32, 64f32, 64f32, 64f32), 1usize).unwrap()
        );
        sprite.insert(
            (Animations::Stand, Direction::Down),
            builder.create_animated(Rect::new(0f32, 128f32, 64f32, 64f32), 1usize).unwrap()
        );
        sprite.insert(
            (Animations::Stand, Direction::Right),
            builder.create_animated(Rect::new(0f32, 192f32, 64f32, 64f32), 1usize).unwrap()
        );
		// walking
		sprite.insert(
            (Animations::Walking, Direction::Up),
            builder.create_animated(Rect::new(64f32, 0f32, 64f32, 64f32), 8usize).unwrap()
        );
        sprite.insert(
            (Animations::Walking, Direction::Left),
            builder.create_animated(Rect::new(64f32, 64f32, 64f32, 64f32), 8usize).unwrap()
        );
        sprite.insert(
            (Animations::Walking, Direction::Down),
            builder.create_animated(Rect::new(64f32, 128f32, 64f32, 64f32), 8usize).unwrap()
        );
        sprite.insert(
            (Animations::Walking, Direction::Right),
            builder.create_animated(Rect::new(64f32, 192f32, 64f32, 64f32), 8usize).unwrap()
        );
		
        let floating_text = Vec::new();

        Blob {
            x: xpos,
            y: ypos,
            hp: 10.0,
            atk: 3.0,
            def: 1.0,
            floating_text,
            invulnerable: Duration::new(1u64, 0u32),
            line_of_sight: LineOfSight::new(xpos, ypos),
            ai_type,
			sprite,
			animation: (Animations::Walking, Direction::Down),
			direction: Direction::Down,
        }
    }

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
            // Check for death and maybe call a death function.
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

    // returns if blob should be able to take damage (time is 1/4 sec)
    fn invulnerable(&self) -> bool {
        self.invulnerable < Duration::from_millis(250u64)
    }
}

impl DrawableEntity for Blob {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, self.sprite.get(&self.animation).unwrap(), dp)?;

        self.floating_text.iter().for_each(|t| t.draw(ctx));

        Ok(())
    }
}

impl CollideEntity for Blob {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.sprite.get(&self.animation).unwrap().dimensions().unwrap();
        r.x = self.x;
        r.y = self.y;
        r
    }
}

impl Enemy for Blob {
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, _level: &Level) {
        self.floating_text.retain(|t| t.live());
        self.floating_text.iter_mut().for_each(|t| t.update(delta));

        // cool down invulnerable of blob
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

    fn islive(&self) -> bool {
        self.hp > 0.0
    }

    fn get_aitype(&mut self) -> &AITypes {
        &self.ai_type
    }

    fn chase_player(
        &mut self,
        ctx: &mut Context,
        delta: Duration,
        player: &mut Player,
        level: &Level,
    ) {
        // holding onto previous location
        let xpos = self.x;
        let ypos = self.y;

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
		}
		else if self.x < player.x && self.y < player.y {
			self.animation = (Animations::Walking, Direction::Right);
            self.direction = Direction::Right;
		}
		else if self.y > player.y && self.x < player.x {
			self.animation = (Animations::Walking, Direction::Up);
            self.direction = Direction::Up;
		}
		else {
			self.animation = (Animations::Walking, Direction::Down);
            self.direction = Direction::Down;
		}
		
		self.sprite.get_mut(&self.animation).unwrap().animate(delta);
        
        // Check wall collision
        if self.collision(level) {
            self.x = xpos;
            self.y = ypos;
        }

        // I touched the player.
        if self.collision(player) {
            // need attack animation
            player.take_dmg(ctx, self.atk);
            self.x = xpos;
            self.y = ypos;
        }
    }

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
		else {
            self.animation.0 = Animations::Stand;
        }
    }

    fn spawn(&self) -> bool {
        false
    }
}
