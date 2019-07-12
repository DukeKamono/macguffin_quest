use ggez::nalgebra as na;
use ggez::*;

pub struct Blob {
    pub x: f32,
    pub y: f32,
    pub sprite: graphics::Image,
    //hitbox: graphics::Image,
}

impl Blob {
    pub fn new(ctx: &mut Context) -> Blob {
        Blob {
            x: 250.0,
            y: 250.0,
            sprite: graphics::Image::new(ctx, "/blob.png").unwrap(),
            //hitbox: graphics::Image::new(ctx, "/assets/pong_spritesheet.png").unwrap(),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        // This sets the location of the thing going to be drawn. (blob)
        let draw_param = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        // This draws the blob.
        graphics::draw(ctx, &self.sprite, draw_param).expect("Can't display blob!");
    }
}
