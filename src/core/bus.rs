//! Bus infrastructure for the emulator core.
//!
//! The bus will become the single place where CPU-visible reads and writes are
//! decoded into the correct hardware region or device.

/// Central address-routing component for hardware-visible memory access.
pub struct Bus;

impl Bus {
    /// Creates an empty bus placeholder.
    pub fn new() -> Self {
        Self
    }
}
