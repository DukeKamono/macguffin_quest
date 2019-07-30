pub struct PauseState {
    text: graphics::Text,
}

impl CustomEventHandler for PauseState {
    fn update(&mut self, _ctx: &mut Context) -> HandlerMessage {
        HandlerMessage::Keep
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, graphics::BLACK);
		let point = nalgebra::Point2::new(500.0, 500.0);
		
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
    
    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) -> HandlerMessage {
        match key {
            KeyCode::P => {
                HandlerMessage::Bail
            },
            _ => HandlerMessage::Keep
        }
    }
}

impl PauseState {
    pub fn new(ctx: &mut Context) -> PauseState {
		let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
		let t = graphics::Text::new(("Paused".to_string(), font, 22.0));
        PauseState {
			text: t,
        }
    }
}