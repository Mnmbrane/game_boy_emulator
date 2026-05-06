mod bus;
mod cartridge;
mod cpu;
mod dma;
mod interrupts;
mod ppu;
mod timers;

pub use self::{
    bus::Bus, cartridge::Cartridge, cpu::CPU, dma::DMA, interrupts::Interrupts, ppu::PPU,
    timers::Timers,
};

pub struct GameBoy {
    bus: Bus,
    cartridge: Cartridge,
    cpu: CPU,
    dma: DMA,
    interrupts: Interrupts,
    ppu: PPU,
    timers: Timers,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            cartridge: Cartridge::new(),
            cpu: CPU::new(),
            dma: DMA::new(),
            interrupts: Interrupts::new(),
            ppu: PPU::new(),
            timers: Timers::new(),
        }
    }
}
