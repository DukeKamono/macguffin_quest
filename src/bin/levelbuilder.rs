use ggez::event::{EventHandler, KeyCode};
use ggez::input::{keyboard, mouse};
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::*;

use macguffin_quest::entities::DrawableEntity;
use macguffin_quest::entities::environment::level_builder::LevelBuilder;
use macguffin_quest::entities::environment::level::Level;

struct State {
    screen: Rect, // used to move image around screen

    mouse_position: mint::Point2<f32>, // adjusted position of the mouse

    sheet: Image, // sheet to be used for tiles

    level: Level, // level being designed
}

impl State {
    fn new(ctx: &mut Context) -> State {
        // what is the drawable region of the screen
        let (width, height) = graphics::drawable_size(ctx);
        let screen = Rect::new(0f32, 0f32, width, height);

        // get position of mouse
        let mouse_position = mouse::position(ctx);

        // tile sprite sheet
        let sheet = Image::new(ctx, "/testwalls.png").unwrap();

        // create basic level to build
        let builder = LevelBuilder::new(ctx, None);
        let level = builder.sample3();

        State {
            screen,
            mouse_position,
            sheet,
            level,
        }
    }
}

impl EventHandler for State {
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.mouse_position.x = f32::floor((x + self.screen.x) / 64f32) * 64f32;
        self.mouse_position.y = f32::floor((y + self.screen.y) / 64f32) * 64f32;
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: mouse::MouseButton, _x: f32, _y: f32) {
        match button {
            mouse::MouseButton::Left => println!("left click"),
            mouse::MouseButton::Right => println!("right click"),
            _ => println!("other mouse click"),
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        println!("mouse wheel x{} y{}", x, y);
    }
    
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // move screen
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.screen.x += 4f32;
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.screen.x -= 4f32;
        } else if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.screen.y -= 4f32;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.screen.y += 4f32;
        }

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
            .dest(self.mouse_position)
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
