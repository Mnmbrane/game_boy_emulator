# Game Boy Advance Emulator TODO

## Immediate Goal

Build a minimal but well-structured emulator core that can load a BIOS and ROM, execute instructions deterministically, and expose enough state for terminal-based debugging.

## Phase 0: Project Setup

- [ ] Decide and document the source tree layout before adding many files
- [ ] Create core module boundaries for CPU, bus, memory, cartridge, interrupts, timers, DMA, PPU, and frontend
- [ ] Add a short crate-level description to explain the project direction
- [ ] Decide early whether to keep everything in one crate or split into `core` and `frontend` crates later
- [ ] Add a basic error type or error strategy for ROM loading and invalid machine state

## Phase 1: Machine Skeleton

- [ ] Create a top-level `Gba` or `Machine` struct
- [ ] Add machine-owned subsystems as fields, even if some are stubs initially
- [ ] Define high-level stepping APIs such as instruction stepping and frame stepping
- [ ] Decide what public state access the frontend and debugger should use
- [ ] Keep frontend concerns out of the machine core

## Phase 2: CPU Foundation

- [ ] Implement the ARM7TDMI register file
- [ ] Implement CPSR and SPSR representations
- [ ] Support ARM and THUMB execution modes
- [ ] Implement CPU mode switching and banked registers
- [ ] Add fetch, decode, and execute scaffolding
- [ ] Make instruction execution return cycle counts
- [ ] Add a clean exception-entry path for interrupts and faults
- [ ] Add helpers for bit operations and status flag updates

## Phase 3: Memory and Bus

- [ ] Define the GBA memory map
- [ ] Add storage for BIOS, EWRAM, IWRAM, VRAM, palette RAM, OAM, cartridge ROM, and save memory
- [ ] Build a central bus API for 8-bit, 16-bit, and 32-bit reads and writes
- [ ] Route memory-mapped I/O through the bus rather than direct subsystem access
- [ ] Decide how invalid or unmapped accesses behave
- [ ] Leave room in the design for waitstates and timing penalties later

## Phase 4: ROM and BIOS Loading

- [ ] Add BIOS loading from file
- [ ] Add ROM loading from file
- [ ] Validate loaded data sizes and expected memory placement
- [ ] Define reset and startup behavior
- [ ] Add a minimal CLI path for selecting BIOS and ROM files

## Phase 5: First Executable Core

- [ ] Implement enough instruction decoding to begin executing real code
- [ ] Add a deterministic single-instruction stepping path
- [ ] Log PC, opcode, registers, and flags while debugging
- [ ] Confirm the CPU fetches instructions through the bus
- [ ] Verify that state changes are deterministic between runs

## Phase 6: Debugging Support

- [ ] Add a register dump view
- [ ] Add a memory dump view
- [ ] Add a disassembly view around the current PC
- [ ] Add breakpoints by address
- [ ] Add instruction trace output
- [ ] Add simple logging controls to avoid overwhelming output

## Phase 7: Interrupts and Timers

- [ ] Implement IME, IE, and IF register behavior
- [ ] Track pending interrupts centrally
- [ ] Wire interrupt entry into CPU execution
- [ ] Implement timer registers and counting behavior
- [ ] Support timer reloads, enable bits, and cascading
- [ ] Connect timer overflow to interrupt generation

## Phase 8: DMA

- [ ] Add DMA channel state structs
- [ ] Implement DMA source, destination, count, and control behavior
- [ ] Support major DMA trigger conditions
- [ ] Define how DMA interacts with CPU execution timing
- [ ] Add DMA-triggered interrupt support

## Phase 9: PPU and Framebuffer

- [ ] Add a framebuffer representation owned by the core
- [ ] Implement scanline timing
- [ ] Add display control register handling
- [ ] Start with a minimal display mode rather than every graphics mode at once
- [ ] Generate framebuffer output without terminal-specific formatting
- [ ] Add VBlank and HBlank signaling as timing becomes more accurate

## Phase 10: Terminal Debugger

- [ ] Add a simple CLI for run, step, reset, and inspect commands
- [ ] Decide whether the first debugger is line-based or TUI-based
- [ ] If using a TUI, evaluate `ratatui` and `crossterm`
- [ ] Show registers, flags, PC, current mode, and recent trace
- [ ] Add memory inspection and breakpoint controls
- [ ] Keep debugger reads non-invasive where possible

## Phase 11: Terminal Graphics Output

- [ ] Convert the framebuffer into terminal-friendly output
- [ ] Start with ANSI truecolor and Unicode half blocks
- [ ] Add frame pacing and redraw logic
- [ ] Measure terminal performance before optimizing output paths
- [ ] Consider optional support for kitty graphics or sixel later

## Phase 12: Input

- [ ] Define GBA button state in the core
- [ ] Map terminal keyboard input to GBA buttons
- [ ] Handle key press and release cleanly
- [ ] Avoid coupling raw input polling to CPU logic

## Testing

- [ ] Add unit tests for bit helpers and flag logic
- [ ] Add tests for register banking and CPU mode changes
- [ ] Add bus and memory map tests
- [ ] Add instruction decode and execution tests
- [ ] Add tests for timer overflow and interrupt behavior
- [ ] Add DMA tests for address/count/control behavior
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
- [ ] CPU registers and modes exist
- [ ] BIOS and ROM can load
- [ ] One instruction can be fetched, decoded, and stepped deterministically
- [ ] Register state can be printed in the terminal

## Things to Avoid Early

- [ ] Do not start with audio
- [ ] Do not optimize rendering before the core works
- [ ] Do not let the frontend mutate hardware state directly
- [ ] Do not bypass the bus for convenience
- [ ] Do not chase full graphics support before CPU and timing basics are stable
