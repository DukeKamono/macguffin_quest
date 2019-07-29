use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::*;

// contains all the information on entities
mod Super::entities;
// get collison trait from entities
use entities::{CollideEntity, DrawableEntity};
// get player struct to use
use entities::player::player::Player;
// get blob struct to use
use entities::enemies::blob::Blob;
// get wall struct to use
use entities::environment::{level::Level, level_builder::LevelBuilder};

mod ui;
use ui::UI;

mod sprites;
//use sprites::sprite::Sprite;
//use sprites::animated_sprite::*;
use sprites::*;

struct StateMachine {
	MainMenuState: MainMenuState,
	MainState: MainState,
}

struct MainMenuState {
	ui: UI,
}

impl EventHandler for MainMenuState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
		Ok(())
	}
	
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
		Ok(())
	}
	
	fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) {
        match key {
            KeyCode::P => println!("Pause? Maybe latter."),
            KeyCode::Escape => ggez::event::quit(ctx),
            // other keys to detect
            _ => { /* Do Nothing */ }
        }
    }
}

struct MainState {
	ui: UI,
    player: Player,
    blob: Vec<Blob>,
    level: Level,
    sprite: Sprite,
    animated: AnimatedSprite,
    rotation: f32,
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let playerx = self.player.x;
        let playery = self.player.y;

        self.player.update(ctx);

        if self.player.collision(&self.level) {
            self.player.move_location(playerx, playery);
        }

        for blob in &mut self.blob {
            if blob.collision(&self.player) {
                self.player.take_dmg(blob.atk);
            }

            if let Some(atk) = &self.player.atk_box {
                if blob.collision(atk) {
                    blob.take_dmg(self.player.atk);
					self.ui.update_dmg_text(ctx, blob.x, blob.y, self.player.atk);
                }
            }
        }

        self.animated.animate(timer::delta(ctx));

        self.rotation += timer::duration_to_f64(timer::delta(ctx)) as f32;
        self.rotation %= 2.0 * std::f32::consts::PI;

		// Should prob make UI update last all the time.
		self.ui.update(ctx, self.player.hp);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.level.draw(ctx)?;

        self.player.draw(ctx)?;

        for blob in &self.blob {
            blob.draw(ctx)?;
        }

        let dp = graphics::DrawParam::default()
            .src(graphics::Rect::new(0.0, 0.0, 1.0, 1.0))
            .dest([736f32, 536f32])
            .offset([0.5, 0.5])
            .scale([2.0, 2.0])
            .rotation(self.rotation)
            .color(graphics::Color::new(
                1.0 - self.rotation / (2.0 * std::f32::consts::PI),
                self.rotation / (2.0 * std::f32::consts::PI),
                self.rotation / (2.0 * std::f32::consts::PI),
                1.0,
            ));
        graphics::draw(ctx, &self.sprite, dp)?;

        let dp = graphics::DrawParam::default()
            .dest([736f32, 64f32])
            .offset([0.5, 0.5])
            .scale([2.0, 2.0])
            .rotation(self.rotation);
        graphics::draw(ctx, &self.animated, dp)?;
		
		self.ui.draw(ctx);

        // This presents the contents of ctx to the game.
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) {
        match key {
            KeyCode::P => println!("Pause? Maybe latter."),
            KeyCode::Escape => ggez::event::quit(ctx),
            // other keys to detect
            _ => { /* Do Nothing */ }
        }
    }
}