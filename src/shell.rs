//! Full-featured shell for RustOS.
//! Supports the command set listed in the specification. Uses the in-memory
//! file system, RTC, PIT, and hardware ports. Implements a 16-entry command
//! history buffer.

extern crate alloc;

use crate::keyboard;
use crate::vga::{self, clear_screen, set_cursor, CURSOR_POS};
use crate::{fs, rtc, pit};
use core::fmt::Write;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

const MAX_LINE: usize = 64;
const HIST_SIZE: usize = 16;
static mut HIST_BUF: [[u8; MAX_LINE]; HIST_SIZE] = [[0; MAX_LINE]; HIST_SIZE];
static HIST_POS: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
static HIST_CURSOR: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

fn print_help() {
    vga_println!("Supported commands:");
    vga_println!("  help   - this list");
    vga_println!("  clear  - clear screen");
    vga_println!("  reboot - reset the machine (keyboard controller)");
    vga_println!("  shutdown - halt the CPU (VM must be powered off manually)");
    vga_println!("  date   - print current date and time");
    vga_println!("  cal    - show calendar for current month");
    vga_println!("  echo <text>            - print text");
    vga_println!("  echo <text> > <file>   - write text to file");
    vga_println!("  ls     - list files");
    vga_println!("  cat <file> - display file contents");
    vga_println!("  lp <file> - print file with placeholder header");
    vga_println!("  whoami - prints \"root\"");
    vga_println!("  uname  - prints OS name and architecture");
    vga_println!("  hostname - prints rustos");
    vga_println!("  uptime - time since boot");
    vga_println!("  bc <n> <op> <n> - integer calculator");
    vga_println!("  grab <word> <file> - print lines containing word");
    vga_println!("  rm <file> - delete file");
    vga_println!("  touch <file> - create empty file");
    vga_println!("  cp <src> <dst> - copy file");
    vga_println!("  mv <src> <dst> - rename file");
    vga_println!("  cut <start> <end> <file> - column slice");
    vga_println!("  head <file> [n] - first n lines");
    vga_println!("  tail <file> [n] - last n lines");
    vga_println!("  chmod <6|4> <file> - set permissions");
    vga_println!("  wc <file> - line/word/byte count");
    vga_println!("  vi <file> - simple line editor");
    vga_println!("  net / ifconfig - show Network Connectivity & Diagnostics");
    vga_println!("  hw / sysinfo   - show Hardware Compatibility Matrix");
    vga_println!("  desktop / menu - return to Graphical Desktop UI");
    vga_println!("  games - return to desktop menu");
    vga_println!("  research / deadlock - launch Deadlock Detection Lab");
}

fn add_history(line: &str) {
    let idx = HIST_POS.load(core::sync::atomic::Ordering::Relaxed) % HIST_SIZE;
    unsafe {
        let mut buf = [0u8; MAX_LINE];
        let bytes = line.as_bytes();
        let len = core::cmp::min(bytes.len(), MAX_LINE - 1);
        buf[..len].copy_from_slice(&bytes[..len]);
        buf[len] = 0;
        HIST_BUF[idx] = buf;
    }
    HIST_POS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    HIST_CURSOR.store(HIST_POS.load(core::sync::atomic::Ordering::Relaxed), core::sync::atomic::Ordering::Relaxed);
}

fn fetch_history(offset: isize) -> Option<String> {
    let cur = HIST_CURSOR.load(core::sync::atomic::Ordering::Relaxed) as isize;
    let target = cur + offset;
    if target < 0 || target >= HIST_POS.load(core::sync::atomic::Ordering::Relaxed) as isize {
        return None;
    }
    let idx = (target as usize) % HIST_SIZE;
    unsafe {
        let slice = &HIST_BUF[idx];
        let end = slice.iter().position(|&b| b == 0).unwrap_or(MAX_LINE);
        Some(core::str::from_utf8(&slice[..end]).unwrap_or("").to_string())
    }
}

