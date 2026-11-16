// src/serial.rs
//! Minimal serial driver for COM1 (0x3F8).
//! Provides `init` (optional) and `write_byte` / `write_str`.
//! Used for boot‑log output to the VirtualBox serial file.

use core::fmt::{self, Write};

const COM1_PORT: u16 = 0x3F8;

/// Write a byte to the serial port. Spins until the transmitter is ready.
pub fn write_byte(byte: u8) {
    unsafe {
        // Wait for Transmit Holding Register Empty (bit 5 of Line Status Register)
        while (inb(COM1_PORT + 5) & 0x20) == 0 {}
        outb(COM1_PORT, byte);
    }
}

/// Write a string slice.
pub fn write_str(s: &str) {
    for b in s.bytes() {
        write_byte(b);
    }
}

/// Simple `core::fmt::Write` implementation.
pub struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_str(s);
        Ok(())
    }
}

// Port I/O helpers – re‑export from vga for convenience.
use crate::vga::{inb, outb};
