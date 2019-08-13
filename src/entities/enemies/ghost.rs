use crate::entities::player::player::Player;
use crate::entities::environment::level::Level;
use crate::entities::enemies::enemies::Enemy;
use crate::entities::enemies::ai::*;
use ggez::nalgebra as na;
use ggez::*;
use std::time::Duration;
//use rand::prelude::*;

use super::super::{CollideEntity, DrawableEntity};
use crate::ui::FloatingText;
use crate::entities::enemies::sight::*;

pub struct Ghost {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
    floating_text: Vec<FloatingText>,
    pub invulnerable: Duration,
    pub line_of_sight: LineOfSight,
    pub ai_type: AITypes,
}

impl Ghost {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32, ai_type: AITypes) -> Ghost {
        let img = graphics::Image::new(ctx, "/pong_spritesheet.png").unwrap();
        let hb = img.dimensions();
        let floating_text = Vec::new();

        Ghost {
            x: xpos,
            y: ypos,
            hp: 15.0,
            atk: 3.0,
            def: 0.0,
            sprite: img,
            hitbox: hb,
            floating_text,
            invulnerable: Duration::new(1u64, 0u32),
            line_of_sight: LineOfSight::new(xpos, ypos),
            ai_type,
        }
    }

    pub fn take_dmg(&mut self, ctx: &mut Context, player: &mut Player) {
        let true_dmg = player.stats.atk - self.def;
		if !self.invulnerable() {
			if true_dmg > 0.0 {
				self.hp -= true_dmg;
				self.invulnerable = Duration::new(0u64, 0u32);
				self.floating_text.push(FloatingText::new(ctx, self.x, self.y, true_dmg.to_string()));
				// Check for death and maybe call a death function.
			}
			else {
				self.floating_text.push(FloatingText::new(ctx, self.x, self.y, "Blocked".to_string()));
			}
		}
        
        if self.hp <= 0.0 {
            player.stats.check_for_level_up(5);
        }
    }

    // returns if Ghost should be able to take damage (time is 1/4 sec)
    fn invulnerable(&self) -> bool {
        self.invulnerable < Duration::from_millis(250u64)
    }
}

impl DrawableEntity for Ghost {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.sprite, dp)?;

        self.floating_text.iter().for_each(|t| t.draw(ctx));

        Ok(())
    }
}

impl CollideEntity for Ghost {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.hitbox;
        r.x = self.x;
        r.y = self.y;
        r
    }
}

impl Enemy for Ghost {
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

    fn islive(&self) -> bool {
        self.hp > 0.0
    }
    
    fn get_aitype(&mut self) -> &AITypes {
        &self.ai_type
    }
    
    fn chase_player(&mut self, ctx: &mut Context, _delta: Duration, player: &mut Player, _level: &Level) {
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
        
        // I touched the player.
        if self.collision(player) {
            // need attack animation
            player.take_dmg(ctx, self.atk);
        }
    }
    
    fn chase_player_sight(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, level: &Level) {
        self.line_of_sight.update(self.x - 100.0, self.y - 100.0, 200.0, 200.0);
        
        if self.line_of_sight.collision(player) {// && !self.line_of_sight.collision(level) {
            self.chase_player(ctx, delta, player, level);
        }
    }
	
	fn spawn(&self) -> bool {
		false
	}
}