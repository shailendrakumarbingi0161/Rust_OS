// src/snakes_and_ladders.rs
//! Simple "Snakes and Ladders" game for RustOS.
//! Two players (Human vs Computer). The board is 1‑100 displayed as a 10×10 grid.
//! Fixed ladders and snakes are defined; after each dice roll the appropriate
//! jump is applied. Press **Space** or **Enter** to roll the dice, **Esc** to quit
//! back to the desktop menu.
//! The game updates the VGA screen each turn and shows the current scores.

extern crate alloc;

use crate::vga::{self, Writer, clear_screen, set_cursor, WIDTH, HEIGHT};
use crate::keyboard;
use crate::pit;
use alloc::vec::Vec;
use alloc::format;
use core::fmt::Write;

// Simple Linear Congruential Generator (same as in snake.rs).
struct Lcg { state: usize }
impl Lcg {
    fn new(seed: usize) -> Self { Self { state: seed } }
    fn next(&mut self) -> usize {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }
    fn next_range(&mut self, max: usize) -> usize { self.next() % max }
}

// Ladders: start -> end (must be higher).
const LADDERS: &[(usize, usize)] = &[
    (3, 22),
    (5, 8),
    (11, 26),
    (20, 29),
];
// Snakes: start -> end (must be lower).
const SNAKES: &[(usize, usize)] = &[
    (27, 1),
    (21, 9),
    (17, 4),
    (19, 7),
];

fn apply_snakes_ladders(pos: usize) -> usize {
    // Apply ladders first, then snakes (order does not matter because they are disjoint).
    for &(start, end) in LADDERS {
        if pos == start { return end; }
    }
    for &(start, end) in SNAKES {
        if pos == start { return end; }
    }
    pos
}

fn draw_board(p1: usize, p2: usize) {
    // Clear screen and draw a simple numeric grid.
    unsafe { clear_screen(); }
    let mut writer = Writer::new(0x07);
    // We'll print 10 rows, each with 10 cells.
    for row in 0..10 {
        // Board rows are printed from top (row 9) to bottom (row 0).
        let board_row = 9 - row;
        for col in 0..10 {
            // Compute square number.
            let mut num = board_row * 10 + col + 1;
            // Reverse direction every other row to mimic classic board.
            if board_row % 2 == 1 {
                num = board_row * 10 + (9 - col) + 1;
            }
            // Determine cell content.
            let mut cell = format!("{:3}", num);
            if num == p1 { cell = format!(" P1"); }
            else if num == p2 { cell = format!(" P2"); }
            writer.write_str(&cell).ok();
            writer.write_str(" ").ok();
        }
        writer.write_str("\n").ok();
    }
    // Show turn info later.
}

pub fn run() {
    let mut rng = Lcg::new(pit::ticks());
    let mut p1_pos = 1usize; // human
    let mut p2_pos = 1usize; // computer
    let mut turn_human = true;
    loop {
        draw_board(p1_pos, p2_pos);
        // Status line
        let mut writer = Writer::new(0x07);
        set_cursor(HEIGHT - 1, 0);
        if turn_human {
            writer.write_str("Your turn – press Space/Enter to roll, Esc to quit").ok();
        } else {
            writer.write_str("Computer's turn – rolling...").ok();
        }
        // Wait for input
        if turn_human {
            // Wait for Space (0x39) or Enter (0x1C) or Esc (0x01)
            loop {
                if let Some(sc) = keyboard::pop_scancode() {
                    match sc {
                        0x01 => return, // Esc → exit to menu
                        0x1C | 0x39 => { break; }
                        _ => {}
                    }
                }
            }
        } else {
            pit::sleep_ms(800); // short pause for computer turn
        }
        // Roll dice (1‑6)
        let dice = rng.next_range(6) + 1;
        let mut writer = Writer::new(0x07);
        set_cursor(HEIGHT - 2, 0);
        writer.write_str(&format!("Dice: {}", dice)).ok();
        // Update position
        if turn_human {
            p1_pos = (p1_pos + dice).min(100);
            p1_pos = apply_snakes_ladders(p1_pos);
        } else {
            p2_pos = (p2_pos + dice).min(100);
            p2_pos = apply_snakes_ladders(p2_pos);
        }
        // Check win condition
        if p1_pos == 100 {
            draw_board(p1_pos, p2_pos);
            let mut w = Writer::new(0x07);
            set_cursor(HEIGHT - 1, 0);
            w.write_str("You win! Press Enter to play again, Esc to quit.").ok();
            // Wait for Enter or Esc
            loop {
                if let Some(sc) = keyboard::pop_scancode() {
                    match sc {
                        0x1C => { // restart
                            p1_pos = 1; p2_pos = 1; turn_human = true; break; }
                        0x01 => return,
                        _ => {}
                    }
                }
            }
            continue;
        } else if p2_pos == 100 {
            draw_board(p1_pos, p2_pos);
            let mut w = Writer::new(0x07);
            set_cursor(HEIGHT - 1, 0);
            w.write_str("Computer wins! Press Enter to play again, Esc to quit.").ok();
            loop {
                if let Some(sc) = keyboard::pop_scancode() {
                    match sc {
                        0x1C => { p1_pos = 1; p2_pos = 1; turn_human = true; break; }
                        0x01 => return,
                        _ => {}
                    }
                }
            }
            continue;
        }
        // Switch turn
        turn_human = !turn_human;
        // Small delay before next frame
        pit::sleep_ms(500);
    }
}
