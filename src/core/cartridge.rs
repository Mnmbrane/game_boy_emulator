//! Cartridge and ROM-facing state.
//!
//! This module will hold ROM data, cartridge metadata, external RAM, and any
//! mapper or banking behavior required by supported cartridge types.

/// Cartridge-owned state such as ROM contents and banking logic.
pub struct Cartridge;

impl Cartridge {
    /// Creates an empty cartridge placeholder.
    pub fn new() -> Self {
        Self
    }
}
