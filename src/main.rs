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
use entities::environment::{level::Level, level_builder::LevelBuilder, wall::Wall};

mod sprites;
//use sprites::sprite::Sprite;
//use sprites::animated_sprite::*;
use sprites::*;

struct MainState {
    player: Player,
    blob: Blob,
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

        if self.blob.collision(&self.player) {
            self.player.take_dmg(self.blob.atk);
        }

        if self.player.collision(&self.level) {
            self.player.move_location(playerx, playery);
        }

        self.animated.animate(timer::delta(ctx));

        self.rotation += timer::duration_to_f64(timer::delta(ctx)) as f32;
        self.rotation = self.rotation % (2.0 * std::f32::consts::PI);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.level.draw(ctx)?;

        self.player.draw(ctx)?;
        self.blob.draw(ctx)?;

        let dp = graphics::DrawParam::default()
            .src(graphics::Rect::new(0.0, 0.0, 1.0, 1.0))
            .dest([736f32, 536f32])
            .offset([0.5,0.5])
            .scale([2.0,2.0])
            .rotation(self.rotation)
            .color(graphics::Color::new(
                1.0 - self.rotation / (2.0 * std::f32::consts::PI),
                self.rotation / (2.0 * std::f32::consts::PI),
                self.rotation / (2.0 * std::f32::consts::PI),
                1.0,
            ))
            ;
        graphics::draw(ctx, &self.sprite, dp)?;

        let dp = graphics::DrawParam::default()
            .dest([736f32, 64f32])
            .offset([0.5,0.5])
            .scale([2.0, 2.0])
            .rotation(self.rotation)
            ;
        graphics::draw(ctx, &self.animated, dp)?;


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

    let mut player = Player::new(ctx);
    player.move_location(150f32, 150f32);

    //let level = LevelBuilder::sample1(ctx);
    let level = LevelBuilder::sample2(ctx);

    let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet.png").unwrap();
    let sprite = Sprite::new(&img, graphics::Rect::new(0f32, 128f32, 64f32, 64f32)).unwrap();
    let animated = AnimatedBuilder::new(&img)
        .create_animated(graphics::Rect::new(0f32, 320f32, 64f32, 64f32), 6usize)
        .unwrap();

    let state = &mut MainState {
        player,
        blob: Blob::new(ctx),
        level,
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
