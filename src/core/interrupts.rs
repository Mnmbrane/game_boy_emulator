//! Interrupt controller state.
//!
//! This module will track interrupt enable, interrupt flags, master enable, and
//! the logic used to signal pending interrupts to the CPU.

/// Placeholder for interrupt controller state.
pub struct Interrupts;

impl Interrupts {
    /// Creates an interrupt controller placeholder.
    pub fn new() -> Self {
        Self
    }

    /// Returns the value of an interrupt control register.
    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    /// Updates an interrupt control register.
    pub fn write8(&mut self, _addr: u16, _value: u8) {}
}
