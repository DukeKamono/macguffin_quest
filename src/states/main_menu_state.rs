/// Main menu state for game.
/// Allows character selection and causes state transition to the main game state.
pub struct MainMenuState {
    text: graphics::Text,
    chosen_player: String,
}

/// Implement CustomEventHandler from macguffin_quest::states::CustomEventHandler.
/// Allows the state machine to pass on information.
impl CustomEventHandler for MainMenuState {
    /// Updates MainMenuState.
    /// Does not really do anything... Real magic happens in key_down_event.
    fn update(&mut self, _ctx: &mut Context) -> HandlerMessage {
        HandlerMessage::Keep
    }
    
    /// Draws MainMenuState.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        let point = nalgebra::Point2::new(250.0, 175.0);
        
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
    
    /// Processes a key down event.
    /// This is where character selection and transitioning to main game state occurs.
    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) -> HandlerMessage {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();

        if !self.chosen_player.is_empty() && key == KeyCode::Return {
            let state = Box::new(MainState::new(ctx, self.chosen_player.to_string()));
            return HandlerMessage::Spawn(state);
        }

        match key {
            KeyCode::E => {
                self.chosen_player = "/elf_fighter.png".to_string();
                self.text = graphics::Text::new(("Macguffin Quest\n\n\nYou Chose the Elf Fighter!\n\nInstructions:\nASWD: Move\nQ: Spell\nSpace: Attack\nHold Shift to Run\nP: Pause\n\nPress Enter to Start".to_string(), font, 22.0));
            },
            KeyCode::S => {
                self.chosen_player = "/dapper-skeleton-sheet.png".to_string();
                self.text = graphics::Text::new(("Macguffin Quest\n\n\nYou Chose the Dapper Skeleton!\n\nInstructions:\nASWD: Move\nQ: Spell\nSpace: Attack\nHold Shift to Run\nP: Pause\n\nPress Enter to Start".to_string(), font, 22.0));
            },
            _ => {
                self.chosen_player = "".to_string();
                self.text = graphics::Text::new(("Macguffin Quest\n\n\nPlease Choose an Adventurer!\n\nPress E for Elf Fighter\nPress S for Dapper Skeleton".to_string(), font, 22.0));
            },
        };

        HandlerMessage::Keep
    }
}

impl MainMenuState {
    /// Creates a new MainMenuState with default values
    pub fn new(ctx: &mut Context) -> MainMenuState {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let t = graphics::Text::new(("Macguffin Quest\n\n\nPlease Choose an Adventurer!\n\nPress E for Elf Fighter\nPress S for Dapper Skeleton".to_string(), font, 22.0));
        MainMenuState {
            text: t,
            chosen_player: "".to_string(),
        }
    }
}

#[cfg(test)]
mod testmain {
    use super::*;

    fn create_menu_state_and_context() -> (MainMenuState, Context) {
        let (mut ctx, _event_loop) =
            ggez::ContextBuilder::new("macguffin_quest", "James M. & William O.")
            .add_resource_path(std::path::PathBuf::from("./resources/texture"))
            .add_resource_path(std::path::PathBuf::from("./resources/font"))
            .add_resource_path(std::path::PathBuf::from("./resources/level"))
            .build()
            .unwrap();
        let mm = MainMenuState::new(&mut ctx);
        (mm, ctx)
    }

    #[test]
    fn test_update() {
        let (ref mut sm, ref mut ctx) = create_menu_state_and_context();
        match sm.update(ctx) {
            HandlerMessage::Keep => (),
            _ => panic!("HandlerMessage was not Keep"),
        }
    }
}