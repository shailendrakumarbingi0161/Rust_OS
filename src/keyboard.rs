// src/keyboard.rs
//! Minimal keyboard driver.
//! Provides `init` and `pop_scancode`.

use crate::vga::inb;

pub fn init() {
}

pub fn pop_scancode() -> Option<u8> {
    unsafe {
        if (inb(0x64) & 0x01) != 0 {
            Some(inb(0x60))
        } else {
            None
        }
    }
}
