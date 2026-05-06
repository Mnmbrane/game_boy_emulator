# Game Boy Emulator Architecture

## Goal

Build a Game Boy emulator in Rust with a terminal-oriented frontend, while keeping the emulation core clean enough to support debugging tools and alternate frontends later.

The main constraint is that terminal rendering is a presentation problem, not an emulation problem. The emulator should therefore be designed as:

- a hardware emulation core
- a frontend layer for terminal I/O and visualization

This keeps the core reusable and avoids mixing terminal-specific compromises into CPU, timing, memory, graphics, or input logic.

## Recommended Direction

Use an accuracy-oriented core with a debugger-first terminal interface, then add a gameplay-oriented terminal renderer later.

This gives you the cleanest path:

- stable subsystem boundaries
- cycle-based execution from the start
- easier debugging
- room to improve rendering without rewriting the core

## Design Options

### Option 1: Accuracy-First Core

Represent the hardware explicitly, execute instructions with cycle accounting, and advance the rest of the machine from elapsed cycles.

### Option 2: Frame-Oriented Prototype

Run the CPU in larger chunks and update hardware less precisely, which is faster to prototype but weaker for correctness.

### Option 3: Headless Emulator with TUI Debugger

Use a terminal debugger as the first frontend, which is strong for visibility and learning but not aimed at immediate playability.

### Option 4: Terminal Graphics Frontend

Render the Game Boy framebuffer into terminal output, which preserves good layering but should come after the core is stable.

## Recommended Combined Approach

Use:

- Option 1 for the emulator core
- Option 3 for the first frontend
- Option 4 later for terminal gameplay output

This keeps hardware emulation, debugging, and presentation cleanly separated.

## Architectural Principles

### 1. Keep the Core Frontend-Agnostic

The core should know nothing about:

- terminal escape sequences
- input libraries
- UI widgets
- terminal graphics protocols

The core should expose state and APIs that a frontend can consume.

### 2. Centralize Memory Access

All hardware-visible reads and writes should flow through a bus.

The bus is where you enforce:

- address decoding
- access width
- memory region behavior
- I/O register side effects
- timing rules later

If those rules are spread across CPU, timer, PPU, DMA, and cartridge code, correctness becomes much harder to verify.

### 3. Advance Time Explicitly

Time should be represented in cycles, not vague updates.

At minimum:

- CPU executes an instruction
- instruction returns cycles consumed
- other hardware advances using those cycles

This is the foundation for timers, PPU timing, interrupts, DMA, and input behavior.

### 4. Separate Orchestration from Device Logic

Each subsystem should own its own behavior, but one top-level machine object should coordinate execution.

The top-level machine decides:

- when to run CPU
- when to tick timers
- when to advance the PPU
- when DMA runs
- when interrupts become pending

This avoids scattering global control flow across many files.

### 5. Prefer Observable State

The emulator will need a lot of debugging, so state should be easy to inspect.

Examples:

- registers should be easy to dump
- flags should be explicit
- memory regions should be named
- interrupt and timer state should be visible

## Proposed Module Layout

One reasonable layout:

```text
src/
  main.rs
  core/
    mod.rs
    game_boy.rs
    bus.rs
    memory.rs
    cartridge.rs
    cpu.rs
    ppu.rs
    apu.rs
    dma.rs
    timers.rs
    interrupts.rs
    joypad.rs
  frontend/
    mod.rs
    tui.rs
    renderer.rs
```

This exact structure is optional, but the separation is important.

## Module Boundaries

Set the project up so each subsystem has one clear owner and one clear place to live. The point is to keep the emulator understandable as it grows.

Module intent:

- `cpu`: owns instruction execution, registers, flags, and interrupt entry behavior
- `bus`: owns all hardware-visible reads and writes and routes addresses to the correct target
- `memory`: owns raw storage for WRAM, VRAM, OAM, HRAM, and other basic memory regions
- `cartridge`: owns ROM loading, MBC behavior, external RAM, and cartridge metadata
- `interrupts`: owns IF, IE, IME, pending interrupt state, and interrupt signaling
- `timers`: owns DIV, TIMA, TMA, TAC, timer stepping, and overflow behavior
- `dma`: owns OAM DMA behavior and transfer state
- `ppu`: owns LCD timing, video registers, scanline state, and framebuffer generation
- `apu`: owns audio state and channel behavior
- `joypad`: owns button state and joypad register behavior
- `frontend`: owns terminal UI, debugger views, input mapping, and framebuffer presentation

First pass expectation:

- create the modules
- give each module a placeholder type
- let `GameBoy` or `Machine` own them
- do not force completeness before the structure is in place

## Core Subsystems

### `GameBoy` or `Machine`

Top-level emulator state.

Responsibilities:

- own all hardware subsystems
- expose high-level stepping APIs
- coordinate timing between devices
- provide snapshots or views for debugging/frontends

Possible API shape:

- `step_instruction()`
- `step_scanline()`
- `step_frame()`
- `framebuffer()`

### CPU

The Game Boy CPU is the Sharp SM83 family, often described as LR35902-like.

CPU responsibilities:

- registers and flags
- fetch, decode, execute
- interrupt entry
- halt and stop behavior later
- cycle reporting

Important design decision:

- keep instruction execution separate from memory ownership
- use the bus for instruction fetches and data access

### Bus

The bus is the central routing layer for reads and writes.

