// src/rtc.rs
//! Simple Real‑Time Clock (RTC) driver using CMOS ports 0x70/0x71.
//! Provides `read_datetime()` returning a `(year, month, day, hour, minute, second)` tuple.
//! All reads are done with interrupts disabled to avoid race conditions.

use core::arch::asm;

// CMOS ports
const CMOS_ADDRESS: u16 = 0x70;
const CMOS_DATA: u16 = 0x71;

// Register numbers
const REG_SECONDS: u8 = 0x00;
const REG_MINUTES: u8 = 0x02;
const REG_HOURS: u8   = 0x04;
const REG_DAY: u8     = 0x07;
const REG_MONTH: u8   = 0x08;
const REG_YEAR: u8    = 0x09;

// SAFETY: Direct port I/O; the ports exist on x86 PCs and are safe on the emulated hardware.
unsafe fn port_out(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
}
unsafe fn port_in(port: u16) -> u8 {
    let val: u8;
    asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack, preserves_flags));
    val
}

/// Read a single CMOS register (interrupts should be disabled by caller).
unsafe fn read_register(reg: u8) -> u8 {
    // Set the register number (disable NMI by setting highest bit to 0).
    port_out(CMOS_ADDRESS, reg);
    port_in(CMOS_DATA)
}

/// Convert a BCD‑encoded byte to binary.
fn bcd_to_bin(val: u8) -> u8 {
    ((val >> 4) * 10) + (val & 0x0F)
}

/// Read the current date‑time from the RTC.
/// Returns `(year, month, day, hour, minute, second)` where year is 20xx.
pub fn read_datetime() -> (u16, u8, u8, u8, u8, u8) {
    // Disable interrupts while reading to avoid inconsistent values.
    unsafe { asm!("cli"); }
    let sec = bcd_to_bin(unsafe { read_register(REG_SECONDS) });
    let min = bcd_to_bin(unsafe { read_register(REG_MINUTES) });
    let hour_raw = unsafe { read_register(REG_HOURS) };
    // Bit 6 of hour register indicates 12/24‑hour mode.
    let hour = if hour_raw & 0x80 != 0 {
        // 12‑hour mode: bit 5 = PM flag.
        let mut h = bcd_to_bin(hour_raw & 0x7F);
        if hour_raw & 0x20 != 0 { // PM
            if h != 12 { h += 12; }
        } else if h == 12 {
            h = 0; // midnight
        }
        h
    } else {
        bcd_to_bin(hour_raw)
    };
    let day = bcd_to_bin(unsafe { read_register(REG_DAY) });
    let month = bcd_to_bin(unsafe { read_register(REG_MONTH) });
    let year = bcd_to_bin(unsafe { read_register(REG_YEAR) }) as u16 + 2000;
    unsafe { asm!("sti"); }
    (year, month, day, hour, min, sec)
}

/// Debug helper – prints the current date‑time to VGA.
pub fn print_datetime() {
    let (y, m, d, h, mi, s) = read_datetime();
    vga_println!("[RTC] {}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, h, mi, s);
}
