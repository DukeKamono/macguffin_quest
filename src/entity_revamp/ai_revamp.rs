use crate::entity_revamp::Entity;

// various ai functions for controlling entities
pub type AIfn = fn(&mut Entity, &mut Entity /*, level: Level, enemies: Iterator Entity, items: Iterator*/);

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

    pub fn execute(&self, entity: &mut Entity, player: &mut Entity) {
        for ai in &self.calls {
            ai(entity, player);
        }
    }
}

impl Default for AI {
    fn default() -> Self {
        AI{ calls: Vec::new() }
    }
}

pub fn chase_player(entity: &mut Entity, player: &mut Entity /*, level: Level, enemies: Iterator Entity, items: Iterator*/) {
    println!("Enemy should chase player...");
}