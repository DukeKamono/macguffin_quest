use super::super::CollideEntity;
use ggez::*;

// Was wanting to use a line, but we only check collision with rects right now.
pub struct LineOfSight {
    pub line_of_sight: graphics::Rect,
}

impl LineOfSight {
    pub fn new(xpos: f32, ypos: f32) -> LineOfSight {
        LineOfSight {
            line_of_sight: graphics::Rect::new(xpos, ypos, 1.0, 1.0),
        }
    }

    pub fn update(&mut self, xpos: f32, ypos: f32, width: f32, height: f32) {
        self.line_of_sight = graphics::Rect::new(xpos, ypos, width, height);
    }
}

impl CollideEntity for LineOfSight {
    fn get_hitbox(&self) -> graphics::Rect {
        self.line_of_sight
    }
}
