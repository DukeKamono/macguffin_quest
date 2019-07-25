use ggez::{GameResult, Context, nalgebra::Point2};
use ggez::graphics::{DrawParam, draw, Image, Rect};

use super::super::{CollideEntity, DrawableEntity};

pub struct Tile {
    image: Image,
    xpos: f32,
    ypos: f32,
}

impl Tile {
    pub fn new(image: &Image, xpos: f32, ypos: f32) -> Self {
        let image = image.clone();
        Tile{
            image,
            xpos,
            ypos,
        }
    }
}

impl DrawableEntity for Tile {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = DrawParam::default().dest(Point2::new(self.xpos, self.ypos));
        draw(ctx, &self.image, dp)
    }
}

impl CollideEntity for Tile {
    fn get_hitbox(&self) -> Rect {
        let mut r = self.image.dimensions();
        r.x = self.xpos;
        r.y = self.ypos;
        r
    }
}