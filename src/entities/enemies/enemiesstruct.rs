use crate::entities::enemies::ai::*;
use crate::entities::enemies::ghost::Ghost;
use crate::entities::environment::level::Level;
use crate::entities::player::playerstruct::Player;
use crate::entities::DrawableEntity;
use ggez::*;
use rand::prelude::*;
use std::time::Duration;

pub trait Enemy: DrawableEntity {
    fn update(&mut self, ctx: &mut Context, delta: Duration, player: &mut Player, level: &Level);
    fn islive(&self) -> bool;
    fn get_aitype(&mut self) -> &AITypes;
    fn chase_player(
        &mut self,
        ctx: &mut Context,
        _delta: Duration,
        player: &mut Player,
        level: &Level,
    );
    fn chase_player_sight(
        &mut self,
        ctx: &mut Context,
        delta: Duration,
        player: &mut Player,
        level: &Level,
    );
    fn spawn(&self) -> bool;
}

#[derive(Default)]
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
        let mut spawning = false;

        // remove dead enemies
        self.enemies.retain(|e| e.islive());

        let mut ai = AI::new();
        // update enemies
        self.enemies.iter_mut().for_each(|e| {
            e.update(ctx, delta, player, level);
            ai.update(ctx, delta, e.as_mut(), player, level);
            // I need to find a better spot for this spawning, keeps spawning ghosts if spawn comes back true. (boss only)
            if e.spawn() {
                spawning = true;
            }
        });

        if spawning {
            let mut rng = thread_rng();
            self.enemies.push(Box::new(Ghost::new(
                ctx,
                rng.gen_range(0, 800) as f32,
                rng.gen_range(0, 800) as f32,
                AITypes::MeleeDirect,
            )));
        }

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

    fn chase_player(
        &mut self,
        _ctx: &mut Context,
        _delta: Duration,
        _player: &mut Player,
        _level: &Level,
    ) {
    }

    fn chase_player_sight(
        &mut self,
        _ctx: &mut Context,
        _delta: Duration,
        _player: &mut Player,
        _level: &Level,
    ) {
    }

    fn spawn(&self) -> bool {
        true
    }
}
