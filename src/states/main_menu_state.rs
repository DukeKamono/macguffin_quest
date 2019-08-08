// Right now this is a PauseState Clone :(
// I want to add more to this later. (Art, settings section, etc)
pub struct MainMenuState {
    text: graphics::Text,
    chosen_player: String,
}

impl CustomEventHandler for MainMenuState {
    fn update(&mut self, _ctx: &mut Context) -> HandlerMessage {
        HandlerMessage::Keep
    }
    
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
    
    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) -> HandlerMessage {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();

        if !self.chosen_player.is_empty() && key == KeyCode::Return {
            let state = Box::new(MainState::new(ctx, self.chosen_player.to_string()));
            return HandlerMessage::Spawn(state);
        }

        match key {
            KeyCode::E => {
                self.chosen_player = "/elf_fighter.png".to_string();
                self.text = graphics::Text::new(("Macguffin Quest\n\n\nYou Chose the Elf Fighter!\n\nInstructions:\nASWD: Move\nQ: Spell\nSpace: Attack\nHold Shift to Run\n\nPress Enter to Start".to_string(), font, 22.0));
            },
            KeyCode::S => {
                self.chosen_player = "/dapper-skeleton-sheet.png".to_string();
                self.text = graphics::Text::new(("Macguffin Quest\n\n\nYou Chose the Dapper Skeleton!\n\nInstructions:\nASWD: Move\nQ: Spell\nSpace: Attack\nHold Shift to Run\n\nPress Enter to Start".to_string(), font, 22.0));
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
    pub fn new(ctx: &mut Context) -> MainMenuState {
        let font = graphics::Font::new(ctx, "/square.ttf").unwrap();
        let t = graphics::Text::new(("Macguffin Quest\n\n\nPlease Choose an Adventurer!\n\nPress E for Elf Fighter\nPress S for Dapper Skeleton".to_string(), font, 22.0));
        MainMenuState {
            text: t,
            chosen_player: "".to_string(),
        }
    }
}