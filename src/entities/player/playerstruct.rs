use super::super::{Animations, CollideEntity, Direction, DrawableEntity};
use super::atk_box::AtkBox;
use super::stats::Stats;
use crate::sprites::*;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{Image, Rect};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::*;
use std::collections::HashMap;
use std::time::Duration;

use crate::ui::FloatingText;

/// Constant values for keys used to determine movement
const KEY_UP: KeyCode = KeyCode::W;
const KEY_DOWN: KeyCode = KeyCode::S;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_LEFT: KeyCode = KeyCode::A;

/// The player struct
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
    pub macguffin: bool,
    pub cooldown: Duration,
    floating_text: Vec<FloatingText>,
}

/// Functions for the Player
impl Player {
    /// News up a new player struct.
    pub fn new(ctx: &mut Context, chosen_player: String) -> Player {
        let mut sprite = HashMap::new();
        let sheet = Image::new(ctx, chosen_player).unwrap();
        let builder = AnimatedBuilder::new(&sheet);
        // standing
        sprite.insert(
            (Animations::Stand, Direction::Up),
            builder
                .create_animated(Rect::new(0f32, 0f32, 64f32, 64f32), 1usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Stand, Direction::Left),
            builder
                .create_animated(Rect::new(0f32, 64f32, 64f32, 64f32), 1usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Stand, Direction::Down),
            builder
                .create_animated(Rect::new(0f32, 128f32, 64f32, 64f32), 1usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Stand, Direction::Right),
            builder
                .create_animated(Rect::new(0f32, 192f32, 64f32, 64f32), 1usize)
                .unwrap(),
        );
        // walking
        sprite.insert(
            (Animations::Walking, Direction::Up),
            builder
                .create_animated(Rect::new(64f32, 0f32, 64f32, 64f32), 8usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Walking, Direction::Left),
            builder
                .create_animated(Rect::new(64f32, 64f32, 64f32, 64f32), 8usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Walking, Direction::Down),
            builder
                .create_animated(Rect::new(64f32, 128f32, 64f32, 64f32), 8usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Walking, Direction::Right),
            builder
                .create_animated(Rect::new(64f32, 192f32, 64f32, 64f32), 8usize)
                .unwrap(),
        );
        // casting
        sprite.insert(
            (Animations::Cast, Direction::Up),
            builder
                .create_animated(Rect::new(64f32, 512f32, 64f32, 64f32), 6usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Cast, Direction::Left),
            builder
                .create_animated(Rect::new(64f32, 576f32, 64f32, 64f32), 6usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Cast, Direction::Down),
            builder
                .create_animated(Rect::new(64f32, 640f32, 64f32, 64f32), 6usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Cast, Direction::Right),
            builder
                .create_animated(Rect::new(64f32, 704f32, 64f32, 64f32), 6usize)
                .unwrap(),
        );
        // slashing
        sprite.insert(
            (Animations::Slash, Direction::Up),
            builder
                .create_animated(Rect::new(64f32, 256f32, 64f32, 64f32), 6usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Slash, Direction::Left),
            builder
                .create_animated(Rect::new(64f32, 320f32, 64f32, 64f32), 6usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Slash, Direction::Down),
            builder
                .create_animated(Rect::new(64f32, 384f32, 64f32, 64f32), 6usize)
                .unwrap(),
        );
        sprite.insert(
            (Animations::Slash, Direction::Right),
            builder
                .create_animated(Rect::new(64f32, 448f32, 64f32, 64f32), 6usize)
                .unwrap(),
        );
        // die
        sprite.insert(
            (Animations::Die, Direction::Down),
            builder
                .create_animated_once(Rect::new(64f32, 768f32, 64f32, 64f32), 5usize)
                .unwrap(),
        );

        let floating_text = Vec::new();

        Player {
            x: 10.0,
            y: 10.0,
            stats: Stats::new(1, 0, 50.0, 150, 3.0, 0.5, 1.0),
            sprite,
            animation: (Animations::Walking, Direction::Right),
            atk_box: None,
            attacking: false,
            atk_cooldown: Duration::new(0u64, 0u32),
            invulnerable: Duration::new(0u64, 0u32),
            direction: Direction::Right, // Starting direction?
            macguffin: false,
            cooldown: Duration::new(1u64, 0u32),
            floating_text,
        }
    }

