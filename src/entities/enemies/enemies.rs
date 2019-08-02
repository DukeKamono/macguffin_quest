use crate::entities::DrawableEntity;
use crate::entities::player::player::Player;
use crate::entities::environment::level::Level;
use std::time::Duration;
use ggez::*;
use crate::entities::enemies::ai::*;

pub trait Enemy: DrawableEntity {
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, level: &Level);
    fn islive(&self) -> bool;
	fn get_aitype(&mut self) -> &AITypes;
	fn chase_player(&mut self, _delta: Duration, player: &mut Player, level: &Level);
	fn chase_player_sight(&mut self, delta: Duration, player: &mut Player, level: &Level);
}

pub struct Enemies {
    enemies: Vec<Box<dyn Enemy>>,
}

impl Enemies {
    pub fn new() -> Enemies {
        Enemies {
            enemies: Vec::new(),
        }
    }

    // Add enemies.
    pub fn push(&mut self, enemy: Box<dyn Enemy>) {
        self.enemies.push(enemy)
    }
}

impl DrawableEntity for Enemies {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        for me in &self.enemies {
            me.draw(ctx)?;
        }
        Ok(())
    }
}

impl Enemy for Enemies {
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, level: &Level) {
        // remove dead enemies
        self.enemies.retain(|e| e.islive());
		
		let mut ai = AI::new();
        // update enemies
        self.enemies.iter_mut().for_each(|e| {
			e.update(ctx, delta, player, level);
			ai.update(delta, e, player, level);
		});

        if !self.islive() {
            // do something if there are no more enemies
            // maybe spawn some new ones
        }
    }

    // returns true if there are enemies
    fn islive(&self) -> bool {
        !self.enemies.is_empty()
    }
	
	// IDK what to do with these empty functions, which makes me second think this structure.
	fn get_aitype(&mut self) -> &AITypes {
		&AITypes::MeleeDirect
	}
	
	fn chase_player(&mut self, _delta: Duration, _player: &mut Player, _level: &Level) {
	}
	
	fn chase_player_sight(&mut self, _delta: Duration, _player: &mut Player, _level: &Level) {
	}
}