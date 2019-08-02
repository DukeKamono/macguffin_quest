use crate::entities::player::player::Player;
use crate::entities::environment::level::Level;
use crate::entities::enemies::enemies::Enemy;
use std::time::Duration;

pub struct AI {
	//ai_type: AITypes,
}

// These could be better named too.
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
    Error,
}

impl AI {
	pub fn new() -> AI {
		AI {
			//ai_type: ai_type,
		}
	}
	
	// Tried to pass in Enemy and do the movement and attack checks here, but now it will call different variations
	// that each enemy can do differntly. Like a skeleton chase_player can be differnt then a blob chase_player. 
	pub fn update(&mut self, delta: Duration, enemy: &mut Box<dyn Enemy>, player: &mut Player, level: &Level) {
		match enemy.get_aitype() {
			AITypes::MeleeDirect => enemy.chase_player(delta, player, level),
			AITypes::MeleeLineOfSight => enemy.chase_player_sight(delta, player, level),
			AITypes::RangeDirect => enemy.chase_player(delta, player, level),
			AITypes::RangeLineOfSight => enemy.chase_player_sight(delta, player, level),
			AITypes::Error => println!("Error"),
		}
	}
}