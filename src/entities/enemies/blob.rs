use ggez::nalgebra as na;
use ggez::*;
//use rand::prelude::*;

use super::super::CollideEntity;

pub struct Blob {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
}

impl Blob {
    pub fn new(ctx: &mut Context, _spawn: graphics::Rect) -> Blob {
        let img = graphics::Image::new(ctx, "/blob.png").unwrap();
        let hb = img.dimensions();

        Blob {
            x: 250.0,
            y: 250.0,
            hp: 10.0,
            atk: 3.0,
            def: 1.0,
            sprite: img,
            hitbox: hb,
        }
    }

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

impl CollideEntity for Blob {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.hitbox;
        r.x = self.x;
        r.y = self.y;
        r
    }
}