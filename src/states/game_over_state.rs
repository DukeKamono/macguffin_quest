pub struct GameOverState {
    text: graphics::Text,
}

impl CustomEventHandler for GameOverState {
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
        .expect("ERROR drawing Game Over Text");
        
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
    
    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) -> HandlerMessage {
        match key {
            KeyCode::R => {
                let state = Box::new(MainState::new(ctx));
                HandlerMessage::Change(state)
            },
             KeyCode::Q => {
                let state = Box::new(MainMenuState::new(ctx));
                HandlerMessage::Change(state)
            },
            _ => HandlerMessage::Keep
        }
    }
}

impl GameOverState {
    pub fn new(ctx: &mut Context) -> GameOverState {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let t = graphics::Text::new(("GAME OVER!\nPress R to Restart\nPress Q to Quit".to_string(), font, 22.0));
        GameOverState {
            text: t,
        }
    }
}