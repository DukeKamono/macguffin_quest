// Namespace sprites
// Contains modules related to drawable sprites

// sprite based on subsection of larger image
//pub mod sprite;
include!("./sprite.rs");

// animated sprite based on subsections of larger image
//pub mod animated_sprite;
include!("./animated_sprite.rs");

// animated sprite builder for creating animated sprites from an image
// kind of dumb... could be more sophisticated
include!("./animated_builder.rs");

// for helper function
use ggez::graphics::Rect;

// helper function to test if smaller rectangle is inside a bigger rectangle
fn contains(bigger: &Rect, smaller: &Rect) -> bool {
    bigger.x <= smaller.x
        && bigger.y <= smaller.y
        && bigger.w >= smaller.x + smaller.w
        && bigger.h >= smaller.y + smaller.h
}