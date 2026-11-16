# RustOS – A Mini Operating System Built with Rust

## Overview

RustOS is a mini operating system developed using the Rust programming language. This project explores fundamental operating system concepts, low-level programming, memory management, interrupts, keyboard input, and hardware interaction.

The project is designed to understand how an operating system works internally, from booting to executing basic system-level operations.

## Features

- Custom OS boot process
- Kernel development using Rust
- Memory allocation
- Global Descriptor Table (GDT)
- Interrupt Descriptor Table (IDT)
- Keyboard input handling
- VGA text-mode display
- Programmable Interval Timer (PIT)
- Real-Time Clock (RTC)
- Serial communication
- Basic filesystem functionality
- Deadlock demonstration
- Snake game
- Car Dodge game
- Snakes and Ladders game

## Technologies Used

- Rust
- Assembly Language
- Cargo
- GRUB Bootloader
- Docker
- QEMU / VirtualBox
- PowerShell

## Project Structure

```text
Rust_OS/
├── .cargo/
├── docs/
├── iso/
│   └── boot/
│       └── grub/
├── scripts/
├── src/
│   ├── allocator.rs
│   ├── boot.S
│   ├── exceptions.rs
│   ├── gdt.rs
│   ├── idt.rs
│   ├── interrupts.S
│   ├── keyboard.rs
│   ├── main.rs
│   ├── memory-related modules
│   └── other kernel modules
├── tests/
│   └── host_tests/
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
├── README.md
└── rust-toolchain.toml
```

## Requirements

- Rust toolchain
- Cargo
- Docker
- VirtualBox
- GRUB utilities (depending on build environment)

## Building the Project

Clone the repository:

```bash
git clone https://github.com/shailendrakumarbingi0161/Rust_OS.git
cd Rust_OS
```

Build using Docker:

```bash
docker build -t rustos-builder .
docker run --rm -v "${PWD}:/src" -w /src rustos-builder /src/scripts/build.sh
```

Alternatively, on Windows PowerShell, use the provided build script:

```powershell
.\scripts\build.ps1
```

## Running the OS

Follow the instructions in:

- `docs/RUN_VIRTUALBOX.md`
- `docs/ACCEPTANCE.md`

## Documentation

Additional project documentation is available in the `docs/` directory.

## Learning Objectives

- Understand operating system architecture.
- Explore kernel development using Rust.
- Learn low-level hardware interaction.
- Understand memory management and interrupts.
- Demonstrate operating system concepts through practical implementations.

## Author

**Bingi Shailendra**

## License

This project is intended for educational and learning purposes.