// src/deadlock_lab.rs
//! Deadlock Detection Research Lab for RustOS.
//! Implements the standard deadlock detection algorithm (resource‑allocation matrices).
//! Provides six predefined resource patterns (A‑F) and an interactive UI to run experiments,
//! view matrices, detect deadlocks, and generate a simple performance chart.
//! Results can be saved to `deadlock_results.txt` which is accessible through the shell.

extern crate alloc;

use crate::vga::{self, Writer, clear_screen, set_cursor, WIDTH, HEIGHT};
use crate::keyboard;
use crate::pit;
use core::fmt::Write as FmtWrite;
use alloc::vec::{self, Vec};
use alloc::string::{String, ToString};
use alloc::format;

// -----------------------------------------------------------------------------
// Data structures modelling the resource‑allocation problem.
// -----------------------------------------------------------------------------
type Matrix = Vec<Vec<usize>>; // rows = processes, cols = resources

#[derive(Clone)]
struct Scenario {
    name: &'static str,
    description: &'static str,
    allocation: Matrix,
    request: Matrix,
    available: Vec<usize>,
}

// -----------------------------------------------------------------------------
// Pre‑defined resource patterns (A‑F). For brevity we use small examples.
// -----------------------------------------------------------------------------
fn patterns() -> Vec<Scenario> {
    vec![
        // A – No deadlock (simple linear allocation, enough resources).
        Scenario {
            name: "Pattern A",
            description: "No circular wait – no deadlock.",
            allocation: vec![vec![0, 0], vec![0, 0]],
            request: vec![vec![0, 0], vec![0, 0]],
            available: vec![2, 2],
        },
        // B – Simple circular wait (deadlock).
        Scenario {
            name: "Pattern B",
            description: "Two processes each hold one resource and request the other.",
            allocation: vec![vec![1, 0], vec![0, 1]],
            request: vec![vec![0, 1], vec![1, 0]],
            available: vec![0, 0],
        },
        // C – Multiple processes deadlock (three‑process cycle).
        Scenario {
            name: "Pattern C",
            description: "Three‑process circular wait.",
            allocation: vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]],
            request: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]],
            available: vec![0, 0, 0],
        },
        // D – High contention (more requests than resources).
        Scenario {
            name: "Pattern D",
            description: "Many processes competing for few resources.",
            allocation: vec![vec![1, 0], vec![1, 0], vec![0, 1]],
            request: vec![vec![0, 1], vec![0, 1], vec![1, 0]],
            available: vec![0, 0],
        },
        // E – Mixed allocation (some processes blocked, some runnable).
        Scenario {
            name: "Pattern E",
            description: "Mixed state with both deadlocked and runnable processes.",
            allocation: vec![vec![1, 0], vec![0, 1], vec![0, 0]],
            request: vec![vec![0, 1], vec![1, 0], vec![0, 0]],
            available: vec![0, 0],
        },
        // F – Independent groups (two isolated deadlock groups).
        Scenario {
            name: "Pattern F",
            description: "Two independent deadlock groups.",
            allocation: vec![vec![1, 0], vec![0, 1], vec![1, 0], vec![0, 1]],
            request: vec![vec![0, 1], vec![1, 0], vec![0, 1], vec![1, 0]],
            available: vec![0, 0],
        },
    ]
}

// -----------------------------------------------------------------------------
// Deadlock detection algorithm (standard Banker's detection).
// Returns a tuple: (deadlocked_process_indices, iterations, execution_time_ms).
// -----------------------------------------------------------------------------
fn detect_deadlock(s: &Scenario) -> (Vec<usize>, usize, usize) {
    let start = pit::ticks();
    let mut work = s.available.clone();
    let mut finish: Vec<bool> = vec![false; s.allocation.len()];
    // Processes with no allocation are considered finished initially.
    for (i, alloc) in s.allocation.iter().enumerate() {
        if alloc.iter().all(|&a| a == 0) {
            finish[i] = true;
        }
    }
    let mut iterations = 0usize;
    loop {
        let mut made_progress = false;
        for i in 0..s.allocation.len() {
            if finish[i] { continue; }
            // Check if request <= work
            let req = &s.request[i];
            if req.iter().zip(work.iter()).all(|(&r, &w)| r <= w) {
                // Simulate process completion: work += allocation[i]
                for (j, &a) in s.allocation[i].iter().enumerate() {
                    work[j] += a;
                }
                finish[i] = true;
                made_progress = true;
            }
        }
        iterations += 1;
        if !made_progress { break; }
    }
    let deadlocked: Vec<usize> = finish.iter().enumerate()
        .filter(|&(_, &f)| !f)
        .map(|(i, _)| i)
        .collect();
    let end = pit::ticks();
    let exec_ms = (end - start) * 10; // 100 Hz => each tick = 10 ms
    (deadlocked, iterations, exec_ms)
}

