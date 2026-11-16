// src/menu.rs
//! Simple desktop/menu UI.
//! Displays a list of applications and allows navigation with the arrow keys.
//! Up/Down arrows move the selection, Enter launches (currently prints a placeholder),
//! Esc returns to the menu (or exits the VM when called from the shell).
//! The status bar at the bottom shows the current time (from the RTC) and the selected
//! application name.

use crate::vga::{self, Writer, clear_screen, set_cursor, CURSOR_POS};
use crate::keyboard;
use crate::rtc;
use crate::pit;
use core::fmt::Write;
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::vec::Vec;

const APPS: &[&str] = &[
    "Shell",
    "Snake",
    "Snake and Ladders",
    "Car Dodge",
    "Deadlock Detection Lab",
    "Files",
    "About",
];

const MAX_APPS: usize = 7;
const STATUS_ROW: usize = 24; // zero‑based index (0‑24 rows)

// Simple ring buffer for command history – used by the shell later.
static HISTORY: [Option<[u8; 64]>; 16] = [None; 16];
static HIST_IDX: AtomicUsize = AtomicUsize::new(0);

/// Draw the menu with the current selection highlighted in cyan (attribute 0x0B).
fn draw_menu(selected: usize) {
    unsafe { clear_screen(); }
    for (i, app) in APPS.iter().enumerate() {
        let attr = if i == selected { 0x0B } else { 0x07 };
        let mut writer = Writer::new(attr);
        let _ = writeln!(writer, "{}", app);
    }
    draw_status(selected);
}

/// Draw the status bar on the last VGA row.
fn draw_status(selected: usize) {
    // Get current time.
    let (_year, _month, _day, hour, min, sec) = rtc::read_datetime();
    // Build status string.
    let status = alloc::format!(
        "[{:02}:{:02}:{:02}] {}",
        hour, min, sec, APPS[selected]
    );
    // Position cursor to the beginning of the last row.
    set_cursor(STATUS_ROW, 0);
    let mut writer = Writer::new(0x0E); // yellow on black
    let _ = writer.write_str(&status);
    // Fill rest of line with spaces.
    let remaining = vga::WIDTH - status.len();
    for _ in 0..remaining {
        let _ = writer.write_char(' ');
    }
    // Restore cursor to where the menu ended.
    set_cursor(APPS.len(), 0);
}

use crate::shell;
use crate::snake;
use crate::snakes_and_ladders;
use crate::car_dodge;
use crate::deadlock_lab;
use crate::fs;

/// Launch the selected application.
fn launch_app(selected: usize) {
    match selected {
        0 => {
            // Shell – return to shell loop
            shell::run();
        }
        1 => {
            snake::run();
        }
        2 => {
            snakes_and_ladders::run();
        }
        3 => {
            car_dodge::run();
        }
        4 => {
            // Deadlock Detection Lab – launch the lab UI
            deadlock_lab::run();
        }
        5 => {
            // Files – just list files
            fs::print_ls();
            pit::sleep_ms(1500);
        }
        6 => {
            // About – short message
            println!("RustOS – educational 32‑bit OS. Version 0.1.");
            pit::sleep_ms(1500);
        }
        _ => {}
    }
    // After app returns, redraw the menu.
    draw_menu(selected);
}

pub fn run() {
    let mut selected = 0usize;
    draw_menu(selected);
    let mut prefix_e0 = false; // track 0xE0 prefix for extended keys
    loop {
        if let Some(sc) = keyboard::pop_scancode() {
            // Handle extended keys.
            if prefix_e0 {
                match sc {
                    0x48 => { // Up arrow
                        if selected > 0 { selected -= 1; }
                        draw_menu(selected);
                    }
                    0x50 => { // Down arrow
                        if selected + 1 < APPS.len() { selected += 1; }
                        draw_menu(selected);
                    }
                    _ => {}
                }
                prefix_e0 = false;
                continue;
            }
            match sc {
                0xE0 => { prefix_e0 = true; }
                0x1C => { // Enter – launch selected app
                    launch_app(selected);
                }
                0x01 => { // Esc – exit menu (return to shell)
                    // Clear screen and return.
                    unsafe { clear_screen(); }
                    return;
                }
                _ => {}
            }
        }
        // Update the status bar once per second.
        if pit::ticks() % 100 == 0 {
            draw_status(selected);
        }
    }
}
