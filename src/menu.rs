// src/menu.rs
//! Windows-Style Graphical Desktop Environment for RustOS.
//! Features desktop icons, a Lord Ganesha artwork wallpaper tribute,
//! live system tray with network connectivity and hardware compatibility status,
//! and an RTC real-time clock.

use crate::vga::{self, clear_screen, set_cursor, write_at, write_char_at};
use crate::keyboard;
use crate::rtc;
use crate::pit;
use crate::shell;
use crate::snake;
use crate::snakes_and_ladders;
use crate::car_dodge;
use crate::deadlock_lab;
use crate::fs;
use crate::net;
use crate::hw;
use crate::println;

struct DesktopItem {
    name: &'static str,
    desc: &'static str,
}

const ITEMS: &[DesktopItem] = &[
    DesktopItem { name: "[1] 💻 Terminal   ", desc: "Command Shell & Utilities" },
    DesktopItem { name: "[2] 🌐 Network    ", desc: "Network Connectivity & Ping" },
    DesktopItem { name: "[3] ⚙️ Hardware   ", desc: "Hardware Compatibility Matrix" },
    DesktopItem { name: "[4] 📁 File Mgr   ", desc: "In-Memory Virtual Storage" },
    DesktopItem { name: "[5] 🔬 DeadlockLab", desc: "Research & Matrix Detection" },
    DesktopItem { name: "[6] 🎮 Snake Game ", desc: "Classic Arcade Snake" },
    DesktopItem { name: "[7] 🎲 SnakesLadd ", desc: "Snakes & Ladders Board" },
    DesktopItem { name: "[8] 🏎️ Car Dodge  ", desc: "Fast-Paced Obstacle Dodge" },
    DesktopItem { name: "[9] ℹ️ About OS   ", desc: "RustOS System Information" },
    // DesktopItem { name: "[0] 🔄 Reboot VM  ", desc: "Restart Keyboard Controller" }, // Reboot option removed per user request
];

pub fn run() {
    let mut selected = 0usize;
    let mut last_sec = 255u8;
    draw_desktop(selected);

    loop {
        // Read clock every loop for live taskbar update
        let (_, _, _, hour, min, sec) = rtc::read_datetime();
        if sec != last_sec {
            last_sec = sec;
            draw_taskbar(hour, min, sec);
        }

        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0x48 => { // Up arrow
                    if selected > 0 { selected -= 1; } else { selected = ITEMS.len() - 1; }
                    draw_desktop(selected);
                }
                0x50 => { // Down arrow
                    if selected + 1 < ITEMS.len() { selected += 1; } else { selected = 0; }
                    draw_desktop(selected);
                }
                // Number hotkeys 1-9 and 0
                0x02 => { launch_item(0); draw_desktop(selected); }
                0x03 => { launch_item(1); draw_desktop(selected); }
                0x04 => { launch_item(2); draw_desktop(selected); }
                0x05 => { launch_item(3); draw_desktop(selected); }
                0x06 => { launch_item(4); draw_desktop(selected); }
                0x07 => { launch_item(5); draw_desktop(selected); }
                0x08 => { launch_item(6); draw_desktop(selected); }
                0x09 => { launch_item(7); draw_desktop(selected); }
                0x0A => { launch_item(8); draw_desktop(selected); }
                0x0B => { launch_item(9); draw_desktop(selected); }

                0x1C => { // Enter
                    launch_item(selected);
                    draw_desktop(selected);
                }
                0x01 => { // Esc -> quick drop to shell
                    shell::run();
                    draw_desktop(selected);
                }
                _ => {}
            }
        }
    }
}

fn launch_item(index: usize) {
    match index {
        0 => shell::run(),
        1 => net::run(),
        2 => hw::run(),
        3 => {
            clear_screen();
            write_at(1, 2, "📁 RustOS File System Storage - In-Memory Volume", 0x1F);
            fs::print_ls();
            println!("\nPress any key to return to Desktop...");
            loop {
                if keyboard::pop_scancode().is_some() { break; }
            }
        }
        4 => deadlock_lab::run(),
        5 => snake::run(),
        6 => snakes_and_ladders::run(),
        7 => car_dodge::run(),
        8 => {
            clear_screen();
            write_at(2, 4, "=== RustOS Desktop Edition 0.1 ===", 0x0E);
            write_at(4, 4, "Built in 32-bit Protected Mode Rust (no_std)", 0x0F);
            write_at(5, 4, "Target Virtual Sandbox: Oracle VirtualBox", 0x0B);
            write_at(7, 4, "Features:", 0x0E);
            write_at(8, 6, "* Full Graphical Desktop with Wallpaper & Icons", 0x07);
            write_at(9, 6, "* Real-Time Network Connectivity & e1000 Driver", 0x07);
            write_at(10, 6, "* 100% Hardware Compatibility & Diagnostics", 0x07);
            write_at(11, 6, "* 20+ Shell Commands, In-Memory File System", 0x07);
            write_at(12, 6, "* 3 Playable Games & Deadlock Detection Lab", 0x07);
            write_at(15, 4, "Press any key to return to Desktop...", 0x0A);
            loop {
                if keyboard::pop_scancode().is_some() { break; }
            }
        }
        // 9 => unsafe { vga::outb(0x64, 0xFE); }, // Reboot option removed
        _ => {}
    }
}

