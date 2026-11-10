//! Joypad input state.
//!
//! This module will track button selection, button presses, and the register
//! behavior used to expose input to the rest of the machine.

/// Placeholder for joypad state.
pub struct Joypad;

impl Joypad {
    /// Creates a joypad placeholder.
    pub fn new() -> Self {
        Self
    }

    /// Returns the current joypad register value.
    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    /// Updates the joypad selection register state.
    pub fn write8(&mut self, _addr: u16, _value: u8) {}
}
