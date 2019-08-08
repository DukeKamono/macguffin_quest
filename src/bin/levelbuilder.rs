use std::collections::HashMap;
use std::io::{BufRead, BufReader};

use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::input::{keyboard, mouse};
use ggez::*;

use macguffin_quest::entities::DrawableEntity;
use macguffin_quest::entities::environment::level_builder::LevelBuilder;
use macguffin_quest::entities::environment::level::Level;
use macguffin_quest::sprites::Sprite;

struct State {
    screen: Rect, // used to move image around screen

    mouse_position: mint::Point2<f32>, // adjusted position of the mouse

    tile_value: usize, // value of currently selected tile type

    builder: LevelBuilder, // used to build levels
    level: Level, // level being designed

    map_tiles: HashMap<(i64, i64), usize>, // generate level from
    vector_types: Vec<Sprite>, // various tile images (after sheet is split up)
}

impl State {
    fn new(ctx: &mut Context, sheet: &Image) -> State {
        // what is the drawable region of the screen
        let (width, height) = graphics::drawable_size(ctx);
        let screen = Rect::new(0f32, 0f32, width, height);

        // get position of mouse
        let mouse_position = mouse::position(ctx);

        // initially selected tile
        let tile_value = 0usize;

        // create basic level to build
        let mut builder = LevelBuilder::new(ctx, None);
        let level = builder.sample0();

        // new level tile information
        let map_tiles = HashMap::new();
        let vector_types = State::tileize(&mut builder, sheet);

        State {
            screen,
            mouse_position,
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
        ret_value
    }
}

impl EventHandler for State {
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.mouse_position.x = f32::floor((x + self.screen.x) / 64f32) * 64f32;
        self.mouse_position.y = f32::floor((y + self.screen.y) / 64f32) * 64f32;
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

        if mouse::button_pressed(ctx, mouse::MouseButton::Left) {
            // update tile
            self.map_tiles.insert((self.mouse_position.x as i64, self.mouse_position.y as i64), self.tile_value);
            // build level with new tile
            self.level = buildlevel(&mut self.builder, &mut self.map_tiles);
        } else if mouse::button_pressed(ctx, mouse::MouseButton::Right) {
            // update tile
            self.map_tiles.remove(&(self.mouse_position.x as i64, self.mouse_position.y as i64));
            // build level with new tile
            self.level = buildlevel(&mut self.builder, &mut self.map_tiles);
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
        
        // display frame
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn readfile(ctx: &mut Context, path: String) -> HashMap<(i64, i64), usize> {
    let retvalue = HashMap::new();

    let file = ggez::filesystem::open(ctx, path).unwrap();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut parse = line.split_whitespace();
        let message = format!("Error on line {}, unable to parse valid value", i + 1);
        let x = parse.next().unwrap().parse::<f32>().expect(&message);
        let y = parse.next().unwrap().parse::<f32>().expect(&message);
        let t = parse.next().unwrap().parse::<usize>().expect(&message);
        println!("{}, ({},{}), {}", i, x, y, t);
    }

    retvalue
}

//fn writefile() {}

fn buildlevel(builder: &mut LevelBuilder, map_tiles: &mut HashMap<(i64, i64), usize>) -> Level {
    builder.generate_level(
        map_tiles.iter()
            .map(|(k, v)| ((k.0 as f32, k.1 as f32), *v))
            .collect()
    )
}

fn main() {
    // view arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1usize {
        println!("{}", args[1]);
    }

    // create a context to access hardware (also creates event loop)
    let (ref mut ctx, ref mut event_loop) =
        ggez::ContextBuilder::new("macguffin_quest", "James M. & William O.")
            .add_resource_path(std::path::PathBuf::from("./resources/texture"))
            .add_resource_path(std::path::PathBuf::from("./resources/font"))
            .add_resource_path(std::path::PathBuf::from("./resources/level"))
            .build()
            .unwrap();

    // initial state to level builder
    let sheet = Image::new(ctx, "/testwalls.png").unwrap();
    let tiles = readfile(ctx, "/testing.lvl".to_string());
    println!("{:?}", tiles);
    let state = &mut State::new(ctx, &sheet);

    // start game loop
    match ggez::event::run(ctx, event_loop, state) {
        Ok(_) => println!("Exiting Level Builder."),
        Err(e) => println!("Crashing Level Builder! {}", e),
    }
}
