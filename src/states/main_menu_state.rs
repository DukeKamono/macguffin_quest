// Right now this is a PauseState Clone :(
// I want to add more to this later. (Art, settings section, etc)
pub struct MainMenuState {
    text: graphics::Text,
}

impl CustomEventHandler for MainMenuState {
    fn update(&mut self, _ctx: &mut Context) -> HandlerMessage {
        HandlerMessage::Keep
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, graphics::BLACK);
		let point = nalgebra::Point2::new(350.0, 250.0);
		
		graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam::default().dest(point),
        )
        .expect("ERROR drawing Paused Text");
		
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
    
    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) -> HandlerMessage {
        match key {
            KeyCode::Return => {
                let state = Box::new(MainState::new(ctx));
                HandlerMessage::Spawn(state)
            },
            _ => HandlerMessage::Keep
        }
    }
}

impl MainMenuState {
    pub fn new(ctx: &mut Context) -> MainMenuState {
		let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
		let t = graphics::Text::new(("Macguffin Quest\nPress Enter to Start".to_string(), font, 22.0));
        MainMenuState {
			text: t,
        }
    }
}