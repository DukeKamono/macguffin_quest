use ggez::*;
use std::mem;

use super::super::{CollideEntity, Direction, DrawableEntity};

pub struct AtkBox {
    pub x: f32,
    pub y: f32,
    pub hitbox: graphics::Rect,
    pub shape: graphics::Mesh,
}

impl AtkBox {
    pub fn new(
        ctx: &mut Context,
        xpos: f32,
        ypos: f32,
        width: f32,
        height: f32,
        direction: &Direction,
        offset: f32,
    ) -> AtkBox {
        let mut xpos = xpos;
        let mut ypos = ypos;
        let mut h: &f32 = &height;
        let mut w: &f32 = &width;

        // The player is kinda off centered and these values need to be adjusted.
        match direction {
            Direction::Up => {
                ypos -= offset;
                mem::swap(&mut h, &mut w);
            }
            Direction::Down => {
                ypos += offset;
                mem::swap(&mut h, &mut w);
            }
            Direction::Left => xpos -= offset,
            Direction::Right => xpos += offset,
        }

        // create hit box
        let hb = graphics::Rect::new(0.0, 0.0, *w, *h);
        // create mesh
        let square = graphics::MeshBuilder::new()
            .rectangle(graphics::DrawMode::fill(), hb, graphics::WHITE)
            .build(ctx)
            .unwrap();

        xpos -= hb.w / 2f32;
        ypos -= hb.h / 2f32;

        AtkBox {
            x: xpos,
            y: ypos,
            hitbox: hb,
            shape: square,
        }
    }
}

impl DrawableEntity for AtkBox {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(nalgebra::Point2::new(self.x, self.y));
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
