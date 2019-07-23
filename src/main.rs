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
use entities::environment::wall::Wall;

mod sprites;
use sprites::sprite::Sprite;

struct MainState {
    player: Player,
    blob: Blob,
    walls: Vec<Wall>,
    sprite: Sprite,
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

        for wall in &self.walls {
            if self.player.collision(wall) {
                self.player.move_location(playerx, playery);
            }
        }

        self.rotation += timer::duration_to_f64(timer::delta(ctx)) as f32;
        self.rotation = self.rotation % (2.0 * std::f32::consts::PI);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for wall in &self.walls {
            wall.draw(ctx)?;
        }

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
        graphics::draw(ctx, &self.sprite, dp.clone())?;

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

    let mut wall_vec = Vec::new();
    wall_vec.push(Wall::new(ctx, 350.0, 150.0));
    wall_vec.push(Wall::new(ctx, 350.0, 250.0));
    wall_vec.push(Wall::new(ctx, 350.0, 350.0));

    let img = graphics::Image::new(ctx, "/dapper-skeleton-sheet.png").unwrap();
    let state = &mut MainState {
        player: Player::new(ctx),
        blob: Blob::new(ctx),
        walls: wall_vec,
        sprite: Sprite::new(&img, graphics::Rect::new(0f32, 128f32, 64f32, 64f32)).unwrap(),
        rotation: 0f32,
    };

    // start game loop
    match ggez::event::run(ctx, event_loop, state) {
        Ok(_) => println!("Exiting Game."),
        Err(e) => println!("Run event loop broke! {}", e),
    }
}
