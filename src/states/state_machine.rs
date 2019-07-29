pub struct StateMachine {
    //main_menu_state: Option<MainMenuState>,
    //pub main_state: Option<MainState>,
    //pause_state: Option<PauseState>,
	current_state: Box<EventHandler>,
	
	//states: Vec<Box<EventHandler>>,
	//current: usize,
}
use ggez::event::EventsLoop;

impl StateMachine {
    pub fn new(ctx: &mut Context) -> StateMachine {
        StateMachine {
            //main_menu_state: None,
            //main_state: None,
            //pause_state: None,
			current_state: Box::new(MainState::new(ctx)),
        }
    }

    //pub fn new_main_state(&mut self, ctx: &mut Context) {
    //    self.main_state = Some(MainState::new(ctx));
    //}

    pub fn run(&mut self, ctx: &mut Context, event_loop: &mut EventsLoop) {
        //self.new_main_state(ctx);
        //let mut state = MainState::new(ctx);
        // start game loop
        match ggez::event::run(ctx, event_loop, self) {
            Ok(_) => println!("Exiting Game."),
            Err(e) => println!("Run event loop broke! {}", e),
        }
    }
}

impl EventHandler for StateMachine {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		///self.current_state = PauseState::new(ctx);
		self.current_state.update(ctx)
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.current_state.draw(ctx)
    }
}