    /// Increase or decrease `position_x` by 2.5, or by 5.0 if Shift is held.
    /// Checks for floating text, and animation and attacking.
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
        else if keyboard::is_key_pressed(ctx, KeyCode::Q)
            && self.stats.hp > 0f32
            && self.stats.mp > 0
        {
            self.atk_box = Some(AtkBox::new(
                ctx,
                self.x,
                self.y,
                64.0,
                100.0,
                &self.direction,
                40.0,
            ));
            self.animation.0 = Animations::Cast;
            self.stats.mp -= 1;
        }
        // slashing
        else if keyboard::is_key_pressed(ctx, KeyCode::Space) && self.stats.hp > 0f32 {
            //&& self.atk_cooldown() {
            self.atk_box = Some(AtkBox::new(
                ctx,
                self.x,
                self.y,
                32.0,
                80.0,
                &self.direction,
                40.0,
            ));
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

        self.floating_text.retain(|t| t.live());
        self.floating_text.iter_mut().for_each(|t| t.update(delta));

        // cooldown
        if self.pick_up_cooldown() {
            self.cooldown += delta;
        }
    }

    /// Returns if player should be able to take damage
    fn invulnerable(&self) -> bool {
        self.invulnerable < Duration::from_millis(250u64)
    }

    /// This is still in the works
    fn atk_cooldown(&self) -> bool {
        self.atk_cooldown > Duration::from_millis(350u64)
    }

    /// Player pick_up item text.
    pub fn pick_up(&mut self, ctx: &mut Context, text: String) {
        if !self.pick_up_cooldown() {
            self.cooldown = Duration::new(0u64, 0u32);
            self.floating_text
                .push(FloatingText::new(ctx, self.x, self.y, text, "Green"));
        }
    }

    /// Checks if the pickup text cooldown is over
    fn pick_up_cooldown(&self) -> bool {
        self.cooldown < Duration::from_millis(250u64)
    }

    /// Moves the player to a new location.
    pub fn move_location(&mut self, xinc: f32, yinc: f32) {
        self.x = xinc;
        self.y = yinc;
    }

    /// When the player takes damage, check how much they can and if they can.
    pub fn take_dmg(&mut self, ctx: &mut Context, dmg_to_take: f32) {
        let true_dmg = dmg_to_take - self.stats.def;
        if !self.invulnerable() {
            if true_dmg > 0.0 {
                self.stats.hp -= true_dmg;
                if self.stats.hp < 0f32 {
                    self.stats.hp = 0f32;
                }
                self.invulnerable = Duration::new(0u64, 0u32);
                self.floating_text.push(FloatingText::new(
                    ctx,
                    self.x,
                    self.y,
                    true_dmg.to_string(),
                    "Red",
                ));
            } else {
                self.floating_text.push(FloatingText::new(
                    ctx,
                    self.x,
                    self.y,
                    "Blocked".to_string(),
                    "Blue",
                ));
            }
        }
    }

    /// Calls the atk_box struct draw function
    pub fn draw_weapon(&self, ctx: &mut Context) {
        if let Some(atk) = &self.atk_box {
            atk.draw(ctx).expect("Failed to draw attack.");
        }
    }
}

/// Draw trait for the Player
impl DrawableEntity for Player {
    /// Draws the weapon, the player, and the floating text.
    fn draw(&self, ctx: &mut Context) -> GameResult {
        self.draw_weapon(ctx);
        let dp = graphics::DrawParam::default()
            .offset(nalgebra::Point2::new(0.5, 0.5))
            .dest(na::Point2::new(self.x, self.y));
        graphics::draw(ctx, self.sprite.get(&self.animation).unwrap(), dp)?;

        self.floating_text.iter().for_each(|t| t.draw(ctx));

        Ok(())
    }
}

/// Collide trait for the Player
impl CollideEntity for Player {
    /// Get the hitbox for the Player
    fn get_hitbox(&self) -> graphics::Rect {
        let mut r = self
            .sprite
            .get(&self.animation)
            .unwrap()
            .dimensions()
            .unwrap();
        r.x = self.x - r.w / 2.0;
        r.y = self.y - r.h / 2.0;
        r
    }
}
