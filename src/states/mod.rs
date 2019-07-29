// Namespace states
// Contains modules related to drawable states

include!("./state_machine.rs");

include!("./main_state.rs");

include!("./pause_state.rs");

include!("./main_menu_state.rs");

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::*;