// Namespace entities
// Contains modules related to objects that can appear/impact the game.


// Note declared modules need to be public so that main.rs can use them
// - If they are not public only the entities namespace and modules declared in
//   the entities namespace will be able to see/use them


// Namespace for things related to the player character
pub mod player;

// Namespace for things related to enemies
pub mod enemies;

// Namespace for things related to the environment (ie level)
pub mod environment;


// required by traits
use ggez::{Context, GameResult};
use ggez::graphics::{Rect};

// trait used to mark drawable entities
// must implement ggez::graphics::Drawable (but not right now)
pub trait DrawableEntity/*: Drawable */{
    fn draw(&self, ctx: &mut Context) -> GameResult;
}

// trait used to mark entities that may collide with each other
pub trait CollideEntity {
    // get hitbox of CollideEntity
    fn get_hitbox(&self) -> Rect;

    // true / false if two CollideEntity's overlap (ie collide)
    fn collision(&self, other: &CollideEntity) -> bool {
        self.get_hitbox().overlaps(&other.get_hitbox())
    }
}