//! DMA transfer state.
//!
//! This module is reserved for OAM DMA behavior and any transfer timing rules
//! that affect memory access and sprite data updates.

/// Placeholder for direct memory access state.
pub struct DMA;

impl DMA {
    /// Creates a DMA placeholder.
    pub fn new() -> Self {
        Self
    }

    /// Returns the value of a DMA-mapped register.
    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    /// Applies a write that may configure or trigger DMA behavior.
    pub fn write8(&mut self, _addr: u16, _value: u8) {}
}
