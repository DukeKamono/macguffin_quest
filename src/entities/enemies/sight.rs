use super::super::CollideEntity;
use ggez::*;

/// Struct for the LineOfSight.
pub struct LineOfSight {
    pub line_of_sight: graphics::Rect,
}

/// Functions for the LineOfSight struct
impl LineOfSight {
    /// News up a LineOfSight struct with a default Rect.
    pub fn new(xpos: f32, ypos: f32) -> LineOfSight {
        LineOfSight {
            line_of_sight: graphics::Rect::new(xpos, ypos, 1.0, 1.0),
        }
    }

    /// Updates the position of the Rect to be around the Entity using the LineOfSight struct
    pub fn update(&mut self, xpos: f32, ypos: f32, width: f32, height: f32) {
        self.line_of_sight = graphics::Rect::new(xpos, ypos, width, height);
    }
}

/// Sets up the hitbox of the Rect to be used for collision.
impl CollideEntity for LineOfSight {
    fn get_hitbox(&self) -> graphics::Rect {
        self.line_of_sight
    }
}
