use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::*;

// contains all the information on entities
mod entities;
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
                    println!("blob took dmg");
                    blob.take_dmg(self.player.atk);
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
            KeyCode::Escape => quit(ctx),
            // other keys to detect
            _ => { /* Do Nothing */ }
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

    // create player
    let mut player = Player::new(ctx);
    player.move_location(150f32, 150f32);
	let hp = player.hp;

    // create blobs (ie enemies)
    let mut blob = Vec::new();
    blob.push(Blob::new(ctx, 250.0, 250.0));
    blob.push(Blob::new(ctx, 250.0, 350.0));
    blob.push(Blob::new(ctx, 250.0, 150.0));

    // build level
    let img = graphics::Image::new(ctx, "/testwalls.png").unwrap();
    let mut lb = LevelBuilder::new(ctx, None);
    lb.set_tile_image(
        0usize,
        &Sprite::new(&img, graphics::Rect::new(0f32, 0f32, 64f32, 64f32)).unwrap(),
    )
    .unwrap();
    lb.set_tile_image(
        1usize,
        &Sprite::new(&img, graphics::Rect::new(64f32, 0f32, 64f32, 64f32)).unwrap(),
    )
    .unwrap();
    lb.set_tile_image(
        2usize,
        &Sprite::new(&img, graphics::Rect::new(128f32, 0f32, 64f32, 64f32)).unwrap(),
    )
    .unwrap();
    lb.set_tile_image(
        3usize,
        &Sprite::new(&img, graphics::Rect::new(192f32, 0f32, 64f32, 64f32)).unwrap(),
    )
    .unwrap();
    let level = lb.sample3();

    // demo sprites
    let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet.png").unwrap();
    let sprite = Sprite::new(&img, graphics::Rect::new(0f32, 128f32, 64f32, 64f32)).unwrap();
    let animated = AnimatedBuilder::new(&img)
        .create_animated(graphics::Rect::new(0f32, 320f32, 64f32, 64f32), 6usize)
        .unwrap();

    // create state
    let state = &mut MainState {
        level,
        blob,
        player,
		ui: UI::new(ctx, "Adventurer".to_string(), hp),
        sprite,
        animated,
        rotation: 0f32,
    };

    // start game loop
    match ggez::event::run(ctx, event_loop, state) {
        Ok(_) => println!("Exiting Game."),
        Err(e) => println!("Run event loop broke! {}", e),
    }
}
