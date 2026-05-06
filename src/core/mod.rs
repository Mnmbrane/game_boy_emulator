//! Core emulation modules.
//!
//! Everything in this module tree is intended to model Game Boy hardware
//! behavior without depending on terminal UI concerns.

/// Bus and address routing for hardware-visible memory access.
mod bus;
/// Cartridge state and ROM-facing behavior.
mod cartridge;
/// CPU state and instruction execution.
mod cpu;
/// DMA transfer state and coordination.
mod dma;
/// Interrupt controller state and signaling.
mod interrupts;
/// Raw memory regions owned by the machine.
mod memory;
/// LCD and framebuffer generation logic.
mod ppu;
/// Divider and timer register behavior.
mod timers;

pub use self::{
    bus::Bus, cartridge::Cartridge, cpu::CPU, dma::DMA, interrupts::Interrupts, memory::Memory,
    ppu::PPU, timers::Timers,
};

/// Top-level emulator state.
///
/// `GameBoy` owns the major hardware subsystems and will eventually coordinate
/// stepping, timing, and state access for the frontend.
pub struct GameBoy {
    bus: Bus,
    cartridge: Cartridge,
    cpu: CPU,
    dma: DMA,
    interrupts: Interrupts,
    memory: Memory,
    ppu: PPU,
    timers: Timers,
}

impl GameBoy {
    /// Creates a new machine with placeholder subsystem state.
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            cartridge: Cartridge::new(),
            cpu: CPU::new(),
            dma: DMA::new(),
            interrupts: Interrupts::new(),
            memory: Memory::new(),
            ppu: PPU::new(),
            timers: Timers::new(),
        }
    }
}
