pub struct PauseState {
    
}

impl EventHandler for PauseState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    
    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) {
        match key {
            KeyCode::P => println!("Pause? Maybe latter."),
            //KeyCode::Escape => quit(ctx),
            // other keys to detect
            _ => { /* Do Nothing */ }
        }
    }
}

impl PauseState {
    pub fn new(ctx: &mut Context) -> PauseState {
        PauseState {
            
        }
    }
}