fn run_command(cmd: &str) {
    if !cmd.trim().is_empty() {
        add_history(cmd.trim());
    }
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() { return; }
    match parts[0] {
        "help" => print_help(),
        "clear" => { clear_screen(); },
        "reboot" => unsafe { vga::outb(0x64, 0xFE); },
        "shutdown" => { vga_println!("System halted. You can now power off the virtual machine."); loop { unsafe { core::arch::asm!("hlt"); } } },
        "date" => {
            let (y,m,d,h,mi,s) = rtc::read_datetime();
            vga_println!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y,m,d,h,mi,s);
        },
        "cal" => {
            let (_, _, day, _, _, _) = rtc::read_datetime();
            vga_println!("Mo Tu We Th Fr Sa Su");
            let mut pos = 0usize;
            for _ in 0..(day as usize % 7) { print!("   "); pos+=1; }
            for d in 1..=31 {
                if d == day as usize { print!("[{:2}]", d); } else { print!(" {:2} ", d); }
                pos+=1;
                if pos % 7 == 0 { vga_println!(); }
            }
            vga_println!();
        },
        "ls" => fs::print_ls(),
        "cat" => {
            if parts.len() != 2 { fs::print_error("Usage: cat <file>"); return; }
            match fs::read_file(parts[1]) {
                Ok(data) => {
                    if let Ok(s) = core::str::from_utf8(data) { vga_println!("{}", s); }
                    else { for b in data { print!("{:02X} ", b); } vga_println!(); }
                },
                Err(e) => fs::print_error(e),
            }
        },
        "lp" => {
            if parts.len() != 2 { fs::print_error("Usage: lp <file>"); return; }
            vga_println!("[lp] no printer attached");
            match fs::read_file(parts[1]) {
                Ok(data) => { if let Ok(s) = core::str::from_utf8(data) { vga_println!("{}", s); } },
                Err(e) => fs::print_error(e),
            }
        },
        "whoami" => vga_println!("root"),
        "helloworld" => vga_println!("helloworld"),
        "uname" => vga_println!("RustOS 0.1 i686"),
        "hostname" => vga_println!("rustos"),
        "uptime" => {
            let secs = pit::ticks() / 100;
            let h = secs / 3600;
            let m = (secs % 3600) / 60;
            let s = secs % 60;
            vga_println!("{:02}:{:02}:{:02}", h, m, s);
        },
        "bc" => {
            if parts.len() != 4 { fs::print_error("Usage: bc <n> <op> <n>"); return; }
            let a: i64 = parts[1].parse().unwrap_or(0);
            let b: i64 = parts[3].parse().unwrap_or(0);
            let res = match parts[2] {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => { if b == 0 { fs::print_error("divide by zero"); return; } else { a / b } },
                "%" => { if b == 0 { fs::print_error("divide by zero"); return; } else { a % b } },
                _ => { fs::print_error("unsupported operator"); return; },
            };
            vga_println!("{}", res);
        },
        "grab" => {
            if parts.len() != 3 { fs::print_error("Usage: grab <word> <file>"); return; }
            match fs::read_file(parts[2]) {
                Ok(data) => {
                    if let Ok(text) = core::str::from_utf8(data) {
                        for line in text.lines() { if line.contains(parts[1]) { vga_println!("{}", line); } }
                    }
                },
                Err(e) => fs::print_error(e),
            }
        },
        "rm" => {
            if parts.len() != 2 { fs::print_error("Usage: rm <file>"); return; }
            match fs::delete_file(parts[1]) { Ok(_) => vga_println!("Deleted {}", parts[1]), Err(e) => fs::print_error(e) }
        },
        "touch" => {
            if parts.len() != 2 { fs::print_error("Usage: touch <file>"); return; }
            match fs::create_file(parts[1], b"", true) { Ok(_) => vga_println!("Created {}", parts[1]), Err(e) => fs::print_error(e) }
        },
        "cp" => {
            if parts.len() != 3 { fs::print_error("Usage: cp <src> <dst>"); return; }
            match fs::read_file(parts[1]) {
                Ok(data) => match fs::create_file(parts[2], data, true) { Ok(_) => vga_println!("Copied {} to {}", parts[1], parts[2]), Err(e) => fs::print_error(e) },
                Err(e) => fs::print_error(e),
            }
        },
        "mv" => {
            if parts.len() != 3 { fs::print_error("Usage: mv <src> <dst>"); return; }
            if fs::find_index(parts[2]).is_some() { fs::print_error("destination exists"); return; }
            match fs::read_file(parts[1]) {
                Ok(data) => {
                    match fs::delete_file(parts[1]) {
                        Ok(_) => match fs::create_file(parts[2], data, true) { Ok(_) => vga_println!("Renamed {} to {}", parts[1], parts[2]), Err(e) => fs::print_error(e) },
                        Err(e) => fs::print_error(e),
                    }
                },
                Err(e) => fs::print_error(e),
            }
        },
        "cut" => {
            if parts.len() != 4 { fs::print_error("Usage: cut <start> <end> <file>"); return; }
            let start: usize = parts[1].parse().unwrap_or(1);
            let end: usize = parts[2].parse().unwrap_or(start);
            match fs::read_file(parts[3]) {
                Ok(data) => {
                    if let Ok(text) = core::str::from_utf8(data) {
                        for line in text.lines() {
                            let chars: Vec<char> = line.chars().collect();
                            let s = if start > 0 && start - 1 < chars.len() { start - 1 } else { chars.len() };
                            let e = if end <= chars.len() { end } else { chars.len() };
                            let slice: String = chars[s..e].iter().collect();
                            vga_println!("{}", slice);
                        }
                    }
                },
                Err(e) => fs::print_error(e),
            }
        },
        "head" => {
            if parts.len() < 2 { fs::print_error("Usage: head <file> [n]"); return; }
            let n: usize = if parts.len() == 3 { parts[2].parse().unwrap_or(10) } else { 10 };
            match fs::read_file(parts[1]) {
                Ok(data) => {
                    if let Ok(text) = core::str::from_utf8(data) {
                        for (i, line) in text.lines().enumerate() { if i >= n { break; } vga_println!("{}", line); }
                    }
                },
                Err(e) => fs::print_error(e),
            }
        },
        "tail" => {
            if parts.len() < 2 { fs::print_error("Usage: tail <file> [n]"); return; }
            let n: usize = if parts.len() == 3 { parts[2].parse().unwrap_or(10) } else { 10 };
            match fs::read_file(parts[1]) {
                Ok(data) => {
                    if let Ok(text) = core::str::from_utf8(data) {
                        let lines: Vec<&str> = text.lines().collect();
                        let start = if lines.len() > n { lines.len() - n } else { 0 };
                        for line in &lines[start..] { vga_println!("{}", line); }
                    }
                },
                Err(e) => fs::print_error(e),
            }
        },
        "chmod" => {
            if parts.len() != 3 { fs::print_error("Usage: chmod <6|4> <file>"); return; }
            let mode = match parts[1] { "6" => 6u8, "4" => 4u8, _ => { fs::print_error("mode must be 6 or 4"); return; } };
            match fs::chmod(parts[2], mode) { Ok(_) => vga_println!("Permissions of {} set to {}", parts[2], parts[1]), Err(e) => fs::print_error(e) }
        },
        "wc" => {
            if parts.len() != 2 { fs::print_error("Usage: wc <file>"); return; }
            match fs::read_file(parts[1]) {
                Ok(data) => {
                    let text = core::str::from_utf8(data).unwrap_or("");
                    let lines = text.matches('\n').count();
                    let words = text.split_whitespace().count();
                    let bytes = data.len();
                    vga_println!("{} {} {}", lines, words, bytes);
                },
                Err(e) => fs::print_error(e),
            }
        },
        "vi" => {
            if parts.len() != 2 { fs::print_error("Usage: vi <file>"); return; }
            let filename = parts[1];
            let content = match fs::read_file(filename) { Ok(d) => core::str::from_utf8(d).unwrap_or("").to_string(), Err(_) => String::new() };
            vga_println!("--- vi {} (enter :wq to save)---", filename);
            // Placeholder: no actual line editing implemented.
            let _ = fs::write_file(filename, content.as_bytes());
        },
        "games" => { crate::menu::run(); },
        "desktop" | "menu" | "gui" | "exit" => { crate::menu::run(); },
        "net" | "network" | "ifconfig" | "ping" => { crate::net::run(); },
        "hw" | "hardware" | "sysinfo" => { crate::hw::run(); },
        "research" | "deadlock" => { crate::deadlock_lab::run(); },
        "echo" => {
            if let Some(idx) = parts.iter().position(|&p| p == ">") {
                let text = parts[1..idx].join(" ");
                if idx + 1 >= parts.len() { fs::print_error("Usage: echo <text> > <file>"); return; }
                let file = parts[idx+1];
                match fs::write_file(file, text.as_bytes()) { Ok(_) => vga_println!("Wrote {} bytes to {}", text.len(), file), Err(e) => fs::print_error(e) }
            } else {
                let text = parts[1..].join(" ");
                vga_println!("{}", text);
            }
        },
        _ => fs::print_error("unknown command - type help"),
    }
}

