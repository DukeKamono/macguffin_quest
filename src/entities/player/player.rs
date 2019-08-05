use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::*;
use ggez::graphics::{Image, Rect};
use std::collections::HashMap;
use std::time::Duration;
use super::stats::Stats;
use super::super::{CollideEntity, DrawableEntity, Direction};
use super::atk_box::AtkBox;
use crate::sprites::*;

// constant values for keys used to determine movement
const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_LEFT: KeyCode = KeyCode::A;

#[derive(PartialEq, Eq, Hash)]
pub enum Animations {
    Stand,
    Walking,
    Cast,
    Slash,
    Die,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub stats: Stats,
    pub sprite: HashMap<(Animations, Direction), AnimatedSprite>,
    pub animation: (Animations, Direction),
    pub atk_box: Option<AtkBox>,
    pub attacking: bool,
    pub atk_cooldown: Duration,
    pub invulnerable: Duration,
    pub direction: Direction,
}

impl Player {
    pub fn new(ctx: &mut Context, chosen_player: String) -> Player {
        let mut sprite = HashMap::new();
        let sheet = Image::new(ctx, chosen_player).unwrap();
        let builder = AnimatedBuilder::new(&sheet);
        // standing
        sprite.insert(
            (Animations::Stand, Direction::Up),
            builder.create_animated(Rect::new(0f32, 0f32, 64f32, 64f32), 1usize).unwrap()
        );
        sprite.insert(
            (Animations::Stand, Direction::Left),
            builder.create_animated(Rect::new(0f32, 64f32, 64f32, 64f32), 1usize).unwrap()
        );
        sprite.insert(
            (Animations::Stand, Direction::Down),
            builder.create_animated(Rect::new(0f32, 128f32, 64f32, 64f32), 1usize).unwrap()
        );
        sprite.insert(
            (Animations::Stand, Direction::Right),
            builder.create_animated(Rect::new(0f32, 192f32, 64f32, 64f32), 1usize).unwrap()
        );
        // walking
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
        // casting
        sprite.insert(
            (Animations::Cast, Direction::Up),
            builder.create_animated(Rect::new(64f32, 512f32, 64f32, 64f32), 6usize).unwrap()
        );
        sprite.insert(
            (Animations::Cast, Direction::Left),
            builder.create_animated(Rect::new(64f32, 576f32, 64f32, 64f32), 6usize).unwrap()
        );
        sprite.insert(
            (Animations::Cast, Direction::Down),
            builder.create_animated(Rect::new(64f32, 640f32, 64f32, 64f32), 6usize).unwrap()
        );
        sprite.insert(
            (Animations::Cast, Direction::Right),
            builder.create_animated(Rect::new(64f32, 704f32, 64f32, 64f32), 6usize).unwrap()
        );
        // slashing
        sprite.insert(
            (Animations::Slash, Direction::Up),
            builder.create_animated(Rect::new(64f32, 256f32, 64f32, 64f32), 6usize).unwrap()
        );
        sprite.insert(
            (Animations::Slash, Direction::Left),
            builder.create_animated(Rect::new(64f32, 320f32, 64f32, 64f32), 6usize).unwrap()
        );
        sprite.insert(
            (Animations::Slash, Direction::Down),
            builder.create_animated(Rect::new(64f32, 384f32, 64f32, 64f32), 6usize).unwrap()
        );
        sprite.insert(
            (Animations::Slash, Direction::Right),
            builder.create_animated(Rect::new(64f32, 448f32, 64f32, 64f32), 6usize).unwrap()
        );
        // die
        sprite.insert(
            (Animations::Die, Direction::Down),
            builder.create_animated_once(Rect::new(64f32, 768f32, 64f32, 64f32), 5usize).unwrap()
        );


        Player {
            x: 10.0,
            y: 10.0,
            stats: Stats::new(1, 0, 30.0, 150, 3.0, 2.0, 1.0, 15.0),
            sprite,
            animation: (Animations::Walking, Direction::Right),
            atk_box: None,
            attacking: false,
            atk_cooldown: Duration::new(0u64, 0u32),
            invulnerable: Duration::new(0u64, 0u32),
            direction: Direction::Right, // Starting direction?
        }
    }

