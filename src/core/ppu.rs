//! Pixel processing unit state.
//!
//! The PPU will eventually handle LCD timing, scanline progression, register
//! state, and framebuffer generation independent of frontend rendering.

/// Placeholder for the Game Boy PPU.
pub struct PPU;

impl PPU {
    /// Creates a PPU placeholder.
    pub fn new() -> Self {
        Self
    }

    /// Returns the value of a PPU-visible register or memory location.
    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    /// Updates a PPU-visible register or memory location.
    pub fn write8(&mut self, _addr: u16, _value: u8) {}
}
