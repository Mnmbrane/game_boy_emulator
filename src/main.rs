//! Entry point for the emulator binary.
//!
//! This crate is currently organized around two top-level concerns:
//! emulation in `core` and user interaction in `frontend`.
//!
//! The project direction is:
//! - keep the emulation core frontend-agnostic
//! - build correctness and debuggability before presentation polish
//! - start with a simple frontend for stepping, inspection, and input wiring
//! - leave richer rendering and platform-specific UI concerns at the boundary
//!
//! In practice, that means `core` should model Game Boy hardware and expose
//! narrow machine APIs, while `frontend` should translate those APIs into a
//! debugger or playable interface without leaking UI concerns back into the
//! emulator.

mod core;
mod frontend;

use crate::core::GameBoy;
use crate::frontend::Frontend;

fn main() {
    println!("Hello, world!");
    let game_boy = GameBoy::new();
    let frontend = Frontend::new();
}
