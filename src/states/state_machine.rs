pub struct StateMachine {
    main_menu_state: Option<MainMenuState>,
    pub main_state: Option<MainState>,
    pause_state: Option<PauseState>,
}
use ggez::event::EventsLoop;

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            main_menu_state: None,
            main_state: None,
            pause_state: None,
        }
    }

    pub fn new_main_state(&mut self, ctx: &mut Context) {
        self.main_state = Some(MainState::new(ctx));
    }

    pub fn run(&mut self, ctx: &mut Context, event_loop: &mut EventsLoop) {
        self.new_main_state(ctx);
        let mut state = MainState::new(ctx);
        // start game loop
        match ggez::event::run(ctx, event_loop, &mut state) {
            Ok(_) => println!("Exiting Game."),
            Err(e) => println!("Run event loop broke! {}", e),
        }
    }
}

