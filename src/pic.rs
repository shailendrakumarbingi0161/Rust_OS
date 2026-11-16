// src/pic.rs
//! Minimal Programmable Interrupt Controller (PIC) stub.
//! Provides a `remap` function used by the kernel entry point.

pub fn remap(_offset1: u8, _offset2: u8) {
    // In a real kernel we would reprogram the PIC offsets.
    // For now this is a no‑op placeholder.
}
