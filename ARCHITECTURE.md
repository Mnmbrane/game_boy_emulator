# Game Boy Advance Emulator Architecture

## Goal

Build a Game Boy Advance emulator in Rust with a terminal-oriented frontend, while keeping the emulation core clean enough to support debugging tools and alternate frontends later.

The main architectural constraint is that terminal rendering is a presentation problem, not an emulation problem. The emulator should therefore be designed as:

- a hardware emulation core
- a frontend layer for terminal I/O and visualization

This keeps the core reusable and avoids mixing terminal-specific compromises into CPU, memory, timing, or graphics logic.

## Recommended Direction

The recommended design is:

- an accuracy-oriented emulation core
- a debugger-first terminal interface
- a gameplay-oriented terminal renderer added later

This balances learning value, maintainability, and long-term correctness. It also avoids the common mistake of over-investing in rendering before the CPU, memory map, and timing model are stable.

## Design Options

### Option 1: Accuracy-First Core

This is the recommended architecture.

Core ideas:

- represent the hardware explicitly
- execute CPU instructions with cycle accounting
- drive peripherals from elapsed cycles
- model memory access through a central bus

Benefits:

- scales well as the emulator grows
- supports proper timing-sensitive behavior
- makes debugging easier because subsystem boundaries are clear
- allows multiple frontends without changing core logic

Costs:

- slower initial progress
- requires more up-front structure

This option is the best base if the project is meant to become a real emulator rather than a short prototype.

### Option 2: Frame-Oriented Prototype

This design runs the CPU in larger chunks and updates devices at frame boundaries or coarse checkpoints.

Benefits:

- fast to prototype
- easier to get visible output quickly

Costs:

- poor fit for GBA timing behavior
- harder to retrofit correctness later
- DMA, IRQ, waitstates, timers, and video timing become awkward

This option is useful only if the immediate goal is experimentation and not long-term correctness.

### Option 3: Headless Emulator with TUI Debugger

This option emphasizes observability first. Instead of trying to make games playable immediately, the first frontend is a terminal debugger.

Typical views:

- CPU registers and flags
- disassembly around the program counter
- memory viewer
- interrupt state
- DMA and timer state
- logs and breakpoints
- small framebuffer preview later

Benefits:

- ideal for learning and diagnosing bugs
- terminal-native from the beginning
- much easier to validate than a fully playable interface

Costs:

- not immediately game-focused
- requires some tooling work before visual payoff

This is not a replacement for the core design. It is a frontend strategy that pairs well with Option 1.

### Option 4: Terminal Graphics Frontend

This is the path for actually displaying games in a terminal.

The core still renders into a normal GBA framebuffer. The frontend converts that framebuffer into terminal output.

Possible rendering approaches:

- ANSI truecolor plus Unicode half blocks
- braille character rendering
- sixel graphics
- kitty graphics protocol

Benefits:

- preserves a clean emulator core
- allows multiple terminal rendering modes

Costs:

- output quality varies a lot by terminal
- input latency and refresh behavior may be inconsistent

This should be added only after the core can already produce a correct framebuffer.

## Recommended Combined Approach

Use:

- Option 1 for the emulator core
- Option 3 for the first frontend
- Option 4 later for terminal gameplay output

This gives a clean separation of concerns:

- core handles hardware emulation
- debugger frontend helps development
- renderer frontend handles user-facing display

## Architectural Principles

### 1. Keep the Core Frontend-Agnostic

The core should know nothing about:

- terminal escape sequences
- input libraries
- UI widgets
- terminal graphics protocols

The core should expose state and APIs that a frontend can consume.

### 2. Centralize Memory Access

All hardware-visible reads and writes should flow through a bus or memory interface.

This is important because many GBA components are memory-mapped, including:

- VRAM and palette RAM
- I/O registers
- timers
- DMA control
- interrupt registers
- cartridge ROM and SRAM/Flash

If subsystems bypass the bus too freely, correctness becomes difficult to reason about.

### 3. Advance Time Explicitly

Time should be represented in cycles, not implicit "updates."

At minimum:

- CPU executes an instruction
- instruction returns cycles consumed
- other hardware advances using those cycles

This is the foundation for making timers, interrupts, DMA, and video timing behave coherently.

### 4. Separate Orchestration from Device Logic

Each subsystem should own its own behavior, but one top-level machine object should coordinate execution.

The top-level machine can decide:

- when to run CPU
- when to tick timers
- when DMA should preempt
- when scanlines or frames complete
- when interrupts become pending

This avoids scattering global control flow across many modules.

### 5. Prefer Observable State

Since this project will likely be debugged heavily in the terminal, it is worth designing state so it can be inspected cleanly.