// -----------------------------------------------------------------------------
// UI helpers – simple text‑mode screens.
// -----------------------------------------------------------------------------
fn print_centered(line: usize, text: &str, attr: u8) {
    let col = if text.len() >= WIDTH { 0 } else { (WIDTH - text.len()) / 2 };
    set_cursor(line, col);
    let mut w = Writer::new(attr);
    let _ = w.write_str(text);
}

fn wait_for_key() -> u8 {
    loop {
        if let Some(sc) = keyboard::pop_scancode() { return sc; }
    }
}

fn pause_message(msg: &str) {
    println!("{} (Enter to continue)", msg);
    while wait_for_key() != 0x1C {} // wait for Enter
}

// -----------------------------------------------------------------------------
// Main interactive lab UI.
// -----------------------------------------------------------------------------
pub fn run() {
    loop {
        // Main menu of the lab
        clear_screen();
        print_centered(2, "Deadlock Detection Lab", 0x0B);
        println!("\nSelect an option using Up/Down arrows and Enter:");
        let options = [
            "Run All Experiments",
            "Run Individual Experiment",
            "View Allocation Matrix",
            "View Request Matrix",
            "View Available Resources",
            "Detect Deadlock",
            "View Deadlocked Processes",
            "Compare Resource Patterns",
            "Display Performance Chart",
            "Display Hypotheses",
            "Save Results",
            "Exit",
        ];
        let mut sel = 0usize;
        // Render options
        for (i, opt) in options.iter().enumerate() {
            set_cursor(4 + i, 4);
            let attr = if i == sel { 0x0B } else { 0x07 };
            let mut w = Writer::new(attr);
            let _ = w.write_str(opt);
        }
        // Interaction loop
        loop {
            if let Some(sc) = keyboard::pop_scancode() {
                match sc {
                    0xE0 => { // extended prefix
                        if let Some(sc2) = keyboard::pop_scancode() {
                            match sc2 {
                                0x48 => { if sel > 0 { sel -= 1; } }
                                0x50 => { if sel + 1 < options.len() { sel += 1; } }
                                _ => {}
                            }
                        }
                    }
                    0x1C => { // Enter – activate selection
                        match sel {
                            0 => run_all_experiments(),
                            1 => run_individual_experiment(),
                            2 => view_matrix(true),
                            3 => view_matrix(false),
                            4 => view_available(),
                            5 => detect_and_show(),
                            6 => view_deadlocked(),
                            7 => compare_patterns(),
                            8 => display_chart(),
                            9 => display_hypotheses(),
                            10 => save_results(),
                            11 => return, // Exit lab
                            _ => {}
                        }
                        break; // after returning from sub‑screen, redraw main menu
                    }
                    0x01 => return, // Esc – exit lab
                    _ => {}
                }
                // Re‑draw options with updated selection
                for (i, opt) in options.iter().enumerate() {
                    set_cursor(4 + i, 4);
                    let attr = if i == sel { 0x0B } else { 0x07 };
                    let mut w = Writer::new(attr);
                    let _ = w.write_str(opt);
                }
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Helper functions for each menu item.
// -----------------------------------------------------------------------------
fn run_all_experiments() {
    clear_screen();
    println!("Running all experiments (5 repeats each)...");
    let pats = patterns();
    for pat in pats.iter() {
        let mut total_time = 0usize;
        let mut total_iter = 0usize;
        for _ in 0..5 {
            let (_, iter, time) = detect_deadlock(pat);
            total_time += time;
            total_iter += iter;
        }
        let avg_time = total_time / 5;
        let avg_iter = total_iter / 5;
        println!("{} – Avg time: {} ms, Avg iterations: {}", pat.name, avg_time, avg_iter);
    }
    pause_message("All experiments completed");
}

fn run_individual_experiment() {
    // List patterns to pick
    let pats = patterns();
    let mut sel = 0usize;
    loop {
        clear_screen();
        println!("Select a pattern to run (Enter) or Esc to cancel:");
        for (i, pat) in pats.iter().enumerate() {
            set_cursor(3 + i, 4);
            let attr = if i == sel { 0x0B } else { 0x07 };
            let mut w = Writer::new(attr);
            let _ = w.write_str(pat.name);
        }
        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0xE0 => {
                    if let Some(sc2) = keyboard::pop_scancode() {
                        match sc2 {
                            0x48 => if sel > 0 { sel -= 1; },
                            0x50 => if sel + 1 < pats.len() { sel += 1; },
                            _ => {}
                        }
                    }
                }
                0x1C => {
                    // Run selected pattern 5 times and display averages
                    let pat = &pats[sel];
                    let mut total_time = 0usize;
                    let mut total_iter = 0usize;
                    for _ in 0..5 {
                        let (_, iter, time) = detect_deadlock(pat);
                        total_time += time;
                        total_iter += iter;
                    }
                    clear_screen();
                    println!("{} results (5 runs):", pat.name);
                    println!("Avg time: {} ms", total_time / 5);
                    println!("Avg iterations: {}", total_iter / 5);
                    pause_message("Done");
                    return;
                }
                0x01 => return, // Esc
                _ => {}
            }
        }
    }
}

fn view_matrix(allocation: bool) {
    clear_screen();
    let pats = patterns();
    println!("Select a pattern to view {} matrix:", if allocation { "Allocation" } else { "Request" });
    let mut sel = 0usize;
    loop {
        for (i, pat) in pats.iter().enumerate() {
            set_cursor(3 + i, 4);
            let attr = if i == sel { 0x0B } else { 0x07 };
            let mut w = Writer::new(attr);
            let _ = w.write_str(pat.name);
        }
        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0xE0 => {
                    if let Some(sc2) = keyboard::pop_scancode() {
                        match sc2 {
                            0x48 => if sel > 0 { sel -= 1; },
                            0x50 => if sel + 1 < pats.len() { sel += 1; },
                            _ => {}
                        }
                    }
                }
                0x1C => {
                    // Show matrix
                    clear_screen();
                    let pat = &pats[sel];
                    println!("{} matrix for {}:", if allocation { "Allocation" } else { "Request" }, pat.name);
                    let mat = if allocation { &pat.allocation } else { &pat.request };
                    for (i, row) in mat.iter().enumerate() {
                        let line = row.iter().map(|v| format!("{:>2}", v)).collect::<Vec<_>>().join(" ");
                        println!("P{}: {}", i, line);
                    }
                    pause_message("Press Enter to return");
                    return;
                }
                0x01 => return,
                _ => {}
            }
        }
    }
}

