//! Audio processing unit state.
//!
//! This module will model the Game Boy sound hardware, including audio
//! registers, channel state, and later sample generation.

/// Placeholder for audio subsystem state.
pub struct Apu;

impl Apu {
    /// Creates an audio subsystem placeholder.
    pub fn new() -> Self {
        Self
    }

    /// Returns the value of an audio register.
    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    /// Updates an audio register.
    pub fn write8(&mut self, _addr: u16, _value: u8) {}
}
