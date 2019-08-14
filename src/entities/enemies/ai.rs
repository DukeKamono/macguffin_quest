use crate::entities::enemies::enemiesstruct::Enemy;
use crate::entities::environment::level::Level;
use crate::entities::player::playerstruct::Player;
use ggez::*;
use std::time::Duration;

/// AI struct. As we work on more with this there will be more to it.
#[derive(Default)]
pub struct AI {}

/// A rough setup of differnt Types of AI the enemies can have.
pub enum AITypes {
    // Move directly to the player and don't stop
    MeleeDirect,
    // When you see the player then go towards.
    MeleeLineOfSight,
    // Keep firing and go towards the player.
    RangeDirect,
    // When you see the player then go towards and fire at them.
    RangeLineOfSight,
    // Boss
    Boss,
    // an error occurred and needs reported.
    Error,
}

/// Impliments the functions for the AI struct.
impl AI {
	/// Creates a new AI struct and returns it.
    pub fn new() -> AI {
        AI {}
    }

    /// This function takes pretty much everything in the level and depending on the
	/// AI Type set for the enemy points to what it is supposed to do.
    pub fn update(
        &mut self,
        ctx: &mut Context,
        delta: Duration,
        enemy: &mut dyn Enemy,
        player: &mut Player,
        level: &Level,
    ) {
        match enemy.get_aitype() {
            AITypes::MeleeDirect => enemy.chase_player(ctx, delta, player, level),
            AITypes::MeleeLineOfSight => enemy.chase_player_sight(ctx, delta, player, level),
            AITypes::RangeDirect => enemy.chase_player(ctx, delta, player, level),
            AITypes::RangeLineOfSight => enemy.chase_player_sight(ctx, delta, player, level),
            AITypes::Boss => {
                enemy.chase_player(ctx, delta, player, level);
                enemy.chase_player_sight(ctx, delta, player, level);
            }
            AITypes::Error => println!("Error"),
        }
    }
}