// from examples on
// https://docs.rs/ggez/0.5.0-rc.2/ggez/
// https://docs.rs/ggez/0.5.0-rc.2/ggez/input/keyboard/index.html


use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::{graphics, nalgebra as na, timer};
use ggez::input::keyboard;
use ggez::*;

mod player;
use crate::player::Player;

struct MainState {
	player: Player
}

impl EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		// Increase or decrease `position_x` by 0.5, or by 5.0 if Shift is held.
		if keyboard::is_key_pressed(ctx, KeyCode::Right) {
			if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
				self.player.x += 4.5;
			}
			self.player.x += 0.5;
		} else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
			if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
				self.player.x -= 4.5;
			}
			self.player.x -= 0.5;
		}
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
		// Create a circle at `position_x` and draw
		let circle = graphics::Mesh::new_circle(
			ctx,
			graphics::DrawMode::fill(),
			na::Point2::new(self.player.x, 380.0),
			100.0,
			2.0,
			graphics::WHITE,
		)?;
		graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
		graphics::present(ctx)?;
		timer::yield_now();
		Ok(())
	}

	fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
		match key {
			// Quit if Shift+Ctrl+Q is pressed.
			KeyCode::Q => {
				if mods.contains(KeyMods::SHIFT & KeyMods::CTRL) {
					println!("Terminating!");
					ggez::quit(ctx);
				} else if mods.contains(KeyMods::SHIFT) || mods.contains(KeyMods::CTRL) {
					println!("You need to hold both Shift and Control to quit.");
				} else {
					println!("Now you're not even trying!");
				}
			}
			_ => (),
		}
	}
}

fn main() {
	// create a context to access hardware (also creates event loop)
	let c = ggez::conf::Conf::new();
	let (ref mut ctx, ref mut event_loop) = ggez::ContextBuilder::new("rust_game", "James M. & William O.")
		.conf(c)
		.build()
		.unwrap();
	
	// create an instance of game state
	let state = &mut MainState { player: Player::new(ctx) };

	// start game loop
	match ggez::event::run(ctx, event_loop, state) {
		Ok(_) => println!("Exiting Game."),
		Err(e) => println!("Run event loop broke! {}", e),
	}
}