//! Raw memory regions for the emulator core.
//!
//! This module will eventually own storage such as WRAM, VRAM, OAM, HRAM, and
//! possibly helper APIs for region-specific access.

/// Placeholder for machine-owned memory regions.
pub struct Memory;

impl Memory {
    /// Creates a memory placeholder.
    pub fn new() -> Self {
        Self
    }

    /// Returns the byte stored in a machine-owned memory region.
    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    /// Stores a byte in a machine-owned memory region.
    pub fn write8(&mut self, _addr: u16, _value: u8) {}
}
