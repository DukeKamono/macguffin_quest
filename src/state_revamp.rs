// Used information from:
// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5
// to figure out how to clone box traits

use std::mem;

use ggez::{Context, GameResult};
use ggez::event::{EventHandler, self};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::graphics::{BLACK, DrawParam, FilterMode, Font, self, Text, WHITE};
use ggez::timer::{self};




// trait defining functions that states need to use if they want to work with StateManager
pub trait CustomStateTrait {
    // required
    
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Option<Box<CustomStateTrait>>>;
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<Option<Box<CustomStateTrait>>>;
    fn box_clone(&self) -> Box<CustomStateTrait>;

    // optional

    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) -> GameResult<Option<Box<CustomStateTrait>>> {
        Ok(None)
    }
}
// add a clone for Box<CustomStateTrait>
impl Clone for Box<CustomStateTrait> {
    fn clone(&self) -> Box<CustomStateTrait> {
        self.box_clone()
    }
}




// Handles all states in the game
// Manages passing of EventHandler methods to current state
pub struct StateManager {
    state: Box<CustomStateTrait>,
}

impl StateManager {
    // associate functions

    // create a new StateManager
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let state = MainMenuState::new(ctx)?;
        Ok(StateManager {
            state
        })
    }

    // member functions

    // potentially change the current state
    fn cycle_state(&mut self, result: Option<Box<CustomStateTrait>>) {
        if let Some(newstate) = result {
            self.state = newstate;
        }
    }
}

// Implements ggez EventHandler trait for StateManager
// https://docs.rs/ggez/0.5.1/ggez/event/trait.EventHandler.html
impl EventHandler for StateManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let result = self.state.update(ctx)?;
        self.cycle_state(result);
        Ok(())
    }

    // draws the current state
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let result = self.state.draw(ctx)?;
        self.cycle_state(result);
        Ok(())
    }

    // Handles key down events from the keyboard
    // - On Esc close the game
    // ! may panic if error occurs in state's key_down_event
    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        if keycode == KeyCode::Escape {
            event::quit(ctx);
        }
        // need to handle the possible error case here!
        let result = self.state.key_down_event(ctx, keycode, keymods, repeat).unwrap();
        self.cycle_state(result);
    }
}




// A State that does not do anything (except be box cloned)
// Should not ever be used as an actual running state
// Makes for a good template though
#[derive(Clone)]
struct EmptyState {
    // see empty no fields at all
}

impl EmptyState {
    fn new() -> GameResult<Box<CustomStateTrait>> {
        Ok(Box::new(EmptyState{}))
    }
}

impl CustomStateTrait for EmptyState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Option<Box<CustomStateTrait>>> {
        // I wonder if this should be the unimplemented macro instead
        Ok(None)
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<Option<Box<CustomStateTrait>>> {
        // I wonder if this should be the unimplemented macro instead
        Ok(None)
    }
    fn box_clone(&self) -> Box<CustomStateTrait>{
        Box::new(self.clone())
    }
}





// Main menu for the video game
// - "q" key will cause game to quit
// - "p" will transition to pause state
#[derive(Clone)]
struct MainMenuState {
    text: Text,
}

impl MainMenuState {
    fn new(_ctx: &mut Context) -> GameResult<Box<CustomStateTrait>> {
        let font = Font::default();
        let message = "Main Menu".to_string();
        let text = Text::new((message, font, 24f32));
        Ok(Box::new(MainMenuState{
            text,
        }))
    }
}

impl CustomStateTrait for MainMenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Option<Box<CustomStateTrait>>> {
        Ok(None)
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<Option<Box<CustomStateTrait>>> {
        graphics::clear(ctx, BLACK);

        // queue menu text
        graphics::queue_text(ctx, &self.text, [0f32, 0f32], Some(WHITE));

        // draw menu text
        let (width, height) = self.text.dimensions(ctx);
        let width = width as f32 / 2f32;
        let height = height as f32 / 2f32;
        let dp = DrawParam::default().dest([400f32 - width, 300f32 - height]);
        graphics::draw_queued_text(ctx, dp, None, FilterMode::Linear)?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(None)
    }
    fn box_clone(&self) -> Box<CustomStateTrait>{
        Box::new(self.clone())
    }
    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) -> GameResult<Option<Box<CustomStateTrait>>> {
        match keycode {
            KeyCode::Q => event::quit(ctx),
            KeyCode::P => {
                let pause = PauseMenuState::new(ctx, self.box_clone())?;
                return Ok(Some(pause));
            },
            _ => (),
        }
        Ok(None)
    }
}




// Paused state for the video game
// - "q" will transition to main menu state
// - "p" will transition previous state (ie the one that did transitioned to pause)
#[derive(Clone)]
struct PauseMenuState {
    text: Text,
    previous_state: Box<CustomStateTrait>,
}

impl PauseMenuState {
    fn new(_ctx: &mut Context, previous_state: Box<CustomStateTrait>) -> GameResult<Box<CustomStateTrait>> {
        let font = Font::default();
        let message = "Pause Menu".to_string();
        let text = Text::new((message, font, 24f32));
        Ok(Box::new(PauseMenuState{
            text,
            previous_state,
        }))
    }
}

impl CustomStateTrait for PauseMenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Option<Box<CustomStateTrait>>> {
        Ok(None)
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<Option<Box<CustomStateTrait>>> {
        graphics::clear(ctx, BLACK);

        // queue menu text
        graphics::queue_text(ctx, &self.text, [0f32, 0f32], Some(WHITE));

        // draw menu text
        let (width, height) = self.text.dimensions(ctx);
        let width = width as f32 / 2f32;
        let height = height as f32 / 2f32;
        let dp = DrawParam::default().dest([400f32 - width, 300f32 - height]);
        graphics::draw_queued_text(ctx, dp, None, FilterMode::Linear)?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(None)
    }
    fn box_clone(&self) -> Box<CustomStateTrait>{
        Box::new(self.clone())
    }
    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) -> GameResult<Option<Box<CustomStateTrait>>> {
        match keycode {
            KeyCode::Q => {
                let main = MainMenuState::new(ctx)?;
                return Ok(Some(main));
            },
            KeyCode::P => {
                let mut newstate = EmptyState::new()?;
                mem::swap(&mut self.previous_state, &mut newstate);
                return Ok(Some(newstate))
            },
            _ => (),
        }
        Ok(None)
    }
}