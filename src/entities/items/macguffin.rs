use ggez::nalgebra as na;
use ggez::*;

use super::super::{CollideEntity, DrawableEntity};

pub struct Macguffin {
    pub x: f32,
    pub y: f32,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
}

impl Macguffin {
    pub fn new(ctx: &mut Context, xpos: f32, ypos: f32) -> Macguffin {
        let img = graphics::Image::new(ctx, "/blob.png").unwrap();
        let hb = img.dimensions();

        Macguffin {
            x: xpos,
            y: ypos,
            sprite: img,
            hitbox: hb,
        }
    }
}

impl DrawableEntity for Macguffin {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.sprite, dp)
    }
}

impl CollideEntity for Macguffin {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.hitbox;
        r.x = self.x;
        r.y = self.y;
        r
    }
}
