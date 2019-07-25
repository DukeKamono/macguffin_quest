use ggez::{GameResult, Context, nalgebra::Point2};
use ggez::graphics::{DrawParam, draw, Rect};

use crate::sprites::Sprite;

use super::super::{CollideEntity, DrawableEntity};

pub struct Tile {
    image: Sprite,
    hitbox: Rect,
}

impl Tile {
    pub fn new(image: &Sprite, xpos: f32, ypos: f32) -> Self {
        let image = image.clone();
        let hitbox = Rect::new(xpos, ypos, image.width(), image.height());
        Tile{
            image,
            hitbox,
        }
    }
}

impl DrawableEntity for Tile {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = DrawParam::default().dest(Point2::new(self.hitbox.x, self.hitbox.y));
        draw(ctx, &self.image, dp)
    }
}

impl CollideEntity for Tile {
    fn get_hitbox(&self) -> Rect {
        self.hitbox
    }
}