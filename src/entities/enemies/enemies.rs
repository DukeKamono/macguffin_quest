use crate::entities::DrawableEntity;
use crate::entities::player::player::Player;
use std::time::Duration;
use ggez::*;
use crate::entities::enemies::ai::AI;

pub trait Enemy: DrawableEntity {
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player);
    fn get_ai(&self) -> AI;
    fn isdead(&mut self) -> bool;
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

     // Remove enemies.
    //pub fn remove(&mut self, enemy: Box<dyn Enemy>) {
    //    let mut i = 0;
    //    while i != self.enemies.len() {
    //        if self.enemies[i].isdead() {
    //           let val = self.enemies.remove(i);
    //           // your code here
    //        } else {
    //            i += 1;
    //        }
    //    }
    //}
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

    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player) {
        for me in &mut self.enemies {
            me.update(ctx, delta, player);
        }
        if self.isdead() {
            // I don't like this implementation. I was tired, sorry.
            // I guess we can add death animation check here?
        }
    }

    // Always returns true...... sorry
    fn isdead(&mut self) -> bool {
        let mut i = 0;
        while i != self.enemies.len() {
            if self.enemies[i].isdead() {
               self.enemies.remove(i);
            } else {
                i += 1;
            }
        }
        true
    }
}