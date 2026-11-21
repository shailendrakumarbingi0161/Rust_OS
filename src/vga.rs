// src/vga.rs – VGA text mode driver and I/O helpers

#![allow(dead_code)]

use core::arch::asm;
use core::fmt::{self, Write};
use core::sync::atomic::{AtomicUsize, Ordering};

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 25;
pub const BUFFER_ADDR: usize = 0xb8000;

pub static CURSOR_POS: AtomicUsize = AtomicUsize::new(0);

// Port I/O helpers
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    asm!("in al, dx", in("dx") port, out("al") val, options(nomem, nostack, preserves_flags));
    val
}

#[inline]
pub unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

/// Set hardware cursor position and update `CURSOR_POS`.
pub fn set_cursor(row: usize, col: usize) {
    let r = if row >= HEIGHT { HEIGHT - 1 } else { row };
    let c = if col >= WIDTH { WIDTH - 1 } else { col };
    let pos = (r * WIDTH + c) as u16;
    CURSOR_POS.store(pos as usize, Ordering::Relaxed);
    unsafe {
        outb(0x3D4, 0x0F);
        outb(0x3D5, (pos & 0xFF) as u8);
        outb(0x3D4, 0x0E);
        outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }
}

/// Clear the screen to spaces with light gray on black (0x07).
pub fn clear_screen() {
    let buf = BUFFER_ADDR as *mut u16;
    let blank = (b' ' as u16) | (0x07u16 << 8);
    for i in 0..(WIDTH * HEIGHT) {
        unsafe {
            *buf.add(i) = blank;
        }
    }
    set_cursor(0, 0);
}

/// Write a string directly at (row, col) without advancing cursor or scrolling.
pub fn write_at(row: usize, col: usize, s: &str, color: u8) {
    if row >= HEIGHT { return; }
    let buf = BUFFER_ADDR as *mut u16;
    for (i, c) in s.chars().enumerate() {
        let c_col = col + i;
        if c_col >= WIDTH { break; }
        let offset = row * WIDTH + c_col;
        unsafe {
            *buf.add(offset) = (c as u8 as u16) | ((color as u16) << 8);
        }
    }
}

/// Write a single character directly at (row, col) with color attribute.
pub fn write_char_at(row: usize, col: usize, c: char, color: u8) {
    if row >= HEIGHT || col >= WIDTH { return; }
    let buf = BUFFER_ADDR as *mut u16;
    let offset = row * WIDTH + col;
    unsafe {
        *buf.add(offset) = (c as u8 as u16) | ((color as u16) << 8);
    }
}

/// Scroll screen up by one line.
fn scroll() {
    let buf = BUFFER_ADDR as *mut u16;
    unsafe {
        core::ptr::copy(buf.add(WIDTH), buf, WIDTH * (HEIGHT - 1));
        let blank = (b' ' as u16) | (0x07u16 << 8);
        for i in 0..WIDTH {
            *buf.add((HEIGHT - 1) * WIDTH + i) = blank;
        }
    }
    set_cursor(HEIGHT - 1, 0);
}

/// Write a single character with color attribute at current cursor.
pub fn write_char_internal(c: char, color: u8) {
    if c == '\n' {
        let cur = CURSOR_POS.load(Ordering::Relaxed);
        let row = cur / WIDTH;
        if row + 1 >= HEIGHT {
            scroll();
        } else {
            set_cursor(row + 1, 0);
        }
    } else if c == '\r' {
        let cur = CURSOR_POS.load(Ordering::Relaxed);
        let row = cur / WIDTH;
        set_cursor(row, 0);
    } else if c == '\x08' {
        let cur = CURSOR_POS.load(Ordering::Relaxed);
        if cur > 0 {
            let new_pos = cur - 1;
            CURSOR_POS.store(new_pos, Ordering::Relaxed);
            unsafe {
                let buf = BUFFER_ADDR as *mut u16;
                *buf.add(new_pos) = (b' ' as u16) | ((color as u16) << 8);
            }
            set_cursor(new_pos / WIDTH, new_pos % WIDTH);
        }
    } else {
        let cur = CURSOR_POS.load(Ordering::Relaxed);
        if cur >= WIDTH * HEIGHT {
            scroll();
        }
        let cur = CURSOR_POS.load(Ordering::Relaxed);
        let entry = (c as u8 as u16) | ((color as u16) << 8);
        unsafe {
            let buf = BUFFER_ADDR as *mut u16;
            *buf.add(cur) = entry;
        }
        let next = cur + 1;
        if next >= WIDTH * HEIGHT {
            scroll();
        } else {
            set_cursor(next / WIDTH, next % WIDTH);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Writer {
    pub color: u8,
}

impl Writer {
    pub const fn new(color: u8) -> Self {
        Self { color }
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) {
        set_cursor(row, col);
    }

    pub fn write_char(&self, c: char) -> fmt::Result {
        write_char_internal(c, self.color);
        Ok(())
    }

    pub fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            write_char_internal(c, self.color);
        }
        Ok(())
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            write_char_internal(c, self.color);
        }
        Ok(())
    }
}

pub static VGA_WRITER: Writer = Writer::new(0x07);

pub fn _print(args: fmt::Arguments) {
    let mut writer = Writer::new(0x07);
    let _ = writer.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::vga::_print(format_args!("\n")));
    ($($arg:tt)*) => {{
        $crate::vga::_print(format_args!($($arg)*));
        $crate::vga::_print(format_args!("\n"));
    }};
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga::_print(format_args!("\n")));
    ($($arg:tt)*) => {{
        $crate::vga::_print(format_args!($($arg)*));
        $crate::vga::_print(format_args!("\n"));
    }};
}
