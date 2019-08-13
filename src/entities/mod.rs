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

// Namespace for things related to items
pub mod items;

// Namespace for things related to npcs
pub mod npcs;

// required by traits
use ggez::graphics::Rect;
use ggez::{Context, GameResult};

#[derive(PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Animations {
    Stand,
    Walking,
    Cast,
    Slash,
    Die,
}

// trait used to mark drawable entities
// must implement ggez::graphics::Drawable (but not right now)
pub trait DrawableEntity /*: Drawable */ {
    fn draw(&self, ctx: &mut Context) -> GameResult;
}

// trait used to mark entities that may collide with each other
pub trait CollideEntity {
    // get hitbox of CollideEntity
    fn get_hitbox(&self) -> Rect;

    // gets iterator over all hitboxes of CollideEntity
    // ok it really returns a vec cause I cant figure out iterators
    fn get_sub_hitboxs(&self) -> Vec<Rect> {
        //Box::new(std::iter::once(self.get_hitbox()))
        vec![self.get_hitbox()]
    }

    // true / false if two CollideEntity's overlap (ie collide)
    fn collision(&self, other: &dyn CollideEntity) -> bool {
        // could improve this using iterator chains... maybe
        for myhb in self.get_sub_hitboxs() {
            for ohb in other.get_sub_hitboxs() {
                if myhb.overlaps(&ohb) {
                    return true;
                }
            }
        }
        false
    }
}
