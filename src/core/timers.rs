//! Timer and divider state.
//!
//! This module is intended to model the Game Boy timer registers and the cycle
//! based behavior that drives timer increments and overflow interrupts.

/// Placeholder for timer-related hardware state.
pub struct Timers;

impl Timers {
    /// Creates a timer placeholder.
    pub fn new() -> Self {
        Self
    }

    /// Returns the value of a timer-mapped register.
    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    /// Updates a timer-mapped register.
    pub fn write8(&mut self, _addr: u16, _value: u8) {}
}
