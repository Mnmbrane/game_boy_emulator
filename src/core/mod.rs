//! Core emulation modules.
//!
//! Everything in this module tree is intended to model Game Boy hardware
//! behavior without depending on terminal UI concerns.

/// Audio processing unit state.
mod apu;
/// Cartridge state and ROM-facing behavior.
mod cartridge;
/// CPU state and instruction execution.
mod cpu;
/// DMA transfer state and coordination.
mod dma;
/// Interrupt controller state and signaling.
mod interrupts;
/// Joypad input state
mod joypad;
/// Raw memory regions owned by the machine.
mod memory;
/// LCD and framebuffer generation logic.
mod ppu;
/// Divider and timer register behavior.
mod timers;

pub use self::{
    apu::Apu, cartridge::Cartridge, cpu::CPU, dma::DMA, interrupts::Interrupts, joypad::Joypad,
    memory::Memory, ppu::PPU, timers::Timers,
};

/// Top-level emulator state.
///
/// `GameBoy` owns the major hardware subsystems and will eventually coordinate
/// stepping, timing, and state access for the frontend.
pub struct GameBoy {
    apu: Apu,
    cartridge: Cartridge,
    cpu: CPU,
    dma: DMA,
    interrupts: Interrupts,
    joypad: Joypad,
    memory: Memory,
    ppu: PPU,
    timers: Timers,
}

#[derive(Debug)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Button::Up => "Up",
            Button::Down => "Down",
            Button::Left => "Left",
            Button::Right => "Right",
            Button::A => "A",
            Button::B => "B",
            Button::Start => "Start",
            Button::Select => "Select",
        };

        write!(f, "{name}")
    }
}

impl GameBoy {
    /// Creates a new machine with placeholder subsystem state.
    pub fn new() -> Self {
        Self {
            apu: Apu::new(),
            cartridge: Cartridge::new(),
            cpu: CPU::new(),
            dma: DMA::new(),
            interrupts: Interrupts::new(),
            joypad: Joypad::new(),
            memory: Memory::new(),
            ppu: PPU::new(),
            timers: Timers::new(),
        }
    }

    pub fn read8(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => return self.cartridge.read8(addr),
            0x8000..=0x9FFF | 0xC000..=0xFDFF | 0xFE00..=0xFE9F | 0xFF80..=0xFFFE => {
                return self.memory.read8(addr);
            }
            0xFF00 => return self.joypad.read8(addr),
            0xFF04..=0xFF07 => self.timers.read8(addr),
            0xFF0F | 0xFFFF => self.interrupts.read8(addr),
            0xFF46 => self.dma.read8(addr),
            0xFF10..=0xFF3F => self.apu.read8(addr),
            0xFF40..=0xFF45 | 0xFF47..=0xFF47 => self.ppu.read8(addr),
            // TODO: change addr to hex
            _ => panic!("Cannot access memory {addr}"),
        }
    }

    pub fn step_instruction() {
        println!("Step instruction");
    }

    pub fn step_frame() {
        println!("Step frame");
    }

    pub fn framebuffer() {
        println!("Framebuffer");
    }

    pub fn set_button(button: Button, pressed: bool) {
        println!("{button} is pressed ? {pressed}")
    }
}
