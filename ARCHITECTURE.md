# Game Boy Emulator Architecture

## Goal

Build a Game Boy emulator in Rust with a clean hardware core and a separate frontend boundary.

The project is not trying to optimize for "draw something on screen fast." It is trying to create a structure that can support:

- deterministic CPU stepping
- accurate-enough timing to grow into a real emulator
- terminal-based debugging tools
- alternate frontends later without rewriting the core

## Current State

The repository is in the skeleton phase.

What exists today:

- a single crate with `core` and `frontend` modules
- a `GameBoy` type that owns the main subsystems
- placeholder modules for bus, cartridge, CPU, DMA, interrupts, memory, PPU, and timers
- a placeholder frontend boundary

What does not exist yet:

- ROM loading
- a real memory map
- CPU registers or instruction execution
- cycle-based orchestration
- framebuffer generation
- joypad and APU modules
- debugger or terminal UI behavior

That means this document needs to describe both:

- the architecture the code is already moving toward
- the specific implementation gaps that still need to be closed

## Source Layout

Current tree:

```text
src/
  main.rs
  core/
    mod.rs
    bus.rs
    cartridge.rs
    cpu.rs
    dma.rs
    interrupts.rs
    memory.rs
    ppu.rs
    timers.rs
  frontend/
    mod.rs
```

Important consequence: the current codebase already has the right top-level split, but most modules are still placeholders. The next work is mostly about filling those boundaries in, not redesigning the tree again.

## Architectural Direction

The intended design is:

- `core` models Game Boy hardware
- `frontend` consumes machine state and machine APIs
- the frontend never becomes the owner of emulation behavior
- all hardware-visible reads and writes eventually pass through the bus
- execution advances in CPU cycles, and other devices step from that timing source

This is an accuracy-oriented core with a debugger-first presentation layer.

## Top-Level Responsibilities

### `main.rs`

`main.rs` is currently only a binary entry point. Eventually it should:

- parse CLI arguments
- load a ROM
- build a `GameBoy`
- choose a frontend mode
- drive the main emulator loop

It should not become a second orchestration layer that duplicates machine logic.

### `core`

The `core` module owns emulation logic. It should remain free of:

- terminal escape handling
- keyboard library details
- UI rendering code
- presentation-specific formatting

### `frontend`

The `frontend` module owns interaction and presentation. It should:

- map user input to core button state
- render debugger views and later framebuffer output
- inspect state exposed by the machine
- avoid mutating internals directly

## `GameBoy` as the Machine Boundary

The current `GameBoy` struct is the right place to centralize system ownership:

- `bus`
- `cartridge`
- `cpu`
- `dma`
- `interrupts`
- `memory`
- `ppu`
- `timers`

This should become the single machine boundary used by the frontend.

`GameBoy` should eventually be responsible for:

- machine construction and reset
- ROM insertion/loading
- stepping one instruction
- stepping one scanline or one frame
- exposing read-only state for debugging
- coordinating device timing after CPU execution

Right now, the public methods are placeholders and do not take `&self` or `&mut self`. That is acceptable during scaffolding, but the real API should converge toward methods like:

```rust
pub fn step_instruction(&mut self) -> u32;
pub fn step_frame(&mut self);
pub fn framebuffer(&self) -> &[u8];
pub fn set_button(&mut self, button: Button, pressed: bool);
```

The exact signatures can change, but the ownership model should be like this:

- mutating machine behavior requires `&mut self`
- inspection APIs use `&self`
- the frontend talks to `GameBoy`, not directly to individual devices

## Execution Model

The target execution flow should be cycle-based.

High-level shape:

1. CPU fetches and executes one instruction.
2. CPU reports how many cycles were consumed.
3. Timers, PPU, DMA, and interrupt state advance from those cycles.
4. If hardware raises an interrupt, the CPU observes it on a later step according to IME rules.

This is the main loop architecture to preserve because it keeps timing decisions explicit.

Avoid these failure modes:

- letting each subsystem advance itself on unrelated schedules
- updating timers or PPU only once per frame without a cycle model
- letting the frontend decide emulation timing

## Subsystem Boundaries

### `bus`

The bus should become the single routing layer for hardware-visible reads and writes.

Responsibilities:

- decode 16-bit addresses
- route reads and writes to the correct region or device
- apply special behavior for memory-mapped I/O registers
- centralize any access restrictions introduced by DMA or PPU timing later

Expected API shape:

```rust
pub fn read8(&mut self, addr: u16) -> u8;
pub fn write8(&mut self, addr: u16, value: u8);
pub fn read16(&mut self, addr: u16) -> u16;
pub fn write16(&mut self, addr: u16, value: u16);
```

Design note: even if some of these helpers are implemented in the CPU first, the project should still keep one authoritative memory-routing path.

### `memory`

`memory` should own raw machine-backed storage for internal regions, not cartridge ROM.

Expected regions:

- VRAM
- WRAM
- OAM
- HRAM
- I/O register storage where direct device ownership is not better

This module should stay simple. It is mostly storage plus region-specific helpers. Policy-heavy behaviors belong in the bus or device modules.

### `cartridge`

`cartridge` should own ROM-facing behavior:

- ROM bytes
- cartridge header data
- MBC selection
- bank switching state
- external RAM

Implementation order should be:

1. ROM-only cartridges
2. header parsing
3. MBC1
4. battery-backed RAM concerns later

This isolates game-specific board behavior from the rest of the machine.

### `cpu`

`cpu` should own:

- registers
- flags
- `PC` and `SP`
- fetch/decode/execute logic
- interrupt entry behavior
- halt/stop state later

The CPU should not own memory. It should use the bus for:

