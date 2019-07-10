use ggez::*;

struct State {
    image: graphics::Image,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let image = graphics::Image::new(ctx, "/pong_spritesheet.png")?;
        Ok(State{ image })
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let draw_param = graphics::DrawParam::default()
            .dest([50.0, 50.0]);
        graphics::draw(ctx, &self.image, draw_param)?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    // create context
    let (ctx, event_loop) = &mut ContextBuilder::new("display image", "people")
        .add_resource_path(
            std::path::PathBuf::from("./src/resources/texture")
        )
        .build()
        .unwrap();
    // create game state
    let state = &mut State::new(ctx).unwrap();
    // run loop
    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Loop Exit"),
        Err(e) => println!("Loop Error: {}", e),
    };
    println!("Goodbye!");
}