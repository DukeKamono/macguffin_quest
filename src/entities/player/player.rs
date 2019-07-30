use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::*;
use ggez::graphics::{Image, Rect};
use std::collections::HashMap;
use std::time::Duration;

use super::super::{CollideEntity, DrawableEntity};
use super::atk_box::AtkBox;
use crate::sprites::*;

// constant values for keys used to determine movement
const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_LEFT: KeyCode = KeyCode::A;

#[derive(PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Animations {
    Walking
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub hp: f32,
    pub atk: f32,
    pub def: f32,
    pub sprite: HashMap<(Animations, Direction), AnimatedSprite>,
    pub animation: (Animations, Direction),
    pub atk_box: Option<AtkBox>,
    pub attacking: bool,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Player {
        let mut sprite = HashMap::new();
        let sheet = Image::new(ctx, "/dapper-skeleton-sheet.png").unwrap();
        let builder = AnimatedBuilder::new(&sheet);
        sprite.insert(
            (Animations::Walking, Direction::Up),
            builder.create_animated(Rect::new(64f32, 0f32, 64f32, 64f32), 8usize).unwrap()
        );
        sprite.insert(
            (Animations::Walking, Direction::Left),
            builder.create_animated(Rect::new(64f32, 64f32, 64f32, 64f32), 8usize).unwrap()
        );
        sprite.insert(
            (Animations::Walking, Direction::Down),
            builder.create_animated(Rect::new(64f32, 128f32, 64f32, 64f32), 8usize).unwrap()
        );
        sprite.insert(
            (Animations::Walking, Direction::Right),
            builder.create_animated(Rect::new(64f32, 192f32, 64f32, 64f32), 8usize).unwrap()
        );

        Player {
            x: 10.0,
            y: 10.0,
            hp: 30.0,
            atk: 3.0,
            def: 2.0,
            sprite,
            animation: (Animations::Walking, Direction::Right),
            atk_box: None,
            attacking: false,
        }
    }

    // Increase or decrease `position_x` by 0.5, or by 5.0 if Shift is held.
    pub fn update(&mut self, ctx: &mut Context, delta: Duration) {
        // private function to return correct speed
        fn move_increment(ctx: &mut Context) -> f32 {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                return 5.0;
            }
            0.5
        }

        if keyboard::is_key_pressed(ctx, KEY_RIGHT) {
            self.x += move_increment(ctx);
            self.animation = (Animations::Walking, Direction::Right);
        } else if keyboard::is_key_pressed(ctx, KEY_LEFT) {
            self.x -= move_increment(ctx);
            self.animation = (Animations::Walking, Direction::Left);
        }
        if keyboard::is_key_pressed(ctx, KEY_UP) {
            self.y -= move_increment(ctx);
            self.animation = (Animations::Walking, Direction::Up);
        } else if keyboard::is_key_pressed(ctx, KEY_DOWN) {
            self.y += move_increment(ctx);
            self.animation = (Animations::Walking, Direction::Down);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.atk_box = Some(AtkBox::new(ctx, 2.0, self.x, self.y));
        } else {
            self.atk_box = None;
        }

        self.sprite.get_mut(&self.animation).unwrap().animate(delta);
    }

    pub fn move_location(&mut self, xinc: f32, yinc: f32) {
        self.x = xinc;
        self.y = yinc;
    }

    pub fn take_dmg(&mut self, dmg_to_take: f32) {
        self.hp -= dmg_to_take;
        // Check for death and maybe call a death function.
    }

    // With multiple weapons, we should make a new struct for each type and attach them to the player.
    pub fn draw_weapon(&self, ctx: &mut Context) {
        if let Some(atk) = &self.atk_box {
            atk.draw(ctx).expect("Failed to draw attack.");
        }
    }
}

impl DrawableEntity for Player {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.draw_weapon(ctx);
        let dp = graphics::DrawParam::default().dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, self.sprite.get(&self.animation).unwrap(), dp)
    }
}

impl CollideEntity for Player {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.sprite.get(&self.animation).unwrap().dimensions().unwrap();
        r.x = self.x;
        r.y = self.y;
        r
    }
}
