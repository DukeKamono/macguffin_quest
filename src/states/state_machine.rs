/// Trait that must be implemented by all states that wish to work with the state machine.
/// Needed to pass ggez::event::EventHandler functions to states managed by state manager.
pub trait CustomEventHandler {
    // required

    /// Used to pass parameters from ggez::event::EventHandler update to state
    fn update(&mut self, _ctx: &mut Context) -> HandlerMessage;

    /// Used to pass parameters from ggez::event::EventHandler draw to state
    fn draw(&mut self, _ctx: &mut Context) -> GameResult;

    /// Used to pass parameters from ggez::event::EventHandler draw to state
    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) -> HandlerMessage { HandlerMessage::Keep }
    
    // add more EventHandler method wrappers as needed
}

/// Enum for states to return to state manager, telling the state manager what to do. (ie how to transition between states)
pub enum HandlerMessage {
    /// no change needed, stick with current CustomEventHandler
    Keep,
    /// leave current CustomEventHandler, going back to previous CustomEventHandler
    Bail,
    /// spawn new CustomEventHandler on-top of current CustomEventHandler
    Spawn(Box<dyn CustomEventHandler>),
    /// change current CustomEventHandler into a new CustomEventHandler [Bail then Spawn]
    Change(Box<dyn CustomEventHandler>),
    /// an error occurred and needs reported
    Error(GameError),
}

impl HandlerMessage {
    /// Helper function for translating what the messages mean and making the state machine do the correct transition.
    fn handle(self, sm: &mut StateMachine) -> GameResult {
        match self {
            HandlerMessage::Keep => (),
            HandlerMessage::Bail => {sm.pop();},
            HandlerMessage::Spawn(new) => sm.push(new),
            HandlerMessage::Change(new) => {sm.pop(); sm.push(new)},
            HandlerMessage::Error(err) => return Err(err),
        };
        Ok(())
    }
}

/// Struct used to manage various state the game may be in.
/// Passes required information to the current state.
pub struct StateMachine{
    // Stack of States (top is active) [should quit if empty]
    states: Vec<Box<dyn CustomEventHandler>>,
}

impl StateMachine {
    /// Create a new StateMachine.
    pub fn new(state: Box<dyn CustomEventHandler>) -> StateMachine {
        let mut states = Vec::new();
        states.push(state);
        StateMachine {
            states,
        }
    }

    /// Push a new state onto state machine's stack.
    pub fn push(&mut self, state: Box<dyn CustomEventHandler>) {
        self.states.push(state)
    }

    /// Pops a state off of the state machine's state.
    pub fn pop(&mut self) -> Option<Box<dyn CustomEventHandler>> {
        self.states.pop()
    }

    /// Does the state machine have any states.
    /// Quits the game if true.
    fn is_empty(&self, ctx: &mut Context) -> bool {
        if self.states.is_empty() {
            event::quit(ctx);
            return true
        }
        false
    }
}

/// Implements EventHandler for State (ie state used by LevelBuilder)
/// https://docs.rs/ggez/0.5.1/ggez/event/trait.EventHandler.html
impl EventHandler for StateMachine {
    /// Updates the current state.
    /// May cause transition to a new state.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // if there are no states don't update anything, just quit
        if self.is_empty(ctx) {
            return Ok(())
        }

        // know that states has something in it so it is ok to unwrap
        self.states.last_mut().unwrap().update(ctx).handle(self)
    }
    
    /// Draws the current State.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // if there are no states don't draw anything, just quit
        // note should already have been checked in StateMachine.update()
        if self.is_empty(ctx) {
            return Ok(())
        }

        
        self.states.last_mut().unwrap().draw(ctx)
    }

    /// Passes key down event to current state.
    /// May cause transition to a new state.
    /// Ignores errors... so make sure none occur! (How could that go wrong?)
    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        // if escape key is hit just quit
        // if there are no states don't pass on anything, just quit
        if keycode == KeyCode::Escape || self.is_empty(ctx) {
            event::quit(ctx);
            return
        }

        match self.states.last_mut().unwrap().key_down_event(ctx, keycode, keymods, repeat).handle(self) {
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_state_machine_and_context() -> (StateMachine, Context) {
        let (mut ctx, _event_loop) =
            ggez::ContextBuilder::new("macguffin_quest", "James M. & William O.")
            .add_resource_path(std::path::PathBuf::from("./resources/texture"))
            .add_resource_path(std::path::PathBuf::from("./resources/font"))
            .add_resource_path(std::path::PathBuf::from("./resources/level"))
            .build()
            .unwrap();
        let mm = MainMenuState::new(&mut ctx);
        (StateMachine::new(Box::new(mm)), ctx)
    }

    #[test]
    fn test_state_machine_push() {
        let (ref mut sm, ref mut ctx) = create_state_machine_and_context();
        assert_eq!(sm.states.len(), 1usize);
        let mm = MainMenuState::new(ctx);
        sm.push(Box::new(mm));
        assert_eq!(sm.states.len(), 2usize);
    }

    #[test]
    fn test_state_machine_pop() {
        let (ref mut sm, ref mut _ctx) = create_state_machine_and_context();
        assert_eq!(sm.states.len(), 1usize);
        let r = sm.pop();
        assert!(r.is_some());
        assert_eq!(sm.states.len(), 0usize);
        let r = sm.pop();
        assert!(r.is_none());
        assert_eq!(sm.states.len(), 0usize);
    }

    #[test]
    fn test_state_machine_is_empty() {
        let (ref mut sm, ref mut ctx) = create_state_machine_and_context();
        assert_eq!(sm.states.len(), 1usize);
        let _r = sm.pop();
        let r = sm.is_empty(ctx);
        assert!(r);
    }
}