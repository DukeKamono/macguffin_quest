// based on guide found at
// https://github.com/ggez/ggez/blob/master/docs/guides/HelloGgez.md

use ggez::*;

struct State {
    dt: std::time::Duration,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        println!("Hello ggex! dt = {}nx", self.dt.subsec_nanos());
        Ok(())
    }
}

fn main() {
    // create an instance of game state
    let state = &mut State { dt: std::time::Duration::new(0, 0) };

    // create a context to access hardware (also creates event loop)
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .conf(c)
        .build()
        .unwrap();
    
    // start game loop
    event::run(ctx, event_loop, state).unwrap();

    println!("Goodbye");
}
