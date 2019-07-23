// Namespace sprites
// Contains modules related to drawable sprites

// sprite based on subsection of larger image
pub mod sprite;

// animated sprite based on subsections of larger image
pub mod animated_sprite;

// for helper function
use ggez::graphics::Rect;

// helper function to test if smaller rectangle is inside a bigger rectangle
fn contains(bigger: &Rect, smaller: &Rect) -> bool {
    bigger.x <= smaller.x
        && bigger.y <= smaller.y
        && bigger.w >= smaller.x + smaller.w
        && bigger.h >= smaller.y + smaller.h
}