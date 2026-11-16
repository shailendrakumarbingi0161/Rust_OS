// src/pit.rs
//! Programmable Interval Timer (PIT) driver.
//! Configures channel 0 to generate a 100 Hz interrupt (IRQ0).
//! Provides a global tick counter and a simple `sleep_ms`.

use core::sync::atomic::{AtomicUsize, Ordering};

const PIT_CHANNEL0: u16 = 0x40;
const PIT_COMMAND: u16 = 0x43;

// 1193182 Hz base frequency.
const PIT_FREQ: u32 = 1_193_182;

static TICK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Initialise the PIT to fire at 100 Hz.
pub fn init() {
    let divisor: u16 = (PIT_FREQ / 100) as u16; // ≈11931
    unsafe {
        // command: channel 0, lo/hi byte access, mode 2 (rate generator), binary mode
        outb(PIT_COMMAND, 0x36);
        outb(PIT_CHANNEL0, (divisor & 0xFF) as u8);
        outb(PIT_CHANNEL0, (divisor >> 8) as u8);
    }
    println!("[PIT] initialized (100 Hz)");
}

/// Called by the IRQ0 handler (defined in `interrupts.S`).
#[no_mangle]
pub extern "C" fn pit_tick_handler() {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
    // Send End‑Of‑Interrupt to the PIC (master port 0x20).
    unsafe { outb(0x20, 0x20); }
}

/// Current tick count (100 ticks per second).
pub fn ticks() -> usize {
    TICK_COUNT.load(Ordering::Relaxed)
}

/// Sleep for at least `ms` milliseconds (busy‑wait).
pub fn sleep_ms(ms: u32) {
    let start = ticks();
    let target = start + ((ms as usize * 100) / 1000);
    while ticks() < target {}
}

// Re‑export port I/O helpers for convenience.
use crate::vga::outb;
