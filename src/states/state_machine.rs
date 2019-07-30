pub trait CustomEventHandler {
    // required
    fn update(&mut self, _ctx: &mut Context) -> HandlerMessage;
    fn draw(&mut self, _ctx: &mut Context) -> GameResult;

    // may be useful
    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) -> HandlerMessage { HandlerMessage::Keep }
    
    // add more EventHandler method wrappers as needed
}

pub enum HandlerMessage {
    // no change needed, stick with current CustomEventHandler
    Keep,
    // leave current CustomEventHandler, going back to previous CustomEventHandler
    Bail,
    // spawn new CustomEventHandler on-top of current CustomEventHandler
    Spawn(Box<CustomEventHandler>),
    // change current CustomEventHandler into a new CustomEventHandler [Bail then Spawn]
    Change(Box<CustomEventHandler>),
    // an error occurred and needs reported
    Error(GameError),
}

impl HandlerMessage {
    fn handle(self, sm: &mut StateMachine) -> GameResult {
        match self {
            HandlerMessage::Keep => (),
            HandlerMessage::Bail => {sm.pop(); ()},
            HandlerMessage::Spawn(new) => sm.push(new),
            HandlerMessage::Change(new) => {sm.pop(); sm.push(new)},
            HandlerMessage::Error(err) => return Err(err),
        };
        Ok(())
    }
}

pub struct StateMachine{
    // Stack of States (top is active) [should quit if empty]
	states: Vec<Box<CustomEventHandler>>,
}

impl StateMachine {
    pub fn new(state: Box<CustomEventHandler>) -> StateMachine {
        let mut states = Vec::new();
        states.push(state);
        StateMachine {
			states,
        }
    }

    pub fn push(&mut self, state: Box<CustomEventHandler>) {
        self.states.push(state)
    }

    pub fn pop(&mut self) -> Option<Box<CustomEventHandler>> {
        self.states.pop()
    }

    fn is_empty(&self, ctx: &mut Context) -> bool {
        if self.states.is_empty() {
            event::quit(ctx);
            return true
        }
        false
    }
}

impl EventHandler for StateMachine {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
        // if there are no states don't update anything, just quit
        if self.is_empty(ctx) {
            return Ok(())
        }

        // know that states has something in it so it is ok to unwrap
        self.states.last_mut().unwrap().update(ctx).handle(self)
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // if there are no states don't draw anything, just quit
        // note should already have been checked in StateMachine.update()
        if self.is_empty(ctx) {
            return Ok(())
        }

        
        self.states.last_mut().unwrap().draw(ctx)
    }

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