# Architecture Overview

## High‑level Design
- **Kernel**: `#![no_std]` Rust code compiled for the `i686-unknown-none` target. It runs as a Multiboot2 image loaded by GRUB.
- **Boot Flow**:
  1. GRUB loads `kernel.bin`.
  2. Assembly stub (`boot.S`) sets up protected mode and jumps to Rust entry point.
  3. Kernel initializes GDT, IDT, PIC, PIT, VGA, serial, keyboard, RTC, and the simple bump allocator.
  4. In‑memory filesystem is populated with the few required files.
  5. Desktop menu (`menu::run`) is displayed. The user can launch the shell, games, or the deadlock lab.
- **Memory Layout**:
  - Load address: **1 MiB** (standard for Multiboot).
  - Bump allocator provides **64 KiB** of heap for dynamic structures (e.g., `Vec`, `String`).
- **Hardware Abstraction**:
  - **VGA Text Mode** driver (`vga.rs`) writes directly to video memory at `0xb8000`.
  - **Keyboard** driver reads scancodes via port `0x60` and queues them for the kernel.
  - **Serial** (`COM1` – port `0x3F8`) is used for boot‑time logging.
  - **PIT** (`0x40`‑`0x43`) generates a 100 Hz tick for sleep and UI refresh.
  - **PIC** remapped to avoid conflicts with CPU exceptions.
  - **RTC** (`0x70`/`0x71`) provides the current date/time for the status bar.
- **Subsystems**:
  - **Filesystem (`fs.rs`)** – a tiny in‑memory flat file system (max 16 files, 2 KiB each). Supports `open`, `read`, `write`, `delete`.
  - **Shell (`shell.rs`)** – parses and executes the complete command set from Section 6 of the spec. Includes history, color‑coded errors, and built‑in utilities (`cat`, `bc`, `cut`, `head`, `tail`, `wc`, etc.).
  - **Menu (`menu.rs`)** – draws a simple desktop UI with a status bar (time, selected app) and launches applications.
  - **Games** – three text‑mode games (`snake.rs`, `snakes_and_ladders.rs`, `car_dodge.rs`). Each runs in its own loop and returns to the menu on exit.
  - **Deadlock Lab (`deadlock_lab.rs`)** – UI for the research lab, implements the Banker's algorithm, runs predefined patterns, measures execution time, shows a chart, and writes results to `deadlock_results.txt`.

## Module Dependency Graph
```
graph TD
    boot[boot.S] --> kernel[main.rs]
    kernel --> gdt[gdt.rs]
    kernel --> idt[idt.rs]
    kernel --> pic[pic.rs]
    kernel --> pit[pit.rs]
    kernel --> vga[vga.rs]
    kernel --> serial[serial.rs]
    kernel --> keyboard[keyboard.rs]
    kernel --> rtc[rtc.rs]
    kernel --> allocator[allocator.rs]
    kernel --> fs[fs.rs]
    kernel --> menu[menu.rs]
    menu --> shell[shell.rs]
    menu --> snake[snake.rs]
    menu --> snakes_and_ladders[snakes_and_ladders.rs]
    menu --> car_dodge[car_dodge.rs]
    menu --> deadlock_lab[deadlock_lab.rs]
```

## Design Decisions
- **No_std**: All kernel code avoids the Rust standard library; only `core` and `alloc` are used.
- **In‑memory FS**: Simplicity – no disk I/O required; all files exist only for the running VM session.
- **Single‑threaded cooperative scheduling**: Each application runs until it yields back to the menu or shell.
- **ASCII graphics**: Keeps VGA text mode implementation simple and portable across VirtualBox versions.
- **Serial logging**: Allows host‑side debugging via the `build\serial.log` file.

---
*This file will be expanded with diagrams, more detailed subsystem descriptions, and references to source files as the project completes.*