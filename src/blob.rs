use ggez::nalgebra as na;
use ggez::*;
use rand::prelude::*;

pub struct Blob {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    spawn_range: graphics::Rect,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
}

impl Blob {
    pub fn new(ctx: &mut Context, spawn: graphics::Rect) -> Blob {
        Blob {
            x: 250.0,
            y: 250.0,
            hp: 10.0,
            spawn_range: spawn,
            sprite: graphics::Image::new(ctx, "/blob.png").unwrap(),
            hitbox: graphics::Rect::new(0.0, 0.0, 50.0 * 2.0, 50.0 * 2.0),//graphics::Image::new(ctx, "/pong_spritesheet.png").unwrap(),
        }
    }

    // really should make use of Player.rect() method... but that does not exist
    pub fn collide(&self, other: &super::Player) -> bool {
        let mut self_rectangle = self.sprite.dimensions();
        let mut other_rectangle = other.sprite.dimensions();
        self_rectangle.move_to([self.x, self.y]);
        other_rectangle.move_to([other.x, other.y]);
        self_rectangle.overlaps(&other_rectangle)
    }

    pub fn relocate(&mut self) {
            let mut rng = thread_rng();
            self.x = rng.gen_range(self.spawn_range.x, self.spawn_range.w - self.sprite.width() as f32);
            self.y = rng.gen_range(self.spawn_range.y, self.spawn_range.h - self.sprite.width() as f32);
    }

    pub fn draw(&self, ctx: &mut Context) {
        // This sets the location of the thing going to be drawn. (blob)
        let draw_param = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        // This draws the blob.
        graphics::draw(ctx, &self.sprite, draw_param).expect("Can't display blob!");
    }
}

//trait MyCollideTrait {
//    fn hit_box(&self) -> graphics::Rect;
//    // not sure if this is right
//    fn collision<T>(&self, other: &T) -> bool
//    where
//        T: MyCollideTrait;
//}
//
//impl MyCollideTrait for Blob {
//    fn hit_box(&self) -> graphics::Rect {
//        let mut r = self.hit_box().clone();
//        r.x = self.x;
//        r.y = self.y;
//        r
//    }
//    fn collision<T>(&self, other: &T) -> bool
//    where
//        T: MyCollideTrait,
//    {
//        self.hit_box().overlaps(&other.hit_box())
//    }
//}