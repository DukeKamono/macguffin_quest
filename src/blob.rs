use ggez::nalgebra as na;
use ggez::*;
//use rand::prelude::*;

pub struct Blob {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    //spawn_range: graphics::Rect,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
}

impl Blob {
    pub fn new(ctx: &mut Context, _spawn: graphics::Rect) -> Blob {
        Blob {
            x: 250.0,
            y: 250.0,
            hp: 10.0,
            atk: 3.0,
            def: 1.0,
            //spawn_range: spawn,
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

    // Might use this logic later for respawn or something
    //pub fn relocate(&mut self) {
    //        let mut rng = thread_rng();
    //        self.x = rng.gen_range(self.spawn_range.x, self.spawn_range.w - self.sprite.width() as f32);
    //        self.y = rng.gen_range(self.spawn_range.y, self.spawn_range.h - self.sprite.width() as f32);
    //}

    pub fn draw(&self, ctx: &mut Context) {
        // This sets the location of the thing going to be drawn. (blob)
        let draw_param = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        // This draws the blob.
        graphics::draw(ctx, &self.sprite, draw_param).expect("Can't display blob!");
    }
    
    // Need to figure out how to do player attacks to hit monsters.
    //pub fn take_dmg(&mut self, dmg_to_take: f32) {
    //    self.hp -= dmg_to_take;
    //    // Check for death and maybe call a death function.
    //    println!("hp is: {}", self.hp);
    //}
}