//! Entry point for the emulator binary.
//!
//! This crate is currently organized around two top-level concerns:
//! emulation in `core` and user interaction in `frontend`.

mod core;
mod frontend;

use crate::core::GameBoy;
use crate::frontend::Frontend;

fn main() {
    println!("Hello, world!");
    let game_boy = GameBoy::new();
    let frontend = Frontend::new();
}
