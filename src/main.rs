// contains all the information on entities
mod entities;
mod ui;
mod sprites;

mod states;
use states::StateMachine;

fn main() {
    // create a context to access hardware (also creates event loop)
    let c = ggez::conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) =
        ggez::ContextBuilder::new("macguffin_quest", "James M. & William O.")
            .add_resource_path(std::path::PathBuf::from("./resources/texture"))
            .add_resource_path(std::path::PathBuf::from("./resources/font"))
            .conf(c)
            .build()
            .unwrap();

    let mut state_machine = StateMachine::new(ctx);
    
    //state_machine.new_main_state(ctx);
    //let state = &mut states::MainState::new(ctx);

    state_machine.run(ctx, event_loop);

    // start game loop
    //match ggez::event::run(ctx, event_loop, &mut state_machine.main_state.unwrap()) {
    //    Ok(_) => println!("Exiting Game."),
    //    Err(e) => println!("Run event loop broke! {}", e),
    //}
}
