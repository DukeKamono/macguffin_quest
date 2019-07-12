use ggez::*;

struct State {
    text: graphics::Text,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        // would need ctx to load new font if not using default
        let font = graphics::Font::new(ctx, "/square.ttf")?;
        let hello = "hello".to_string();
        let text = graphics::Text::new((hello, font, 22.0));
        Ok(State { text })
    }
}

impl event::EventHandler for State {
    // game loop to update logic... should do something...
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    // draw things to screen
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // clear screen
        graphics::clear(ctx, graphics::BLACK);

        //// draw text in state at a certian point
        //let point2 = nalgebra::Point2::new(250.0, 5.0);
        //graphics::draw(ctx, &self.text, (point2, 0.0, graphics::WHITE))?;

        // draw text in state
        graphics::draw(ctx, &self.text, graphics::DrawParam::default())?;

        // display to screen
        graphics::present(ctx)?;
        // try to not burn out cpu...
        timer::yield_now();
        Ok(())
    }

    fn text_input_event(&mut self, ctx: &mut Context, character: char) {
        let font = graphics::Font::new(ctx, "/square.ttf").expect("can't find font");
        // should do some checking here on character...

        //if character == "".chars().next().unwrap() {
        //	self.text = graphics::Text::new(("", font, 22.0));
        //}

        if character == "\u{8}".chars().next().unwrap() && self.text.contents().is_empty() {
            println!("{:?} All gone!", character);
        } else if character == "\u{8}".chars().next().unwrap() && !self.text.contents().is_empty() {
            let t: String = self
                .text
                .contents()
                .drain(..self.text.contents().len() - 1)
                .collect();
            self.text = graphics::Text::new((t, font, 22.0));

            println!("{:?} {:?}", character, self.text.contents());
        } else {
            // pretty sure that unicode UTF-8 may be a problem here
            self.text
                .add(character)
                .set_font(font, graphics::Scale { x: 22.0, y: 22.0 });
            // for sanity see what is happening in the console
            println!("{:?} {:?}", character, self.text.contents());
        }
    }
}

fn main() {
    // create context
    let (ctx, event_loop) = &mut ContextBuilder::new("display_user_text", "people")
        //https://docs.rs/ggez/0.5.0-rc.2/ggez/conf/struct.WindowSetup.html
        .window_setup(conf::WindowSetup::default().title("Neat Title"))
        //https://docs.rs/ggez/0.5.0-rc.2/ggez/conf/struct.WindowMode.html
        .window_mode(
            conf::WindowMode::default()
                .dimensions(800.0, 600.0)
                .resizable(false),
        )
        .add_resource_path(std::path::PathBuf::from("./resources/font"))
        .add_resource_path(std::path::PathBuf::from("./resources/texture"))
        .build()
        .unwrap();
    // create state and game loop
    let state = &mut State::new(ctx).unwrap();
    // run loop
    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Clean loop exit"),
        Err(e) => println!("Error loop exit {}", e),
    };

    println!("Goodbye!");
}
