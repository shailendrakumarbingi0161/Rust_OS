# Deadlock Detection Lab Documentation

The **Deadlock Lab** (`deadlock_lab.rs`) provides a research‑focused UI that demonstrates resource‑allocation deadlock detection using the classic Banker's algorithm.

## Features
- **Six predefined resource‑allocation patterns** (no deadlock, simple deadlock, circular wait, etc.).
- **Simulation UI**: Users can step through each pattern, view the allocation matrix, and watch the algorithm decide whether the system is safe.
- **Experiment Mode**: Runs all patterns sequentially, measures the execution time for each using the PIT tick counter, and records the timings.
- **Chart View**: A simple ASCII bar chart visualises the measured times.
- **Hypotheses Evaluation**: Two hypotheses are presented and automatically marked as *supported* or *refuted* based on the observed data.
- **Result Export**: After experiments, the lab writes a text file `deadlock_results.txt` into the in‑memory filesystem. The file contains:
  - Pattern name
  - Safe/Unsafe result
  - Execution time (µs)
  - Hypothesis outcome

## How to Run
From the desktop menu select **Deadlock Lab** or type `run deadlock_lab` in the shell.

## Implementation Details
- The **Banker's algorithm** is implemented in a pure‑Rust function `detect_deadlock(resources: &[[u32; N]; M], available: &[u32; N]) -> bool`.
- Timing uses the **PIT** counter: `pit::ticks()` gives the number of ticks since boot; we convert ticks to microseconds based on the 100 Hz frequency.
- UI rendering re‑uses the VGA text‑mode helper functions to draw tables and charts.
- Results are stored in a static `ResultLog` struct; writing to the filesystem is done via `fs::write_file("deadlock_results.txt", &log)`.

---
*Future work*: Extend the lab with user‑defined patterns, larger resource counts, and a graphical chart using Unicode block characters.*