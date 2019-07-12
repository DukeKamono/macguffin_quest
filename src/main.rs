// from examples on
// https://docs.rs/ggez/0.5.0-rc.2/ggez/
// https://docs.rs/ggez/0.5.0-rc.2/ggez/input/keyboard/index.html

use ggez::event::{EventHandler, KeyCode, KeyMods};
//use ggez::{nalgebra as na};
use ggez::input::keyboard;
use ggez::*;
use rand::prelude::*;

mod player;
use crate::player::Player;

mod blob;
use crate::blob::Blob;

struct MainState {
    player: Player,
    blob: Blob,
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Increase or decrease `position_x` by 0.5, or by 5.0 if Shift is held.
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.player.x += 4.5;
            }
            self.player.x += 0.5;
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.player.x -= 4.5;
            }
            self.player.x -= 0.5;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.player.y -= 4.5;
            }
            self.player.y -= 0.5;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
                self.player.y += 4.5;
            }
            self.player.y += 0.5;
        }

        if (self.player.x >= self.blob.x && self.player.x <= self.blob.x + 32.0)
            && (self.player.y >= self.blob.y && self.player.y <= self.blob.y + 32.0)
        {
            let mut rng = thread_rng();
            self.blob.x = rng.gen_range(0, 800) as f32;
            self.blob.y = rng.gen_range(0, 600) as f32;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // This sets the background to BLACK.
        graphics::clear(ctx, graphics::BLACK);

        self.player.draw(ctx);
        self.blob.draw(ctx);

        // This presents the contents of ctx to the game.
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        // Quit if Shift+Ctrl+Q is pressed.
        if let KeyCode::Q = key {
            if mods.contains(KeyMods::SHIFT & KeyMods::CTRL) {
                println!("Terminating!");
                ggez::quit(ctx);
            } else if mods.contains(KeyMods::SHIFT) || mods.contains(KeyMods::CTRL) {
                println!("You need to hold both Shift and Control to quit.");
            } else {
                println!("Now you're not even trying!");
            }
        }
    }
}

fn main() {
    // create a context to access hardware (also creates event loop)
    let c = ggez::conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) =
        ggez::ContextBuilder::new("macguffin_quest", "James M. & William O.")
            .add_resource_path(std::path::PathBuf::from("./resources/texture"))
            .add_resource_path(std::path::PathBuf::from("./resources/font"))
            .conf(c)
            .build()
            .unwrap();

    // create an instance of game state
    let state = &mut MainState {
        player: Player::new(ctx),
        blob: Blob::new(ctx),
    };

    // start game loop
    match ggez::event::run(ctx, event_loop, state) {
        Ok(_) => println!("Exiting Game."),
        Err(e) => println!("Run event loop broke! {}", e),
    }
}
