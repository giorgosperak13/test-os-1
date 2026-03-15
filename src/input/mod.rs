// src/input/mod.rs

mod keyboard;  // a module for handling keyboard events
mod mouse;     // a module for handling mouse events

pub use keyboard::*;
pub use mouse::*;