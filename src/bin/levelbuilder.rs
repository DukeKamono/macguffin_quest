use std::collections::HashMap;

use ggez::event::{EventHandler, KeyCode};
use ggez::input::{keyboard, mouse};
use ggez::graphics::{DrawParam, Image, Mesh, Rect};
use ggez::*;

use macguffin_quest::entities::DrawableEntity;
use macguffin_quest::entities::environment::level_builder::LevelBuilder;
use macguffin_quest::entities::environment::level::Level;
use macguffin_quest::sprites::Sprite;

struct State {
    screen: Rect, // used to move image around screen

    mouse_position: mint::Point2<f32>, // adjusted position of the mouse
    click_start: Option<mint::Point2<f32>>, // start of click

    tile_value: usize, // value of currently selected tile type

    builder: LevelBuilder, // used to build levels
    level: Level, // level being designed

    map_tiles: HashMap<(i64, i64), usize>, // generate level from
    vector_types: Vec<Sprite>, // various tile images (after sheet is split up)
}

impl State {
    fn new(ctx: &mut Context) -> State {
        // what is the drawable region of the screen
        let (width, height) = graphics::drawable_size(ctx);
        let screen = Rect::new(0f32, 0f32, width, height);

        // get position of mouse
        let mouse_position = mouse::position(ctx);
        let click_start = None;

        // initially selected tile
        let tile_value = 0usize;

        // tile sprite sheet
        let sheet = Image::new(ctx, "/testwalls.png").unwrap();

        // create basic level to build
        let mut builder = LevelBuilder::new(ctx, None);
        let level = builder.sample0();

        // new level tile information
        let map_tiles = HashMap::new();
        let vector_types = State::tileize(&mut builder, & sheet);

        State {
            screen,
            mouse_position,
            click_start,
            tile_value,
            builder,
            level,
            map_tiles,
            vector_types,
        }
    }

    fn tileize(build: &mut LevelBuilder, img: &Image) -> Vec<Sprite> {
        let mut ret_value = Vec::new();

        let width = f32::floor(img.width() as f32 / 64f32); // max
        let height = f32::floor(img.height() as f32 / 64f32); // max

        //println!("{} {}", width, height);

        // do the tiling
        let mut h = 0f32; // counting
        while h < height {
            let mut w = 0f32; // counting
            while w < width {
                ret_value.push(Sprite::new(img, Rect::new(w * 64f32, h * 64f32, 64f32, 64f32)).unwrap());
                build.set_tile_image(
                    ret_value.len() - 1usize,
                    ret_value.last().unwrap(),
                ).unwrap();
                w += 1f32;
            }
            h += 1f32;
        }
        //println!("{:?}", ret_value);
        //println!("{:?}", ret_value.len());
        ret_value
    }
}

impl EventHandler for State {
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.mouse_position.x = f32::floor((x + self.screen.x) / 64f32) * 64f32;
        self.mouse_position.y = f32::floor((y + self.screen.y) / 64f32) * 64f32;
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: mouse::MouseButton, x: f32, y: f32) {
        match button {
            mouse::MouseButton::Left => println!("left click"),
            mouse::MouseButton::Right => println!("right click"),
            _ => println!("other mouse click"),
        }
        if button == mouse::MouseButton::Left {
            self.click_start = Some([x, y].into());
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: mouse::MouseButton, _x: f32, _y: f32) {
        match button {
            mouse::MouseButton::Left => println!("left released"),
            mouse::MouseButton::Right => println!("right released"),
            _ => println!("other mouse released"),
        }
        if button == mouse::MouseButton::Left {
            // add new tiles
            let mut point = self.click_start.unwrap();
            point.x = f32::floor((point.x + self.screen.x) / 64f32) * 64f32;
            point.y = f32::floor((point.y + self.screen.y) / 64f32) * 64f32;
            self.map_tiles.insert((point.x as i64, point.y as i64), self.tile_value);

            // build level with new tiles
            self.level = self.builder.generate_level(
                self.map_tiles.iter()
                    .map(|(k, v)| ((k.0 as f32, k.1 as f32), *v))
                    .collect()
            );
            
            // no longer tracking line (hope!)
            self.click_start = None;
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        //println!("mouse wheel x{} y{}", x, y);
        if y > 0f32 && self.tile_value < self.vector_types.len() - 1usize {
            self.tile_value += 1usize;
        } else if y < 0f32 && self.tile_value > usize::min_value() {
            self.tile_value -= 1usize;
        }
        //println!("{}", self.tile_value);
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
        graphics::draw(ctx, &self.vector_types[self.tile_value], dp)?;

        if let Some(point) = self.click_start {
            let mut mouse_position = mouse::position(ctx);
            mouse_position.x += self.screen.x;
            mouse_position.y += self.screen.y;
            if mouse_position == point {
                mouse_position.x += 1f32;
                mouse_position.y += 1f32;
            }
            let line = Mesh::new_line(ctx, &[point, mouse_position], 8f32, graphics::WHITE)?;
            graphics::draw(ctx, &line, DrawParam::default())?;
        }
        
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
