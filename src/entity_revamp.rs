use std::time::Duration;

use ggez::{Context, GameResult};
use ggez::graphics::{BlendMode, Drawable, DrawParam, Image, Rect};

use crate::sprite_revamp::{Frame, Sprite};

#[derive(Clone)]
pub struct Entity {
    sprite: Sprite,
}

impl Entity {
    pub fn new(ctx: &mut Context) -> Entity {
        let img = Image::new(ctx, "/dapper-skeleton-sheet.png").unwrap();
        let frames = vec![
            Frame::new(Rect::new(64f32, 768f32, 64f32, 64f32), Duration::new(1u64, 0u32))
        ];
        let sprite = Sprite::new(&img, &frames).unwrap();
        Entity{
            sprite
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.sprite.update(delta);
    }
}

impl Drawable for Entity {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult{
        self.sprite.draw(ctx, param)
    }
    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        self.sprite.dimensions(ctx)
    }
    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.sprite.set_blend_mode(mode)
    }
    fn blend_mode(&self) -> Option<BlendMode> {
        self.sprite.blend_mode()
    }
}