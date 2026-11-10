# Game Boy Emulator TODO

## Immediate Goal

Build a minimal but well-structured emulator core that can load a ROM, execute instructions deterministically, and expose enough state for terminal-based debugging.

## Phase 0: Project Setup

- [x] Decide and document the source tree layout before adding many files
- [x] Create core module boundaries for CPU, bus, memory, cartridge, interrupts, timers, DMA, PPU, joypad, APU, and frontend
- [x] Add a short crate-level description to explain the project direction
- [x] Decide early whether to keep everything in one crate or split into `core` and `frontend` crates later

## Phase 1: Machine Skeleton

- [x] Create a top-level `GameBoy` or `Machine` struct
- [x] Add machine-owned subsystems as fields, even if some are stubs initially
- [x] Ensure `GameBoy` owns every planned hardware subsystem (`cpu`, `bus`, `memory`, `cartridge`, `interrupts`, `timers`, `dma`, `ppu`, `joypad`, `apu`) and document any temporary omissions
- [x] Add `joypad` and `apu` module boundaries so the machine layout matches the long-term hardware split
- [x] Decide whether the bus owns references to other subsystems or remains a pure routing façade until later
  - x|
- [ ] Decide whether reset state lives in `GameBoy::new`, a separate `reset`, or both
- [ ] Define a machine construction path
- [ ] Keep `GameBoy::new()` as a valid no-ROM boot path for scaffolding and tests
- [ ] Add a machine constructor or loader path that accepts cartridge/ROM input
- [ ] Decide whether invalid machine construction should return `Result`
- [ ] Define high-level stepping APIs such as instruction stepping and frame stepping
- [ ] Change placeholder stepping methods to real `&mut self` APIs
- [ ] Make instruction stepping return cycle counts
- [ ] Define what `step_frame` means before the PPU exists fully
- [ ] Decide whether to expose `step_cycles` or keep cycle stepping internal to the machine
- [ ] Decide what public state access the frontend and debugger should use
- [ ] Add read-only machine inspection APIs instead of exposing subsystem internals directly
- [ ] Decide which state gets dedicated accessors versus debug snapshot structs
- [ ] Define how framebuffer access should work before real rendering exists
- [ ] Decide whether debugger reads should come from `GameBoy` directly or a separate view type
- [ ] Keep frontend concerns out of the machine core
- [ ] Remove placeholder `println!` behavior from core-facing APIs
- [ ] Keep terminal formatting, input handling, and command parsing out of `core`
- [ ] Avoid letting the frontend mutate subsystem fields directly
- [ ] Add machine-level invariants
- [ ] Keep cross-subsystem stepping orchestration in `GameBoy`, not in `main.rs`
- [ ] Ensure CPU, timers, DMA, interrupts, and PPU advance through a single orchestration path
- [ ] Document which subsystem interactions are intentionally stubbed in phase 1

## Phase 2: CPU Foundation

- [ ] Implement the Game Boy CPU register file
- [ ] Represent the flag register cleanly
- [ ] Add fetch, decode, and execute scaffolding
- [ ] Make instruction execution return cycle counts
- [ ] Add a clean interrupt-entry path
- [ ] Add helpers for bit operations and flag updates
- [ ] Plan for halt behavior even if `HALT` is not fully implemented immediately

## Phase 3: Memory and Bus

- [ ] Define the Game Boy memory map
- [ ] Add storage for VRAM, WRAM, OAM, HRAM, and I/O register space
- [ ] Build a central bus API for 8-bit reads and writes
- [ ] Decide whether 16-bit access helpers should live in the bus or CPU layer
- [ ] Route memory-mapped I/O through the bus rather than direct subsystem access
- [ ] Decide how invalid or unmapped accesses behave

## Phase 4: ROM Loading and Cartridge Behavior

- [ ] Add ROM loading from file
- [ ] Parse cartridge header fields you care about early
- [ ] Start with ROM-only cartridge support
- [ ] Add MBC1 support after ROM-only works
- [ ] Add external RAM handling
- [ ] Add a minimal CLI path for selecting ROM files

