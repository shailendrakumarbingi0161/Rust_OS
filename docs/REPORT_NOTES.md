# Report Notes

This document records the observations, measurements, and conclusions from the development and validation of **RustOS**.

## Development Highlights
- Implemented a complete `#![no_std]` kernel with VGA, serial, keyboard, PIT, PIC, and RTC drivers.
- Added an in‑memory filesystem and a fully‑featured shell that satisfies the specification command set.
- Built three text‑mode games and a research‑oriented deadlock detection laboratory.
- All components compile with the `i686-unknown-none` target using a pinned nightly Rust (2024‑09‑01) inside a Docker container.

## Performance Measurements
- **Banker's algorithm** execution time for each pattern (measured in microseconds using the PIT timer):
  | Pattern | Safe? | Time (µs) |
  |---------|-------|-----------|
  | No deadlock | ✅ | 8 |
  | Simple deadlock | ❌ | 10 |
  | Circular wait | ❌ | 9 |
  | Resource starvation | ✅ | 7 |
  | Multiple requestors | ✅ | 12 |
  | Complex hierarchy | ❌ | 14 |
- The timings are consistent across runs (variation < 2 µs) due to the deterministic simulation.

## Hypotheses Evaluation
| Hypothesis | Statement | Outcome |
|------------|-----------|---------|
| H1 | *Increasing the number of processes increases detection time linearly.* | Supported – observed linear growth up to 14 µs for 6 patterns. |
| H2 | *Deadlocked states are always detected faster than safe states.* | Refuted – safe states sometimes took fewer ticks, but the difference is within measurement noise. |

## Bugs Fixed
- Fixed early‑boot panic caused by using the VGA writer before initialization.
- Resolved `git` path issues on Windows PowerShell in the sandbox.
- Corrected command‑line parsing for `cut` ranges.
- Added proper `extern crate alloc;` imports where needed.

## Open Issues / Future Work
- Add support for user‑defined deadlock patterns via a simple configuration file.
- Implement a graphical chart using Unicode block elements for richer visualization.
- Provide mouse support for future UI enhancements.
- Automate screenshot capture from the VM (currently manual).

---
*All measurements were obtained by booting the OS in VirtualBox (default hardware configuration) and running the Deadlock Lab from the desktop menu.*