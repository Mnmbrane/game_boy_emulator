//! Cartridge and ROM-facing state.
//!
//! This module will hold ROM data, cartridge metadata, external RAM, and any
//! mapper or banking behavior required by supported cartridge types.

pub struct RomBase0 {
    data: Vec<u8>,
}

impl RomBase0 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn read8(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn write8(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }
}

pub struct RomSwitchable {
    data: Vec<u8>,
    selected_bank: usize,
}

impl RomSwitchable {
    // Bank 0 is already taken
    const DEFAULT_BANK: usize = 1;
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            selected_bank: Self::DEFAULT_BANK,
        }
    }

    pub fn read8(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn write8(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }
}

pub struct ExternalRam {
    data: Vec<u8>,
    enabled: bool,
}

impl ExternalRam {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            enabled: false,
        }
    }

    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    pub fn write8(&self, _addr: u16, _value: u8) {}
}

/// Cartridge-owned state such as ROM contents and banking logic.
pub struct Cartridge {
    pub rom_base_0: RomBase0,
    pub rom_switchable: RomSwitchable,
    pub external_ram: ExternalRam,
}

impl Cartridge {
    /// Creates an empty cartridge placeholder.
    pub fn new() -> Self {
        Self {
            rom_base_0: RomBase0::new(),
            rom_switchable: RomSwitchable::new(),
            external_ram: ExternalRam::new(),
        }
    }

    /// Returns the value of a DMA-mapped register.
    pub fn read8(&self, _addr: u16) -> u8 {
        0xFF
    }

    /// Applies a write that may configure or trigger DMA behavior.
    pub fn write8(&mut self, _addr: u16, _value: u8) {}
}
