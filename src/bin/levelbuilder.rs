use ggez::event::EventHandler;
use ggez::input::mouse;
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::*;

use macguffin_quest::entities::DrawableEntity;
use macguffin_quest::entities::environment::level_builder::LevelBuilder;
use macguffin_quest::entities::environment::level::Level;

struct State {
    screen: Rect, // used to move image around screen

    sheet: Image, // sheet to be used for tiles

    level: Level, // level being designed
}

impl State {
    fn new(ctx: &mut Context) -> State {
        // what is the drawable region of the screen
        let (width, height) = graphics::drawable_size(ctx);
        let screen = Rect::new(0f32, 0f32, width, height);

        // tile sprite sheet
        let sheet = Image::new(ctx, "/testwalls.png").unwrap();

        // create basic level to build
        let builder = LevelBuilder::new(ctx, None);
        let level = builder.sample3();

        State {
            screen,
            sheet,
            level,
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //nothing to do
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // set background color
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // set screen coordinates
        graphics::set_screen_coordinates(ctx, self.screen)?;

        // draw level
        self.level.draw(ctx)?;

        // draw mouse placement
        let dp = DrawParam::default()
            .dest(mouse::position(ctx))
            ;
        graphics::draw(ctx, &self.sheet, dp)?;
        
        // display frame
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}


fn main() {
    // create a context to access hardware (also creates event loop)
    let (ref mut ctx, ref mut event_loop) =
        ggez::ContextBuilder::new("macguffin_quest", "James M. & William O.")
            .add_resource_path(std::path::PathBuf::from("./resources/texture"))
            .add_resource_path(std::path::PathBuf::from("./resources/font"))
            .build()
            .unwrap();

    // initial state to level builder
    let state = &mut State::new(ctx);

    // start game loop
    match ggez::event::run(ctx, event_loop, state) {
        Ok(_) => println!("Exiting Level Builder."),
        Err(e) => println!("Crashing Level Builder! {}", e),
    }
}