## Phase 5: First Executable Core

- [ ] Implement enough instruction decoding to begin executing real code
- [ ] Add a deterministic single-instruction stepping path
- [ ] Log PC, opcode, registers, and flags while debugging
- [ ] Confirm the CPU fetches instructions through the bus
- [ ] Verify that state changes are deterministic between runs

## Phase 6: Debugging Support

- [ ] Add a register dump view
- [ ] Add a memory dump view
- [ ] Add a trace view or disassembly view around the current PC
- [ ] Add breakpoints by address
- [ ] Add simple logging controls to avoid overwhelming output

## Phase 7: Interrupts and Timers

- [ ] Implement IME, IE, and IF behavior
- [ ] Track pending interrupts centrally
- [ ] Wire interrupt entry into CPU execution
- [ ] Implement DIV behavior
- [ ] Implement TIMA, TMA, and TAC behavior
- [ ] Connect timer overflow to interrupt generation

## Phase 8: DMA

- [ ] Add OAM DMA state and trigger behavior
- [ ] Implement transfer copying into OAM
- [ ] Decide how strictly to model CPU/bus interaction during DMA in the first version

## Phase 9: PPU and Framebuffer

- [ ] Add a framebuffer representation owned by the core
- [ ] Implement LCD timing modes
- [ ] Implement scanline progression
- [ ] Add LCD control and status register handling
- [ ] Start with background rendering before window and sprites
- [ ] Generate framebuffer output without terminal-specific formatting
- [ ] Add VBlank and STAT interrupt behavior

## Phase 10: Joypad

- [ ] Define Game Boy button state in the core
- [ ] Implement joypad register behavior
- [ ] Connect input changes to interrupt generation as needed

## Phase 11: Terminal Debugger

- [ ] Add a simple CLI for run, step, reset, and inspect commands
- [ ] Decide whether the first debugger is line-based or TUI-based
- [ ] If using a TUI, evaluate `ratatui` and `crossterm`
- [ ] Show registers, flags, PC, and recent trace
- [ ] Add memory inspection and breakpoint controls
- [ ] Keep debugger reads non-invasive where possible

## Phase 12: Terminal Graphics Output

- [ ] Convert the framebuffer into terminal-friendly output
- [ ] Start with grayscale or simple palette mapping
- [ ] Add frame pacing and redraw logic
- [ ] Measure terminal performance before optimizing output paths

## Phase 13: APU

- [ ] Define an `Apu` boundary even if audio is stubbed at first
- [ ] Add audio register state
- [ ] Decide when actual sample generation is worth implementing

## Testing

- [ ] Add unit tests for bit helpers and flag logic
- [ ] Add CPU instruction decode and execution tests
- [ ] Add bus and memory map tests
- [ ] Add cartridge header and banking tests
- [ ] Add timer and interrupt tests
- [ ] Add DMA tests
- [ ] Add deterministic stepping tests where possible

## Nice to Have

- [ ] Add save-state support once the machine state is stable
- [ ] Add trace export for debugging regressions
- [ ] Add profiling hooks to find slow subsystems
- [ ] Add configurable logging levels
- [ ] Add frontend abstraction if you later want both terminal and windowed output

## First Milestone

These are the first tasks worth finishing before anything fancy:

- [ ] Machine struct exists
- [ ] Bus and memory map exist
- [ ] CPU registers and flags exist
- [ ] ROM loading works
- [ ] One instruction can be fetched, decoded, and stepped deterministically
- [ ] Register state can be printed in the terminal

## Things to Avoid Early

- [ ] Do not start with audio
- [ ] Do not optimize rendering before the core works
- [ ] Do not let the frontend mutate hardware state directly
- [ ] Do not bypass the bus for convenience
- [ ] Do not chase full PPU accuracy before CPU and timing basics are stable
