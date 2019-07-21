use ggez::nalgebra as na;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::*;


// constant values for keys used to determine movement
const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_LEFT: KeyCode = KeyCode::A;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub sprite: graphics::Image,
    pub hitbox: graphics::Rect,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Player {
        let sprt = graphics::Image::new(ctx, "/pong_spritesheet.png").unwrap();

        let hp = sprt.dimensions();

        Player {
            x: 10.0,
            y: 10.0,
            hp: 30.0,
            sprite: sprt,
            hitbox: hp,
        }
    }

    // Increase or decrease `position_x` by 0.5, or by 5.0 if Shift is held.
    pub fn update(&mut self, ctx: &mut Context) {
        // private function to return correct speed
        fn move_increment(ctx: &mut Context) -> f32 {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                return 5.0;
            }
            0.5
        }

        if keyboard::is_key_pressed(ctx, KEY_RIGHT) {
            self.x += move_increment(ctx);
        } else if keyboard::is_key_pressed(ctx, KEY_LEFT) {
            self.x -= move_increment(ctx);
        }
        if keyboard::is_key_pressed(ctx, KEY_UP) {
            self.y -= move_increment(ctx);
        } else if keyboard::is_key_pressed(ctx, KEY_DOWN) {
            self.y += move_increment(ctx);
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        // This sets the location of the thing going to be drawn. (player)
        let draw_param = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        // This draws the player.
        graphics::draw(ctx, &self.sprite, draw_param).expect("Can't display Player!");
    }

    pub fn hit_box(&self) -> graphics::Rect {
       let mut r = self.hitbox.clone();
       r.x = self.x;
       r.y = self.y;
       r
    }

    pub fn collide(&self, other: &super::Wall) -> bool {
        self.hit_box().overlaps(&other.hit_box())
    }

    pub fn move_location(&mut self, xinc: f32, yinc: f32) {
        self.x += xinc;
        self.y += yinc;
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
//impl MyCollideTrait for Player {
//    fn hit_box(&self) -> graphics::Rect {
//        let mut r = self.hitbox.clone();
//        r.x = self.x;
//        r.y = self.y;
//        r
//    }
//    fn collision<T>(&self, other: &T) -> bool
//    where
//        T: MyCollideTrait,
//    {
//        self.hitbox.overlaps(&other.hit_box())
//    }
//}