fn draw_desktop(selected: usize) {
    clear_screen();

    // Top Desktop Banner
    for col in 0..vga::WIDTH {
        write_char_at(0, col, ' ', 0x1F); // Dark blue header
    }
    write_at(0, 2, "🪟 RustOS Desktop [v0.1 Professional] | Oracle VirtualBox Edition", 0x1F);

    // Desktop divider column
    for r in 1..23 {
        write_char_at(r, 22, '|', 0x08);
    }

    // Left Column: Desktop Icons
    write_at(1, 2, "DESKTOP ICONS", 0x0B);
    write_at(2, 2, "-------------", 0x08);

    for (i, item) in ITEMS.iter().enumerate() {
        let row = 3 + (i * 2);
        if row < 23 {
            let (bg_color, desc_color) = if i == selected {
                (0x3F, 0x3E) // Bright White on Cyan / highlight
            } else {
                (0x07, 0x08) // Light Grey on Black
            };

            write_at(row, 2, item.name, bg_color);
        }
    }

    // Center Wallpaper: Lord Ganesha Artwork Tribute
    let wallpaper_color = 0x0C; // Light Red / Saffron
    let gold_color = 0x0E;      // Gold / Yellow
    let white_color = 0x0F;

    write_at(2, 36, "   ___      ___   ", gold_color);
    write_at(3, 36, "  /   \\____/   \\  ", gold_color);
    write_at(4, 36, " /  _        _  \\ ", gold_color);
    write_at(5, 36, "|  (o)  /\\  (o)  |", gold_color);
    write_at(6, 36, "|      /  \\      |", wallpaper_color);
    write_at(7, 36, " \\    / || \\    / ", wallpaper_color);
    write_at(8, 36, "  \\  /  ||  \\  /  ", wallpaper_color);
    write_at(9, 36, "   \\/   ||   \\/   ", wallpaper_color);
    write_at(10, 36, "   /    ||    \\   ", wallpaper_color);
    write_at(11, 36, "  |    _||_    |  ", wallpaper_color);
    write_at(12, 36, "   \\  (____)  /   ", wallpaper_color);
    write_at(13, 36, "    \\________/    ", wallpaper_color);

    write_at(14, 30, "ॐ  श्री गणेशाय नमः  ॐ", gold_color);
    write_at(15, 27, "LORD GANESHA - BLESSINGS OF WISDOM", white_color);

    // System Status Card on the Desktop
    write_at(17, 25, "+----------------------------------------------------+", 0x09);
    write_at(18, 25, "| 🌐 Network Connectivity : 10.0.2.15 (Intel e1000)  |", 0x0A);
    write_at(19, 25, "| ⚡ Hardware Compatibility: 100% VirtualBox (128 MB)|", 0x0E);
    write_at(20, 25, "| 💡 Selected:                                       |", 0x07);
    write_at(20, 40, ITEMS[selected].desc, 0x0F);
    write_at(21, 25, "+----------------------------------------------------+", 0x09);

    // Initial taskbar render
    let (_, _, _, hour, min, sec) = rtc::read_datetime();
    draw_taskbar(hour, min, sec);

    set_cursor(24, 79);
}

fn draw_taskbar(hour: u8, min: u8, sec: u8) {
    // Row 23: Taskbar Divider
    for c in 0..vga::WIDTH {
        write_char_at(23, c, '=', 0x08);
    }

    // Row 24: Windows-Style Taskbar
    for c in 0..vga::WIDTH {
        write_char_at(24, c, ' ', 0x70); // Black on Light Gray taskbar
    }

    // Start Button
    write_at(24, 1, "[ 🪟 Start ]", 0x1F); // White on Blue Start Button

    // Active Task
    write_at(24, 14, "| Desktop (Active) |", 0x70);

    // System Tray Statuses
    let net_status = net::get_status_short();
    write_at(24, 36, net_status, 0x72); // Green text on taskbar

    let hw_status = hw::get_status_short();
    write_at(24, 61, hw_status, 0x71); // Blue text on taskbar

    // Clock
    let mut time_buf = [b' '; 8];
    time_buf[0] = b'0' + (hour / 10);
    time_buf[1] = b'0' + (hour % 10);
    time_buf[2] = b':';
    time_buf[3] = b'0' + (min / 10);
    time_buf[4] = b'0' + (min % 10);
    time_buf[5] = b':';
    time_buf[6] = b'0' + (sec / 10);
    time_buf[7] = b'0' + (sec % 10);
    if let Ok(time_str) = core::str::from_utf8(&time_buf) {
        write_at(24, 72, time_str, 0x70);
    }
}