Examples:

- register values should be easy to dump
- CPU mode and flags should be explicit
- memory regions should be named
- scheduler or event state should be visible

## Proposed Module Layout

One reasonable layout:

```text
src/
  main.rs
  core/
    mod.rs
    gba.rs
    bus.rs
    memory.rs
    scheduler.rs
    cartridge.rs
    cpu/
      mod.rs
      arm.rs
      thumb.rs
      registers.rs
      status.rs
    ppu/
      mod.rs
      framebuffer.rs
    apu/
      mod.rs
    dma/
      mod.rs
    timers/
      mod.rs
    interrupts/
      mod.rs
  frontend/
    mod.rs
    tui.rs
    renderer.rs
```

This exact structure is optional, but the separation is important.

## Core Subsystems

### `Gba` or `Machine`

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

The GBA uses an ARM7TDMI core with ARM and THUMB modes.

CPU responsibilities:

- registers and banked registers
- CPSR and SPSR handling
- fetch, decode, execute
- mode switching
- exception entry
- cycle reporting

Important design decision:

- keep instruction execution logic separate from memory ownership
- use the bus for instruction fetches and data access

### Bus

The bus is the central routing layer for reads and writes.

Responsibilities:

- map addresses to memory regions
- enforce region behavior
- route I/O register access
- model waitstate behavior later

This should become the single source of truth for observable memory access.

### Memory

Represent the major GBA regions explicitly:

- BIOS
- EWRAM
- IWRAM
- palette RAM
- VRAM
- OAM
- I/O registers
- cartridge ROM
- save memory

This can begin as plain arrays or vectors with simple bounds logic.

### PPU

The renderer should generate a normal framebuffer independent of terminal presentation.

Responsibilities:

- scanline timing
- background/object rendering
- palette lookup
- framebuffer output
- VBlank/HBlank signaling later

Do not tie this directly to ANSI output or terminal character cells.

### APU

Audio can be stubbed initially, but the subsystem should still exist as a boundary.

Responsibilities later:

- channel state
- FIFO audio
- timer-linked sample generation

### DMA

DMA is important on GBA and affects correctness more than many first implementations expect.

Responsibilities:

- channel control
- trigger conditions
- transfer execution
- IRQ generation

DMA should eventually integrate closely with timing and bus access rules.

### Timers

Responsibilities:

- increment counters based on cycles
- handle cascades
- trigger interrupts
- support audio timing interactions later

### Interrupts

Keep interrupt state centralized.

Responsibilities:

- IME / IE / IF behavior
- pending interrupt evaluation
- CPU interrupt signaling

### Scheduler

You may or may not need an explicit scheduler early, but the concept is useful.

Possible responsibilities:

- track the next hardware event
- advance devices efficiently
- coordinate timed subsystem changes

Early versions can be simple. Over time, a scheduler helps avoid ad hoc timing logic spread everywhere.

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

- scale down the framebuffer
- map pixels to ANSI truecolor
- use Unicode half blocks for vertical density

This is useful for validation even if it is not yet comfortable for full gameplay.

### Phase 3: Full Terminal Renderer

After the emulator is stable enough, improve presentation:

- double buffering
- partial redraws
- terminal capability detection
- sixel or kitty graphics support if desired

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
- ARM and THUMB mode handling
- flags and branch behavior
- cycle return per instruction

At this stage, correctness matters more than breadth.

### Stage 3: ROM and BIOS Loading

Build:

- BIOS loading
- cartridge ROM loading
- reset/startup flow

This enables meaningful execution and early debugging.

### Stage 4: Interrupts, Timers, and DMA

Build:

- timer registers and counting
- interrupt controller
- DMA channels

This is where the architecture will start paying off.

### Stage 5: PPU and Framebuffer

Build:

- scanline timing
- framebuffer generation
- basic display modes first

Treat framebuffer production as a core responsibility and terminal display as a separate step.

### Stage 6: Debugger Frontend

Build:

- register view
- disassembly view
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

- unit tests for bit manipulation and decode logic
- CPU instruction tests
- memory map tests
- timer and interrupt tests
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

GBA behavior depends heavily on timing. Even if timing starts simplified, the architecture should leave room for cycle-based execution.

### Over-Building the Renderer First

A good terminal renderer cannot rescue an unstable CPU or memory model.

### Allowing Unstructured Memory Access

Direct reads and writes into random subsystem arrays bypass important hardware behavior and make bugs harder to track.

## Initial Milestone

A good first milestone is not "run games in the terminal."

A better first milestone is:

- load BIOS and ROM
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

That design gives the best combination of correctness, visibility, and maintainability for a GBA emulator project in Rust.
