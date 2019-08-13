use std::collections::VecDeque;
use std::mem;
use std::time::Duration;

use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, DrawParam, Image, Rect};
use ggez::timer;

use crate::sprite_revamp::{Sprite, SpriteBuilder};

mod ai_revamp;
use ai_revamp::{AI};

#[derive(Clone)]
pub struct StatBlock {
    pub name: String,
    pub lvl: u64,
    pub exp: u64,
    pub health: (u64, u64),
    pub magic: (u64, u64),
    pub attack: u64,
    pub defense: u64,
    pub speed: f32
}

impl Default for StatBlock {
    fn default() -> StatBlock {
        StatBlock {
            name: "Default Entity Name".to_string(),
            lvl: 1u64,
            exp: 0u64,
            health: (10u64, 10u64),
            magic: (10u64, 10u64),
            attack: 1u64,
            defense: 0u64,
            speed: 1f32,
        }
    }
}

// useful for collisions
#[derive(Clone)]
pub enum EntityType {
    Player,
    Enemy,
    Item,
    Wall,
    Misc, // anything else (will probably use for ignored things) [or remove]
}
use EntityType::*;

#[derive(Clone)]
pub enum EntityState {
    Idle,
    Attacking, // attack box, duration
    Casting, // attack box, duration
    Walking,
    Dead, // duration
}
use EntityState::*;

#[derive(Clone)]
enum EntityDirection {
    Up,
    Down,
    Left,
    Right,
}
use EntityDirection::*;

    /*
        animations (with animated sprite to use)
        state (attacking, moving, damaged, dead, etc)
            - could make enum containing needed information
            - like attacking could have attack's hitbox and animation cooldown
        items (ie have the macguffin)
        floating text
        visible
        AI function (how act/update)
        sight radius
    */

#[derive(Clone)]
pub struct Entity {
    entitytype: EntityType,
    stats: StatBlock,
    sprite: Sprite, // change to have multiple animations [hitbox baked into frame information]
    param: DrawParam, // location as DrawParam.dest
    direction: EntityDirection,
    aibehavior: AI,
    state: EntityState,
    // not implemented
    // items
    // floating text
    // visible
}

impl Entity {
    fn new(entitytype: EntityType, stats: StatBlock, sprite: Sprite, param: DrawParam, direction: EntityDirection, aibehavior: AI, state: EntityState) -> Entity {
        Entity{
            entitytype,
            stats,
            sprite,
            param,
            direction,
            aibehavior,
            state
        }
    }

    pub fn update(&mut self, ctx: &mut Context, player: &mut Entity, enemies: &mut VecDeque<Entity>) {
        let delta = timer::delta(ctx);

        let mut ai = AI::default();
        mem::swap(&mut self.aibehavior, &mut ai);
        ai.execute(ctx, self, player, enemies);
        mem::swap(&mut self.aibehavior, &mut ai);

        self.sprite.update(delta);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        self.sprite.draw(ctx, self.param)
    }

    // translate

    // take damage

    // collision detection

    // is alive
}

/*
Entity Builder
builds an entity like player or skeleton
*/
pub struct EntityBuilder {
    // fields
}

impl EntityBuilder {
    pub fn build_player(ctx: &mut Context) -> GameResult<Entity> {
        let img = Image::new(ctx, "/dapper-skeleton-sheet.png")?;
        let sprite = SpriteBuilder::new(&img)
            .add_frame(Rect::new(0f32, 768f32, 64f32, 64f32), None, None, None)
            .build_sprite()
            ?;
        let stats = StatBlock::default();
        let param = DrawParam::default();
        let ai = AI::new(&vec![ai_revamp::player_input]);
        Ok(Entity::new(Player, stats, sprite, param, Down, ai, Idle))
    }

    pub fn build_enemy(ctx: &mut Context) -> GameResult<Entity> {
        let img = Image::new(ctx, "/dapper-skeleton-sheet.png")?;
        let sprite = SpriteBuilder::new(&img)
            .add_frame(Rect::new(128f32, 768f32, 64f32, 64f32), None, None, None)
            .build_sprite()
            ?;
        let stats = StatBlock::default();
        let param = DrawParam::default();
        let ai = AI::new(&vec![ai_revamp::chase_player]);
        Ok(Entity::new(Player, stats, sprite, param, Down, ai, Idle))
    }
}