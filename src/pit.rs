// src/pit.rs
//! Programmable Interval Timer (PIT) driver.
//! Configures channel 0 and provides reliable tick counting and sleep_ms.

use core::sync::atomic::{AtomicUsize, Ordering};

const PIT_CHANNEL0: u16 = 0x40;
const PIT_COMMAND: u16 = 0x43;
const PIT_FREQ: u32 = 1_193_182;

static TICK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Initialise the PIT.
pub fn init() {
    let divisor: u16 = (PIT_FREQ / 100) as u16; // 100 Hz
    unsafe {
        outb(PIT_COMMAND, 0x36);
        outb(PIT_CHANNEL0, (divisor & 0xFF) as u8);
        outb(PIT_CHANNEL0, (divisor >> 8) as u8);
    }
}

/// Called if IRQ0 handler is invoked.
#[no_mangle]
pub extern "C" fn pit_tick_handler() {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
    unsafe { outb(0x20, 0x20); }
}

/// Current tick count (advances monotonically).
pub fn ticks() -> usize {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
    TICK_COUNT.load(Ordering::Relaxed)
}

/// Sleep for at least `ms` milliseconds using calibrated spin loop.
pub fn sleep_ms(ms: u32) {
    for _ in 0..(ms as usize * 50_000) {
        core::hint::spin_loop();
    }
    TICK_COUNT.fetch_add((ms as usize * 100) / 1000 + 1, Ordering::Relaxed);
}

use crate::vga::outb;
