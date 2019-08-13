use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, DrawParam, Rect, Text};
use ggez::input::keyboard;
use ggez::{nalgebra as na, timer, Context, ContextBuilder, GameResult};

struct MyStruct {
    text: Text,
    other: Text,
    other_dest: na::Point2<f32>,
    screen_coord: Rect,
}

impl MyStruct {
    pub fn new(ctx: &mut Context) -> MyStruct {
        let text = Text::new("Hello World");
        let other = Text::new("|o|");
        let other_dest = na::Point2::new(0f32, 0f32);
        let screen_coord = graphics::screen_coordinates(ctx);
        MyStruct {
            text,
            other,
            other_dest,
            screen_coord,
        }
    }
}

impl EventHandler for MyStruct {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // move the screen around
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.screen_coord.x -= 5f32;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.screen_coord.x += 5f32;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.screen_coord.y += 5f32;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.screen_coord.y -= 5f32;
        }

        // move the tie fighter around
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.other_dest.x += 5f32;
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.other_dest.x -= 5f32;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.other_dest.y -= 5f32;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.other_dest.y += 5f32;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        // used to move the window being drawn on
        // does not affect any other thing being drawn
        graphics::set_screen_coordinates(ctx, self.screen_coord)?;

        // orginally I tried to draw text using graphics.draw()...
        // but It would always align to the same place
        // found that using queued text to do drawing fixed issue

        // queue up text to draw
        // https://docs.rs/ggez/0.5.1/ggez/graphics/fn.queue_text.html
        graphics::queue_text(
            ctx,
            &self.text,
            na::Point2::new(400f32, 300f32),
            Some(graphics::BLACK),
        );

        graphics::queue_text(ctx, &self.other, self.other_dest, Some(graphics::BLACK));

        // if any other text drawn using graphics::draw() the queue will be drawn
        // https://docs.rs/ggez/0.5.1/ggez/graphics/fn.draw_queued_text.html
        graphics::draw_queued_text(
            ctx,
            DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("screen_coord", "people")
        .build()
        .unwrap();

    let mut my_struct = MyStruct::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_struct) {
        Ok(_) => println!("Clean loop exit"),
        Err(e) => println!("Error loop exit {}", e),
    };
    println!("Goodbye!");
}