fn scancode_to_char(sc: u8) -> Option<char> {
    match sc {
        0x02 => Some('1'), 0x03 => Some('2'), 0x04 => Some('3'), 0x05 => Some('4'),
        0x06 => Some('5'), 0x07 => Some('6'), 0x08 => Some('7'), 0x09 => Some('8'),
        0x0A => Some('9'), 0x0B => Some('0'), 0x0C => Some('-'), 0x0D => Some('='),
        0x10 => Some('q'), 0x11 => Some('w'), 0x12 => Some('e'), 0x13 => Some('r'),
        0x14 => Some('t'), 0x15 => Some('y'), 0x16 => Some('u'), 0x17 => Some('i'),
        0x18 => Some('o'), 0x19 => Some('p'), 0x1A => Some('['), 0x1B => Some(']'),
        0x1E => Some('a'), 0x1F => Some('s'), 0x20 => Some('d'), 0x21 => Some('f'),
        0x22 => Some('g'), 0x23 => Some('h'), 0x24 => Some('j'), 0x25 => Some('k'),
        0x26 => Some('l'), 0x27 => Some(';'), 0x28 => Some('\''),
        0x2C => Some('z'), 0x2D => Some('x'), 0x2E => Some('c'), 0x2F => Some('v'),
        0x30 => Some('b'), 0x31 => Some('n'), 0x32 => Some('m'), 0x33 => Some(','),
        0x34 => Some('.'), 0x35 => Some('/'),
        0x39 => Some(' '),
        _ => None,
    }
}

