/// Game over menu state for game.
/// Sadly informs the player they have lost and lets them return to the main game state.
pub struct GameOverState {
    text: graphics::Text,
}

/// Implement CustomEventHandler from macguffin_quest::states::CustomEventHandler.
/// Allows the state machine to pass on information.
impl CustomEventHandler for GameOverState {
    /// Updates GameOverState.
    /// Does not really do anything... Real magic happens in key_down_event.
    fn update(&mut self, _ctx: &mut Context) -> HandlerMessage {
        HandlerMessage::Keep
    }
    
    /// Draws GameOverState.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        let point = nalgebra::Point2::new(250.0, 175.0);
        
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
    
    /// Processes a key down event.
    /// This is where transitioning to main menu state occurs.
    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) -> HandlerMessage {
        match key {
             KeyCode::Return => {
                let state = Box::new(MainMenuState::new(ctx));
                HandlerMessage::Change(state)
            },
            _ => HandlerMessage::Keep
        }
    }
}

impl GameOverState {
    /// Creates a new GameOverState with default values
    pub fn new(ctx: &mut Context) -> GameOverState {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let t = graphics::Text::new(("GAME OVER!\nPress Enter To Go Back To The Main Menu".to_string(), font, 22.0));
        GameOverState {
            text: t,
        }
    }
}

#[cfg(test)]
mod testgameover {
    use super::*;

    fn create_game_over_state_and_context() -> (GameOverState, Context) {
        let (mut ctx, _event_loop) =
            ggez::ContextBuilder::new("macguffin_quest", "James M. & William O.")
            .add_resource_path(std::path::PathBuf::from("./resources/texture"))
            .add_resource_path(std::path::PathBuf::from("./resources/font"))
            .add_resource_path(std::path::PathBuf::from("./resources/level"))
            .build()
            .unwrap();
        let mm = GameOverState::new(&mut ctx);
        (mm, ctx)
    }

    #[test]
    fn test_update() {
        let (ref mut sm, ref mut ctx) = create_game_over_state_and_context();
        match sm.update(ctx) {
            HandlerMessage::Keep => (),
            _ => panic!("HandlerMessage was not Keep"),
        }
    }
}