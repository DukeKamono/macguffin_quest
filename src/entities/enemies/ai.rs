use crate::entities::player::player::Player;
use crate::entities::enemies::blob::Blob;
use crate::entities::environment::level::Level;
use crate::entities::enemies::enemies::Enemy;
use ggez::nalgebra as na;
use ggez::*;
use std::time::Duration;

pub struct AI {
	//ai_type: AITypes,
}

pub enum AITypes {
    // Move directly to the player and don't stop
    MeleeDirect,
    // When you see the player then go towards.
    MeleeLineOfSight,
    // Keep firing and go towards the player.
    RangeDirect,
    // When you see the player then go towards and fire at them.
    RangeLineOfSight,
    // an error occurred and needs reported.
    Error(GameError),
}

impl AI {
	pub fn new() -> AI {
		AI {
			//ai_type: ai_type,
		}
	}
	
	pub fn update(&mut self, delta: Duration, enemy: &mut Box<dyn Enemy>, player: &mut Player, level: &Level) {
		// holding onto previous location
		//let xpos = enemy.x;
		//let ypos = enemy.y;
		//
		//// Move this out to ai later. Charge towards player.
		//if enemy.x != player.x {
		//	if enemy.x > player.x {
		//		enemy.x -= 1.0;
		//	}
		//	if enemy.x < player.x {
		//		enemy.x += 1.0;
		//	}
		//}
		//if enemy.y != player.y {
		//	if enemy.y > player.y {
		//		enemy.y -= 1.0;
		//	}
		//	if enemy.y < player.y {
		//		enemy.y += 1.0;
		//	}
		//}
		//
		//// after moving check wall collision
		//if enemy.collision(level) {
		//	enemy.x = xpos;
		//	enemy.y = ypos;
		//}
		//
		//// I touched the player.
        //if enemy.collision(player) {
		//	// need attack animation
        //    player.take_dmg(enemy.atk);
		//	enemy.x = xpos;
		//	enemy.y = ypos;
        //}
	}
}