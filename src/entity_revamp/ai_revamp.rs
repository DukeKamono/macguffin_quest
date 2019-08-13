use std::collections::VecDeque;

use ggez::{Context, GameResult};
use ggez::event::{EventHandler, self};
use ggez::input::keyboard::{KeyCode, KeyMods};

use crate::entity_revamp::Entity;

// various ai functions for controlling entities with this type
pub type AIfn = fn(ctx: &mut Context, entity: &mut Entity, player: &mut Entity, enemies: &mut VecDeque<Entity> /*, level: Level, items: Iterator*/);

#[derive(Clone)]
pub struct AI {
    calls: Vec<AIfn>,
}

impl AI {
    pub fn new(calls: &Vec<AIfn>) -> Self {
        let mut ai = AI::default();
        ai.calls.extend(calls.iter());
        ai
    }

    pub fn execute(&self, ctx: &mut Context, entity: &mut Entity, player: &mut Entity, enemies: &mut VecDeque<Entity>) {
        for ai in &self.calls {
            ai(ctx, entity, player, enemies);
        }
    }
}

impl Default for AI {
    fn default() -> Self {
        AI{ calls: Vec::new() }
    }
}

// note `entity` is current enemy and should not be present in the enemies VecDeque
pub fn chase_player(ctx: &mut Context, entity: &mut Entity, player: &mut Entity, enemies: &mut VecDeque<Entity>) {
    println!("Enemy should chase player...");
}

// note `ignore` is probably a clone of the player or a dummy entity... so yeah.
pub fn player_input(ctx: &mut Context, player: &mut Entity, _ignore: &mut Entity, enemies: &mut VecDeque<Entity>) {
    println!("Input from player");
}