    // Increase or decrease `position_x` by 2.5, or by 5.0 if Shift is held.
    pub fn update(&mut self, ctx: &mut Context, delta: Duration) {
        // private function to return correct speed
        fn move_increment(ctx: &mut Context) -> f32 {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                return 5.0;
            }
            2.5
        }

        // clear attack box in case player is not attacking any more
        self.atk_box = None;

        // cool down invulnerable of player
        if self.invulnerable() {
            self.invulnerable += delta;
        }

        if !self.atk_cooldown() {
            self.atk_cooldown += delta;
        }

        // dead
        if self.stats.hp <= 0f32 {
            self.animation = (Animations::Die, Direction::Down);
        }
        // casting
        else if keyboard::is_key_pressed(ctx, KeyCode::Q) && self.stats.hp > 0f32 && self.stats.mp > 0 {
            self.atk_box = Some(AtkBox::new(ctx, 5.0, self.x, self.y, 25.0, 25.0, &self.direction, 5.0));
            self.animation.0 = Animations::Cast;
            self.stats.mp -= 1;
        }
        // slashing
        else if keyboard::is_key_pressed(ctx, KeyCode::Space) && self.stats.hp > 0f32 { //&& self.atk_cooldown() {
            self.atk_box = Some(AtkBox::new(ctx, 2.0, self.x, self.y, 5.0, 5.0, &self.direction, 5.0));
            self.animation.0 = Animations::Slash;
            self.atk_cooldown = Duration::new(0u64, 0u32);
        }
        // walking animations
        else if keyboard::is_key_pressed(ctx, KEY_RIGHT) {
            self.x += move_increment(ctx);
            self.animation = (Animations::Walking, Direction::Right);
            self.direction = Direction::Right;
        } else if keyboard::is_key_pressed(ctx, KEY_LEFT) {
            self.x -= move_increment(ctx);
            self.animation = (Animations::Walking, Direction::Left);
            self.direction = Direction::Left;
        } else if keyboard::is_key_pressed(ctx, KEY_UP) {
            self.y -= move_increment(ctx);
            self.animation = (Animations::Walking, Direction::Up);
            self.direction = Direction::Up;
        } else if keyboard::is_key_pressed(ctx, KEY_DOWN) {
            self.y += move_increment(ctx);
            self.animation = (Animations::Walking, Direction::Down);
            self.direction = Direction::Down;
        }
        // standing animation
        else {
            self.animation.0 = Animations::Stand;
        }

        self.sprite.get_mut(&self.animation).unwrap().animate(delta);
    }

    // returns if player should be able to take damage
    // player is invulnerable for 1/4
    fn invulnerable(&self) -> bool {
        self.invulnerable < Duration::from_millis(250u64)
    }

    fn atk_cooldown(&self) -> bool {
        self.atk_cooldown > Duration::from_millis(350u64)
    }

    pub fn move_location(&mut self, xinc: f32, yinc: f32) {
        self.x = xinc;
        self.y = yinc;
    }

    pub fn take_dmg(&mut self, dmg_to_take: f32) {
        if !self.invulnerable() {
            self.stats.hp -= dmg_to_take;
            if self.stats.hp < 0f32 {
                self.stats.hp = 0f32;
            }
            self.invulnerable = Duration::new(0u64, 0u32);
            // Check for death and maybe call a death function.
        }
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
        let dp = graphics::DrawParam::default().offset(nalgebra::Point2::new(0.5, 0.5)).dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, self.sprite.get(&self.animation).unwrap(), dp)
    }
}

impl CollideEntity for Player {
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self.sprite.get(&self.animation).unwrap().dimensions().unwrap();
        r.x = self.x - r.w/2.0;
        r.y = self.y - r.h/2.0;
        r
    }
}
