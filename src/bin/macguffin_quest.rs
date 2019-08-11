//use macguffin_quest::states::*;
use macguffin_quest::state_revamp::StateManager;

fn main() {
    // create a context to access hardware (also creates event loop)
    let (ref mut ctx, ref mut event_loop) =
        ggez::ContextBuilder::new("macguffin_quest", "James M. & William O.")
            .add_resource_path(std::path::PathBuf::from("./resources/texture"))
            .add_resource_path(std::path::PathBuf::from("./resources/font"))
            .add_resource_path(std::path::PathBuf::from("./resources/level"))
            .build()
            .unwrap();

    /* Old Code - Will be cleaned up / removed 
    // initial state to start game
    let state = Box::new(MainMenuState::new(ctx));
    // create state machine to manage states (add initial state)
    let state_machine = &mut StateMachine::new(state);
    */

    // revamp code
    // may want to test for errors and if found print a pretty error message
    let state_machine = &mut StateManager::new(ctx).unwrap();

    // start game loop
    match ggez::event::run(ctx, event_loop, state_machine) {
        Ok(_) => println!("Exiting Game."),
        Err(e) => println!("Crashing Game! {}", e),
    }
}
