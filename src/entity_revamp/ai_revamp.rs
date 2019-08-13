use std::collections::VecDeque;
use std::iter;

use ggez::{Context, GameResult};
use ggez::input::keyboard::{KeyCode, KeyMods, self};

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
    let p = entity.param.dest;

    if entity.param.dest.x < player.param.dest.x {
        entity.param.dest.x += 1f32
    } else if entity.param.dest.x > player.param.dest.x {
        entity.param.dest.x -= 1f32
    }

    // undo move if collision occurred
    for e in iter::once(&mut player.clone()).chain(enemies.iter_mut()) {
        if let Some(_) = entity.collision(e) {
            entity.param.dest = p;
        }
    }

    let p = entity.param.dest;

    if entity.param.dest.y < player.param.dest.y {
        entity.param.dest.y += 1f32
    } else if entity.param.dest.y > player.param.dest.y {
        entity.param.dest.y -= 1f32
    }

    // undo move if collision occurred
    for e in iter::once(&mut player.clone()).chain(enemies.iter_mut()) {
        if let Some(_) = entity.collision(e) {
            entity.param.dest = p;
        }
    }
}

// note `ignore` is probably a clone of the player or a dummy entity... so yeah.
pub fn player_input(ctx: &mut Context, player: &mut Entity, _ignore: &mut Entity, enemies: &mut VecDeque<Entity>) {
    let p = player.param.dest;
    
    // only doing simple input from player
    if keyboard::is_key_pressed(ctx, KeyCode::Right) {
        player.param.dest.x += 4f32;
    } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
        player.param.dest.x -= 4f32;
    } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
        player.param.dest.y += 4f32;
    } else if keyboard::is_key_pressed(ctx, KeyCode::Up) {
        player.param.dest.y -= 4f32;
    }

    // undo move if collision occurred
    for e in enemies.iter() {
        if let Some(_) = player.collision(e) {
            player.param.dest = p;
        }
    }
}