- opcode fetch
- operand fetch
- stack reads and writes
- memory-indirect operations

This is a critical architectural rule. If the CPU starts reaching into memory directly, the emulator will become harder to reason about once timers, DMA, and I/O side effects matter.

### `timers`

`timers` should model:

- `DIV`
- `TIMA`
- `TMA`
- `TAC`
- timer stepping from elapsed cycles
- timer overflow interrupt requests

Timer behavior is usually one of the earliest correctness traps in emulator work. That is a reason to keep the state isolated and explicit.

### `interrupts`

`interrupts` should own:

- `IE`
- `IF`
- `IME`
- interrupt request helpers
- pending interrupt resolution

This module should answer questions such as:

- which interrupts are requested
- which are enabled
- whether the CPU should service one now

The CPU should consume this information, but should not become the owner of global interrupt state.

### `dma`

`dma` should own OAM DMA transfer behavior:

- source high byte
- active/inactive transfer state
- transfer timing
- copy progression

The first implementation can be simplified, but the simplification should be explicit. For example:

- first version: copy immediately when DMA is triggered
- later version: model timing and bus restrictions more accurately

### `ppu`

`ppu` should own video behavior and produce a normal framebuffer independent of terminal rendering.

Responsibilities:

- LCD control/status registers
- scanline state
- mode timing
- background rendering
- window rendering later
- sprite rendering later
- framebuffer storage
- VBlank and STAT interrupt requests

The PPU should never emit terminal-formatted output. It should emit pixels or palette indices that a frontend can choose to display.

### Missing but Planned Modules

The current architecture notes and TODO mention two modules that do not exist yet:

- `joypad`
- `apu`

Those should be added once the current skeleton begins to solidify.

`joypad` responsibilities:

- button state
- joypad register behavior
- interrupt signaling on input changes as needed

`apu` responsibilities:

- audio register ownership
- channel state
- sample generation later

`joypad` is higher priority than `apu`.

## Memory Map Responsibilities

The emulator should eventually support the standard DMG-style memory layout:

```text
0000-3FFF  ROM bank 0
4000-7FFF  Switchable ROM bank
8000-9FFF  VRAM
A000-BFFF  External cartridge RAM
C000-DFFF  WRAM
E000-FDFF  Echo RAM
FE00-FE9F  OAM
FEA0-FEFF  Unusable
FF00-FF7F  I/O registers
FF80-FFFE  HRAM
FFFF        IE register
```

The important architectural decision is not just knowing this map. It is deciding ownership:

- cartridge owns ROM and external RAM behavior
- memory owns raw VRAM/WRAM/OAM/HRAM storage
- device modules own device-specific register semantics
- bus ties the address map to those owners

## Data Ownership Rules

To keep the codebase maintainable, use these rules:

- device logic owns device state
- `GameBoy` owns the devices
- the bus routes accesses, but should not become a giant second copy of all hardware state
- frontend code reads machine state through stable APIs instead of poking fields directly

This avoids the two common failure modes:

- everything is public and nothing has a clear owner
- the bus becomes a dumping ground for unrelated hardware logic

## Frontend Contract

The frontend should eventually depend on a narrow machine contract.

Likely needs:

- step controls
- current register snapshot
- recent trace data later
- memory inspection APIs
- framebuffer access
- button input APIs

The frontend should not need to know:

- where VRAM is physically stored
- how timer edges are computed
- how interrupt priority is resolved internally

If the frontend needs that level of access, the core boundary is probably too weak.

## Immediate Gaps in the Current Code

The current code structure is good, but these gaps are visible right now:

- `ARCHITECTURE.md` and `TODO.md` describe `joypad` and `apu`, but those modules are not present
- `GameBoy` exists, but its stepping APIs are still placeholder print calls
- subsystem modules have clear intent comments, but no internal state yet
- `main.rs` constructs placeholders but does not drive any emulator workflow
- `frontend` exists only as a type boundary and exposes no debugger or rendering behavior
- `Cargo.toml` has no dependencies yet, which is correct for now but means no CLI or TUI direction has been chosen in code

## Recommended Build Order

The highest-value implementation order from here is:

1. Define CPU registers and a real `GameBoy` mutable API.
2. Implement `Memory` storage and the bus memory map.
3. Add ROM loading with ROM-only cartridge support.
4. Make the CPU fetch opcodes through the bus and return cycle counts.
5. Add timer and interrupt plumbing.
6. Add a minimal debugger-style frontend.
7. Add PPU timing and framebuffer generation.
8. Add joypad handling.
9. Add MBC support, DMA refinements, and later APU work.

This order matches the current project shape and avoids premature work on presentation or audio.

## Testing Strategy

Architecture should support testing from the start.

Useful early tests:

- CPU register and flag helpers
- bus address decoding
- memory region reads and writes
- cartridge header parsing
- timer increment and overflow behavior
- interrupt priority and masking
- deterministic instruction stepping

Good architecture makes these tests local. If every subsystem test requires booting the whole emulator, the boundaries are probably too entangled.

## Design Constraints to Preserve

As implementation begins, protect these rules:

- do not bypass the bus for convenience
- do not let the frontend own emulation timing
- do not tie the PPU to terminal rendering
- do not start with APU work
- do not spread interrupt state across CPU and unrelated modules
- do not make every field public just to get moving

These constraints matter more than exact file names.

## Near-Term Definition of Success

The architecture is on track when the project can do all of the following:

- load a ROM-only cartridge
- fetch and execute at least a small subset of instructions
- expose CPU state for inspection
- step deterministically
- advance timers from CPU cycles
- keep all frontend behavior outside the core

Once that exists, the project moves from "scaffold" to "real emulator core."
