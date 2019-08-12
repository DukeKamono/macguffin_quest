use std::time::Duration;

use ggez::{Context, GameResult};
use ggez::graphics::{BlendMode, Drawable, DrawParam, Image, Rect};

use crate::sprite_revamp::{Sprite, SpriteBuilder};

#[derive(Clone)]
pub struct Entity {
    sprite: Sprite,
}

impl Entity {
    pub fn new(ctx: &mut Context) -> Entity {
        let img = Image::new(ctx, "/dapper-skeleton-sheet.png").unwrap();
        // Duration::new(1u64, 500_000_000u32)
        let sprite = SpriteBuilder::new(&img)
            //.add_frames_going_up(4usize, Rect::new(0f32, 256f32, 64f32, 64f32), None, None, None)
            .add_frames_going_down(4usize, Rect::new(0f32, 576f32, 64f32, 64f32), None, None, None)
            //.add_frames_going_left(9usize, Rect::new(576f32, 0f32, 64f32, 64f32), None, None, None)
            //.add_frames_going_right(6usize, Rect::new(0f32, 768f32, 64f32, 64f32), None, None, None)
            //.add_frame(Rect::new(64f32, 768f32, 64f32, 64f32), None, None, None)
            //.build_sprite()
            .build_looping_sprite()
            .unwrap();
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