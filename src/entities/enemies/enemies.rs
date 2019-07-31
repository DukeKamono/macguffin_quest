use crate::entities::DrawableEntity;
use crate::entities::player::player::Player;
use crate::entities::environment::level::Level;
use std::time::Duration;
use ggez::*;
use crate::entities::enemies::ai::AI;

pub trait Enemy: DrawableEntity {
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, level: &Level);
    fn get_ai(&self) -> AI;
    fn islive(&self) -> bool;
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
    fn get_ai(&self) -> AI {
        AI {
        
        }
    }

    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, level: &Level) {
        // remove dead enemies
        self.enemies.retain(|e| e.islive());

        // update enemies
        self.enemies.iter_mut().for_each(|e| e.update(ctx, delta, player, level));

        if !self.islive() {
            // do something if there are no more enemies
            // maybe spawn some new ones
        }
    }

    // returns true if there are enemies
    fn islive(&self) -> bool {
        !self.enemies.is_empty()
    }
}