Responsibilities:

- map addresses to memory regions
- route I/O register access
- enforce region behavior
- become the single source of truth for observable memory access

### Memory

Represent the major Game Boy memory regions explicitly:

- VRAM
- WRAM
- OAM
- HRAM
- I/O register space

Cartridge ROM and external RAM should usually live under the cartridge subsystem, even if the bus exposes them.

### Cartridge

Cartridge behavior matters early on because many games depend on MBC logic.

Responsibilities:

- ROM loading
- cartridge header parsing
- MBC selection
- ROM banking
- RAM banking
- save-backed memory behavior later

### PPU

The PPU should generate a normal framebuffer independent of terminal presentation.

Responsibilities:

- LCD timing modes
- scanline progression
- background and window rendering
- sprite rendering
- palette application
- framebuffer output
- VBlank and STAT interrupt conditions

Do not tie this directly to terminal output.

### APU

Audio can be stubbed at first, but the boundary is still useful.

Responsibilities later:

- channel state
- mixer behavior
- register-backed audio state

### DMA

For the original Game Boy, the main DMA concern early on is OAM DMA.

Responsibilities:

- DMA trigger behavior
- OAM copy execution
- bus interaction rules during transfer later

### Timers

Responsibilities:

- DIV progression
- TIMA increment behavior
- TMA reload behavior
- TAC control bits
- timer interrupt generation

### Interrupts

Keep interrupt state centralized.

Responsibilities:

- IF / IE / IME behavior
- pending interrupt evaluation
- CPU interrupt signaling

### Joypad

Input is part of the hardware model, not just the frontend.

Responsibilities:

- button state
- joypad register behavior
- interrupt triggering when input changes

## Frontend Strategy

### Phase 1: CLI + Debugger

Start with a command-line and debugger-oriented frontend.

Possible commands:

- load ROM
- run
- step
- break
- inspect registers
- inspect memory
- dump framebuffer

Possible libraries:

- `clap` for CLI
- `crossterm` and `ratatui` for terminal UI

This is the highest-value frontend early on.

### Phase 2: Terminal Framebuffer Preview

Once the PPU can produce a framebuffer, provide a minimal visualization path.

Simplest route:

- scale or map the framebuffer to terminal cells
- use ANSI grayscale or limited color
- use Unicode half blocks for better density

This is useful for validation even if it is not yet comfortable for full gameplay.

### Phase 3: Full Terminal Renderer

After the emulator is stable enough, improve presentation:

- double buffering
- partial redraws
- terminal capability detection
- optional enhanced graphics protocols later

## Development Sequence

### Stage 1: Skeleton and Core State

Build:

- machine struct
- CPU register file
- basic memory regions
- bus read/write API

Do not build terminal graphics yet.

### Stage 2: CPU Execution Foundation

Build:

- fetch/decode/execute loop
- flags and branch behavior
- instruction timing return values
- interrupt check points

At this stage, correctness matters more than breadth.

### Stage 3: ROM Loading and Cartridge Behavior

Build:

- ROM loading
- cartridge header parsing
- no-MBC support first
- MBC1 later

This enables meaningful execution quickly.

### Stage 4: Timers, Interrupts, and DMA

Build:

- DIV/TIMA/TMA/TAC
- interrupt controller
- OAM DMA

This is where cycle accounting starts paying off.

### Stage 5: PPU and Framebuffer

Build:

- LCD timing
- scanline progression
- framebuffer generation
- basic background rendering first

Treat framebuffer production as a core responsibility and terminal display as a separate step.

### Stage 6: Debugger Frontend

Build:

- register view
- disassembly or instruction trace view
- memory inspector
- stepping controls

This should make further emulator work much easier.

### Stage 7: Terminal Playability

Build:

- visual renderer
- input mapping
- frame pacing

This should happen only once the emulator core is coherent.

## Testing Strategy

This kind of project is difficult to debug without tight feedback loops.

Useful testing layers:

- unit tests for bit manipulation and flag logic
- CPU instruction tests
- memory map tests
- timer and interrupt tests
- cartridge banking tests
- golden tests for known execution traces

Also useful:

- instruction trace logging
- register diff logging
- deterministic stepping APIs

If possible, design the emulator so that one instruction or one scanline can be stepped deterministically.

## Common Pitfalls

### Mixing Frontend and Core Too Early

If terminal rendering code reaches into emulator internals directly, the design will become harder to maintain and reason about.

### Ignoring Timing Until Later

Even on the original Game Boy, timing matters. The architecture should leave room for cycle-based execution from the start.

### Over-Building the Renderer First

A good terminal renderer cannot rescue an unstable CPU or memory model.

### Allowing Unstructured Memory Access

Direct reads and writes into random subsystem arrays bypass important hardware behavior and make bugs harder to track.

## Initial Milestone

A good first milestone is not "run games in the terminal."

A better first milestone is:

- load a ROM
- execute instructions
- inspect CPU state in a simple terminal interface
- produce deterministic logs while stepping

Once this works, the rest of the emulator becomes much easier to build confidently.

## Summary

The emulator should be built as a reusable hardware core with a terminal-oriented frontend layered on top.

Recommended path:

- cycle-aware emulation core
- debugger-first terminal workflow
- framebuffer-to-terminal rendering later

That design gives the best combination of correctness, visibility, and maintainability for a Game Boy emulator project in Rust.