pub fn run() {
    let mut line: [u8; MAX_LINE] = [0; MAX_LINE];
    let mut len = 0usize;
    print_prompt();
    let mut prefix_e0 = false;
    loop {
        if let Some(sc) = keyboard::pop_scancode() {
            if prefix_e0 {
                match sc {
                    0x48 => { // Up arrow - history back
                        if let Some(prev) = fetch_history(-1) {
                            while len > 0 {
                                len -= 1;
                                unsafe {
                                    set_cursor(CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) / vga::WIDTH, (CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) % vga::WIDTH) - 1);
                                    vga::VGA_WRITER.write_char(' ').ok();
                                    set_cursor(CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) / vga::WIDTH, (CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) % vga::WIDTH) - 1);
                                }
                            }
                            let bytes = prev.as_bytes();
                            for &b in bytes {
                                if len < MAX_LINE {
                                    line[len] = b;
                                    len += 1;
                                }
                                unsafe { vga::VGA_WRITER.write_char(b as char).ok(); }
                            }
                        }
                    },
                    0x50 => { // Down arrow - history forward
                        if let Some(next) = fetch_history(1) {
                            while len > 0 {
                                len -= 1;
                                unsafe {
                                    set_cursor(CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) / vga::WIDTH, (CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) % vga::WIDTH) - 1);
                                    vga::VGA_WRITER.write_char(' ').ok();
                                    set_cursor(CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) / vga::WIDTH, (CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) % vga::WIDTH) - 1);
                                }
                            }
                            let bytes = next.as_bytes();
                            for &b in bytes {
                                if len < MAX_LINE {
                                    line[len] = b;
                                    len += 1;
                                }
                                unsafe { vga::VGA_WRITER.write_char(b as char).ok(); }
                            }
                        }
                    },
                    _ => {}
                }
                prefix_e0 = false;
                continue;
            }
            match sc {
                0xE0 => { prefix_e0 = true; }
                0x0E => { // Backspace
                    if len > 0 {
                        len -= 1;
                        unsafe {
                            set_cursor(CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) / vga::WIDTH, (CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) % vga::WIDTH) - 1);
                            vga::VGA_WRITER.write_char(' ').ok();
                            set_cursor(CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) / vga::WIDTH, (CURSOR_POS.load(core::sync::atomic::Ordering::Relaxed) % vga::WIDTH) - 1);
                        }
                    }
                },
                0x1C => { // Enter
                    vga_println!();
                    let cmd = core::str::from_utf8(&line[..len]).unwrap_or("");
                    run_command(cmd.trim());
                    len = 0;
                    print_prompt();
                },
                sc => {
                    if let Some(ch) = scancode_to_char(sc) {
                        if len < MAX_LINE {
                            line[len] = ch as u8;
                            len += 1;
                            unsafe { vga::VGA_WRITER.write_char(ch).ok(); }
                        }
                    }
                }
            }
        }
    }
}

fn print_prompt() {
    print!("rustos> ");
}
