use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::*;

use super::super::{CollideEntity, DrawableEntity};

// constant values for keys used to determine movement
const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_LEFT: KeyCode = KeyCode::A;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub sprite: graphics::Image,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Player {
        let sprt = graphics::Image::new(ctx, "/pong_spritesheet.png").unwrap();

        Player {
            x: 10.0,
            y: 10.0,
            hp: 30.0,
            atk: 3.0,
            def: 2.0,
            sprite: sprt,
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

        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            println!("Attempting to attack. Atk: {}", self.atk);
        }
    }

    pub fn move_location(&mut self, xinc: f32, yinc: f32) {
        self.x = xinc;
        self.y = yinc;
    }

    pub fn take_dmg(&mut self, dmg_to_take: f32) {
        self.hp -= dmg_to_take;
        // Check for death and maybe call a death function.
        println!("hp is: {}", self.hp);
    }
}

impl DrawableEntity for Player {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, &self.sprite, dp)
    }
}

impl CollideEntity for Player {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.sprite.dimensions();
        r.x = self.x;
        r.y = self.y;
        r
    }
}
