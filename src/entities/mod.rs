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
use ggez::graphics::{Drawable, Rect};

// trait used to mark drawable entities
// must implement ggez::graphics::Drawable
trait DrawableEntity: Drawable {}

// trait used to mark entities that may collide with each other
trait CollideEntity {
    fn get_hitbox(&self) -> Rect;
    fn collision(&self, other: &CollideEntity) -> bool {
        self.get_hitbox().overlaps(&other.get_hitbox())
    }
}