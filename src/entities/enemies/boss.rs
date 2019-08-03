use crate::entities::player::player::Player;
use crate::entities::environment::level::Level;
use crate::entities::enemies::enemies::Enemy;
use crate::entities::enemies::ai::*;
use ggez::nalgebra as na;
use ggez::*;
use std::time::Duration;
//use rand::prelude::*;

use super::super::{CollideEntity, DrawableEntity};
use crate::ui::DmgText;
use crate::entities::enemies::sight::*;

pub struct Boss {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
    dmg_text: Vec<DmgText>,
    pub invulnerable: Duration,
	pub line_of_sight: LineOfSight,
	pub ai_type: AITypes,
}

impl Boss {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32, ai_type: AITypes) -> Boss {
        let img = graphics::Image::new(ctx, "/pong_spritesheet.png").unwrap();
        let hb = img.dimensions();
        let dmg_text = Vec::new();

        Boss {
            x: xpos,
            y: ypos,
            hp: 20.0,
            atk: 3.0,
            def: 1.0,
            sprite: img,
            hitbox: hb,
            dmg_text,
            invulnerable: Duration::new(1u64, 0u32),
			line_of_sight: LineOfSight::new(xpos, ypos),
			ai_type,
        }
    }

    pub fn take_dmg(&mut self, ctx: &mut Context, dmg_to_take: f32) {
        if !self.invulnerable() {
            self.hp -= dmg_to_take;
            self.invulnerable = Duration::new(0u64, 0u32);
            self.dmg_text.push(DmgText::new(ctx, self.x, self.y, dmg_to_take));
            // Check for death and maybe call a death function.
        }
    }

    // returns if Boss should be able to take damage (time is 1/4 sec)
    fn invulnerable(&self) -> bool {
        self.invulnerable < Duration::from_millis(250u64)
    }
}

impl DrawableEntity for Boss {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.sprite, dp)?;

        self.dmg_text.iter().for_each(|t| t.draw(ctx));

        Ok(())
    }
}

impl CollideEntity for Boss {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.hitbox;
        r.x = self.x;
        r.y = self.y;
        r
    }
}

impl Enemy for Boss {
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, _level: &Level) {
        self.dmg_text.retain(|t| t.live());
        self.dmg_text.iter_mut().for_each(|t| t.update(delta));
        
        // cool down invulnerable of Boss
        if self.invulnerable() {
            self.invulnerable += delta;
        }

        if self.collision(player) {
            player.take_dmg(self.atk);
        }
        
        if let Some(atk) = &player.atk_box {
            if self.collision(atk) {
                self.take_dmg(ctx, player.atk);
            }
        }
    }

    fn islive(&self) -> bool {
        self.hp > 0.0
    }
	
	fn get_aitype(&mut self) -> &AITypes {
		&self.ai_type
	}
	
	fn chase_player(&mut self, _delta: Duration, player: &mut Player, level: &Level) {
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
		
		// Check wall collision
		if self.collision(level) {
			self.x = xpos;
			self.y = ypos;
		}
		
		// I touched the player.
		if self.collision(player) {
			// need attack animation
			player.take_dmg(self.atk);
			self.x = xpos;
			self.y = ypos;
		}
	}
	
	fn chase_player_sight(&mut self, delta: Duration, player: &mut Player, level: &Level) {
		self.line_of_sight.update(self.x - 100.0, self.y - 100.0, 200.0, 200.0);
		
		if self.line_of_sight.collision(player) {// && !self.line_of_sight.collision(level) {
			self.chase_player(delta, player, level);
		}
	}
}