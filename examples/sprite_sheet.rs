use ggez::*;

struct State {
    // fields
}
impl State {
    fn new(ctx: &mut Context) -> State {
        State {
        }
    }
}
impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //update
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        // draw
        
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    // create context
    let (ctx, event_loop) = &mut ContextBuilder::new("collisions", "people")
        .window_setup(conf::WindowSetup::default().title("Collision Detection"))
        .build()
        .unwrap();
    // create state and game loop
    let state = &mut State::new(ctx);
    // run loop
    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Clean loop exit"),
        Err(e) => println!("Error loop exit {}", e),
    };
    println!("Goodbye!");
}