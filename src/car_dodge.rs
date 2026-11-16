// src/car_dodge.rs
//! Simple Car Dodge game for RustOS.
//! Three vertical lanes, player car at the bottom row, obstacles fall from the top.
//! Arrow Left/Right moves the car between lanes. The game runs at ~10 Hz (100 ms per tick).
//! Score increases each tick survived; the high score is saved in a static variable.
//! Press **Esc** to quit back to the desktop menu.

extern crate alloc;

use crate::vga::{self, Writer, clear_screen, set_cursor, WIDTH, HEIGHT};
use crate::keyboard;
use crate::pit;
use core::fmt::Write;
use core::sync::atomic::{AtomicUsize, Ordering};

// Lanes are fixed X positions (roughly a quarter, half, three‑quarters of the screen width).
const LANES: [usize; 3] = [WIDTH / 4, WIDTH / 2, 3 * WIDTH / 4];
// Y coordinate for the player car (second‑last row, leaving the bottom row for the status line).
const CAR_Y: usize = HEIGHT - 3;

// Simple Linear Congruential Generator for pseudo‑random numbers (same parameters as other games).
struct Lcg { state: usize }
impl Lcg {
    fn new(seed: usize) -> Self { Self { state: seed } }
    fn next(&mut self) -> usize {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }
    fn next_range(&mut self, max: usize) -> usize { self.next() % max }
}

// Obstacle representation: lane index (0‑2) and current Y position.
#[derive(Clone, Copy)]
struct Obstacle { lane: usize, y: usize }

static HIGH_SCORE: AtomicUsize = AtomicUsize::new(0);

pub fn run() {
    // Initialise game state.
    let mut rng = Lcg::new(pit::ticks());
    let mut car_lane: usize = 1; // start in middle lane
    let mut obstacles: alloc::vec::Vec<Obstacle> = alloc::vec::Vec::new();
    let mut score: usize = 0;
    let mut game_over = false;

    // Main loop.
    while !game_over {
        // ----- INPUT -----
        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0xE0 => { // extended prefix – next scancode is an arrow key
                    if let Some(sc2) = keyboard::pop_scancode() {
                        match sc2 {
                            0x4B => if car_lane > 0 { car_lane -= 1 }, // Left
                            0x4D => if car_lane < 2 { car_lane += 1 }, // Right
                            _ => {}
                        }
                    }
                }
                0x01 => { // Esc – quit to menu
                    return;
                }
                _ => {}
            }
        }

        // ----- UPDATE -----
        // Move existing obstacles down.
        for obs in obstacles.iter_mut() {
            obs.y += 1;
        }
        // Remove obstacles that have moved past the car row.
        obstacles.retain(|obs| obs.y < CAR_Y);
        // Randomly spawn a new obstacle (30% chance each tick).
        if rng.next_range(100) < 30 {
            let lane = rng.next_range(3);
            obstacles.push(Obstacle { lane, y: 0 });
        }
        // Check collision with player.
        if obstacles.iter().any(|obs| obs.y == CAR_Y && obs.lane == car_lane) {
            game_over = true;
        }
        // Increment score.
        score += 1;

        // ----- RENDER -----
        unsafe { clear_screen(); }
        let mut writer = Writer::new(0x07);
        // Draw lanes as vertical lines.
        for lane_x in LANES.iter() {
            for y in 0..(HEIGHT - 1) { // leave last row for status
                set_cursor(y, *lane_x);
                writer.write_char('|').ok();
            }
        }
        // Draw obstacles as 'X'.
        for obs in obstacles.iter() {
            set_cursor(obs.y, LANES[obs.lane]);
            writer.write_char('X').ok();
        }
        // Draw player car as 'A'.
        set_cursor(CAR_Y, LANES[car_lane]);
        writer.write_char('A').ok();
        // Status line (bottom row) – show current score.
        set_cursor(HEIGHT - 1, 0);
        let _ = write!(writer, "Score: {}  High: {}  (Esc to quit)", score, HIGH_SCORE.load(Ordering::Relaxed));

        // ----- TIMING -----
        pit::sleep_ms(100); // ~10 Hz
    }

    // ----- GAME OVER SCREEN -----
    unsafe { clear_screen(); }
    vga_println!("Game Over! Final score: {}", score);
    // Update high score if needed.
    loop {
        let high = HIGH_SCORE.load(Ordering::Relaxed);
        if score > high {
            HIGH_SCORE.store(score, Ordering::Relaxed);
        }
        // Show final high score.
        vga_println!("High score: {}", HIGH_SCORE.load(Ordering::Relaxed));
        vga_println!("Press Enter to play again, Esc to return to menu.");
        // Await input.
        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0x1C => { // Enter – restart
                    run();
                    return;
                }
                0x01 => return, // Esc – exit to menu
                _ => {}
            }
        }
    }
}