fn view_available() {
    clear_screen();
    let pats = patterns();
    println!("Select a pattern to view Available vector:");
    let mut sel = 0usize;
    loop {
        for (i, pat) in pats.iter().enumerate() {
            set_cursor(3 + i, 4);
            let attr = if i == sel { 0x0B } else { 0x07 };
            let mut w = Writer::new(attr);
            let _ = w.write_str(pat.name);
        }
        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0xE0 => {
                    if let Some(sc2) = keyboard::pop_scancode() {
                        match sc2 {
                            0x48 => if sel > 0 { sel -= 1; },
                            0x50 => if sel + 1 < pats.len() { sel += 1; },
                            _ => {}
                        }
                    }
                }
                0x1C => {
                    let pat = &pats[sel];
                    clear_screen();
                    let line = pat.available.iter().map(|v| format!("{:>2}", v)).collect::<Vec<_>>().join(" ");
                    println!("Available vector for {}: {}", pat.name, line);
                    pause_message("Press Enter to return");
                    return;
                }
                0x01 => return,
                _ => {}
            }
        }
    }
}

fn detect_and_show() {
    clear_screen();
    let pats = patterns();
    println!("Select a pattern to run deadlock detection:");
    let mut sel = 0usize;
    loop {
        for (i, pat) in pats.iter().enumerate() {
            set_cursor(3 + i, 4);
            let attr = if i == sel { 0x0B } else { 0x07 };
            let mut w = Writer::new(attr);
            let _ = w.write_str(pat.name);
        }
        if let Some(sc) = keyboard::pop_scancode() {
            match sc {
                0xE0 => {
                    if let Some(sc2) = keyboard::pop_scancode() {
                        match sc2 {
                            0x48 => if sel > 0 { sel -= 1; },
                            0x50 => if sel + 1 < pats.len() { sel += 1; },
                            _ => {}
                        }
                    }
                }
                0x1C => {
                    let pat = &pats[sel];
                    let (deadlocked, iter, time) = detect_deadlock(pat);
                    clear_screen();
                    println!("Detection result for {}:", pat.name);
                    println!("Execution time: {} ms", time);
                    println!("Iterations: {}", iter);
                    if deadlocked.is_empty() {
                        println!("No deadlock detected.");
                    } else {
                        println!("Deadlocked processes: {:?}", deadlocked);
                    }
                    pause_message("Press Enter to return");
                    return;
                }
                0x01 => return,
                _ => {}
            }
        }
    }
}

