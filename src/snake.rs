// src/snake.rs
//! Simple Snake game for RustOS.
//! Uses the VGA driver for rendering, the keyboard driver for input, and the PIT timer for ticks.
//! The game runs at ~10 Hz (100 ms per tick) and ends on wall or self‑collision.
//! After a game‑over the screen shows the final score and waits for Enter to restart or Esc to return to the menu.

extern crate alloc;

use crate::vga::{self, Writer, clear_screen, set_cursor, WIDTH, HEIGHT};
use crate::keyboard;
use crate::pit;
use core::fmt::Write;
use core::sync::atomic::Ordering;
use alloc::vec::Vec;

// Directions (dx, dy)
#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn dx(self) -> isize { match self { Dir::Left => -1, Dir::Right => 1, _ => 0 } }
    fn dy(self) -> isize { match self { Dir::Up => -1, Dir::Down => 1, _ => 0 } }
}

// Simple Linear Congruential Generator for pseudo‑random numbers.
struct Lcg { state: usize }
impl Lcg {
    fn new(seed: usize) -> Self { Self { state: seed } }
    fn next(&mut self) -> usize {
        // Parameters from Numerical Recipes.
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }
    fn next_range(&mut self, max: usize) -> usize { self.next() % max }
}

pub fn run() {
    // Initialise game state.
    let mut snake: Vec<(usize, usize)> = Vec::new();
    let start_x = WIDTH / 2;
    let start_y = (HEIGHT - 2) / 2; // leave last row for status
    snake.push((start_x, start_y));
    let mut dir = Dir::Right;
    let mut rng = Lcg::new(pit::ticks());
    let mut food = (rng.next_range(WIDTH), rng.next_range(HEIGHT - 1));
    let mut score: usize = 0;
    let mut game_over = false;

    while !game_over {
        // ----- INPUT -----
        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0xE0 => { // extended prefix – next scancode will be an arrow key
                    if let Some(sc2) = keyboard::pop_scancode() {
                        match sc2 {
                            0x48 => if dir != Dir::Down { dir = Dir::Up },
                            0x50 => if dir != Dir::Up { dir = Dir::Down },
                            0x4B => if dir != Dir::Right { dir = Dir::Left },
                            0x4D => if dir != Dir::Left { dir = Dir::Right },
                            _ => {}
                        }
                    }
                }
                0x01 => return, // Esc – exit to menu
                _ => {}
            }
        }

        // ----- MOVE -----
        let (head_x, head_y) = *snake.last().unwrap();
        let new_x = ((head_x as isize) + dir.dx()) as usize;
        let new_y = ((head_y as isize) + dir.dy()) as usize;

        // Collision with walls (excluding bottom status row)
        if new_x == 0 || new_x >= WIDTH - 1 || new_y == 0 || new_y >= HEIGHT - 1 {
            game_over = true;
        } else if snake.iter().any(|&(x, y)| x == new_x && y == new_y) {
            // Self‑collision
            game_over = true;
        } else {
            // Add new head
            snake.push((new_x, new_y));
            // Eat food?
            if new_x == food.0 && new_y == food.1 {
                score += 1;
                // Place new food at an empty cell
                loop {
                    let fx = rng.next_range(WIDTH);
                    let fy = rng.next_range(HEIGHT - 1);
                    if !snake.iter().any(|&(x, y)| x == fx && y == fy) {
                        food = (fx, fy);
                        break;
                    }
                }
            } else {
                // Normal movement: remove tail
                snake.remove(0);
            }
        }

        // ----- RENDER -----
        unsafe { clear_screen(); }
        let mut writer = Writer::new(0x07);
        // Draw borders
        for x in 0..WIDTH {
            writer.set_cursor(0, x);
            writer.write_char('#').ok();
            writer.set_cursor(HEIGHT - 2, x);
            writer.write_char('#').ok();
        }
        for y in 1..(HEIGHT - 2) {
            writer.set_cursor(y, 0);
            writer.write_char('#').ok();
            writer.set_cursor(y, WIDTH - 1);
            writer.write_char('#').ok();
        }
        // Draw food
        writer.set_cursor(food.1, food.0);
        writer.write_char('*').ok();
        // Draw snake body
        for &(x, y) in &snake {
            writer.set_cursor(y, x);
            writer.write_char('o').ok();
        }
        // Score line (top row after border)
        writer.set_cursor(0, 2);
        let _ = write!(writer, "Score: {}", score);
        set_cursor(HEIGHT - 1, 0); // hide cursor

        // ----- TIMING -----
        pit::sleep_ms(100); // ~10 Hz
    }

    // ----- GAME OVER -----
    unsafe { clear_screen(); }
    vga_println!("Game Over – Score: {}", score);
    vga_println!("Press Enter to restart, Esc to return to menu.");
    loop {
        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0x1C => { run(); return; } // Enter – restart
                0x01 => return,               // Esc – exit
                _ => {}
            }
        }
    }
}
