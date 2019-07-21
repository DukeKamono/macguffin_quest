use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::*;

mod player;
use player::Player;

mod blob;
use blob::Blob;

mod wall;
use wall::Wall;

struct MainState {
    player: Player,
    blob: Blob,
    walls: Vec<Wall>,
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player.update(ctx);

        if self.blob.collide(&self.player) {
            self.blob.relocate();
        }

        for wall in &self.walls {
            if self.player.collide(wall) {
                self.player.move_location(-0.5, -0.5);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for wall in &self.walls {
            wall.draw(ctx);
        }

        self.player.draw(ctx);
        self.blob.draw(ctx);

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
            _ => { /* Do Nothing */ },
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
    let win_width = ctx.conf.window_mode.width;
    let win_height = ctx.conf.window_mode.height;

    let mut v = Vec::new();
    v.push(Wall::new(ctx, 350.0, 150.0));
    v.push(Wall::new(ctx, 350.0, 250.0));
    v.push(Wall::new(ctx, 350.0, 350.0));

    let state = &mut MainState {
        player: Player::new(ctx),
        blob: Blob::new(ctx, graphics::Rect::new(0f32, 0f32, win_width, win_height)),
        walls: v,
    };

    // start game loop
    match ggez::event::run(ctx, event_loop, state) {
        Ok(_) => println!("Exiting Game."),
        Err(e) => println!("Run event loop broke! {}", e),
    }
}
