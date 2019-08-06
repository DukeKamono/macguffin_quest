use ggez::*;

use super::super::{CollideEntity, DrawableEntity, Direction};

pub struct AtkBox {
    pub duration: f32,
    pub x: f32,
    pub y: f32,
    pub hitbox: graphics::Rect,
    pub shape: graphics::Mesh,
}

impl AtkBox {
    pub fn new(ctx: &mut Context, duration: f32, xpos: f32, ypos: f32, width: f32, height: f32, direction: &Direction, offset: f32) -> AtkBox {
        
        //let mut xpos = xpos;
        //let mut ypos = ypos;
        
        // The player is kinda off centered and these values need to be adjusted.
        //match direction {
        //    Direction::Up => ypos -= offset,
        //    Direction::Down => ypos += offset,
        //    Direction::Left => xpos -= offset,
        //    Direction::Right => xpos += offset,
        //}
        
        // create hit box
        let hb = graphics::Rect::new(offset+50.0, offset, width, height);
        // create mesh
        let square = graphics::MeshBuilder::new()
            .rectangle(graphics::DrawMode::fill(), hb, graphics::WHITE)
            .build(ctx)
            .unwrap();

        AtkBox {
            duration,
            x: xpos,
            y: ypos,
            hitbox: hb,
            shape: square,
        }
    }
}

impl DrawableEntity for AtkBox {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().offset(nalgebra::Point2::new(0.5, 0.5)).dest(nalgebra::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.shape, dp)
    }
}

impl CollideEntity for AtkBox {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.hitbox;
        r.x = self.x;
        r.y = self.y;
        r
    }
}