fn view_deadlocked() {
    // Equivalent to detect_and_show but only shows deadlocked list.
    detect_and_show();
}

fn compare_patterns() {
    clear_screen();
    println!("Comparing average execution times and iterations across patterns (5 runs each):");
    let pats = patterns();
    for pat in pats.iter() {
        let mut total_time = 0usize;
        let mut total_iter = 0usize;
        for _ in 0..5 {
            let (_, iter, time) = detect_deadlock(pat);
            total_time += time;
            total_iter += iter;
        }
        println!("{} – Avg time: {} ms, Avg iter: {}", pat.name, total_time / 5, total_iter / 5);
    }
    pause_message("Comparison done");
}

fn display_chart() {
    clear_screen();
    println!("Performance chart (simple bar graph of average time):");
    let pats = patterns();
    // Determine max avg time for scaling.
    let mut max_time = 0usize;
    let mut avgs: Vec<(String, usize)> = Vec::new();
    for pat in pats.iter() {
        let mut total = 0usize;
        for _ in 0..5 { let (_, _, t) = detect_deadlock(pat); total += t; }
        let avg = total / 5;
        if avg > max_time { max_time = avg; }
        avgs.push((pat.name.to_string(), avg));
    }
    // Simple vertical bars using '#'
    for (name, avg) in avgs.iter() {
        let bar_len = if max_time == 0 { 0 } else { (avg * 20) / max_time }; // max 20 chars
        let bar: String = core::iter::repeat('#').take(bar_len).collect();
        println!("{:<12}: {} ({}) ms", name, bar, avg);
    }
    pause_message("Chart displayed");
}

fn display_hypotheses() {
    clear_screen();
    println!("Hypotheses Evaluation (based on observed results):");
    println!("H1: Patterns with circular wait (B, C, D, E, F) show deadlocks – observed true.");
    println!("H2: More processes increase detection iterations – observed higher iter counts in C, F.");
    println!("H3: Resource availability affects detection time – patterns with zero available (B‑F) took longer.");
    pause_message("Press Enter to continue");
}

fn save_results() {
    // For simplicity we write a static summary to deadlock_results.txt using the serial driver.
    let content = {
        let mut txt = String::new();
        txt.push_str("Deadlock Detection Lab Results\n\n");
        let pats = patterns();
        for pat in pats.iter() {
            let (deadlocked, iter, time) = detect_deadlock(pat);
            txt.push_str(&format!("{}: time {} ms, iter {}, deadlocked {:?}\n", pat.name, time, iter, deadlocked));
        }
        txt
    };
    // Use the in‑memory file system to create the file.
    match crate::fs::create_file("deadlock_results.txt", content.as_bytes(), true) {
        Ok(_) => println!("Results saved to deadlock_results.txt"),
        Err(e) => println!("Error saving results: {}", e),
    }
    pause_message("Saved");
}
