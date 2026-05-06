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
}
