# RustOS

**RustOS** is a 32‑bit x86 educational operating system written in pure Rust (`#![no_std]`). It boots via GRUB, runs inside Oracle VirtualBox, and provides:

- A VGA text‑mode desktop with a menu.
- A fully‑featured shell implementing the command set from the specification (cat, bc, cut, head, tail, wc, date, clear, help, run, exit).
- Three interactive games: **Snake**, **Snakes & Ladders**, and **Car Dodge**.
- The **Deadlock Detection Lab**, a research feature that demonstrates the Banker's algorithm, runs experiments, and writes results to `deadlock_results.txt`.
- An in‑memory filesystem and basic unit tests that run on the host.

The project includes build scripts (`scripts/build.ps1`, `scripts/build.sh`) that use Docker to compile the kernel and create a bootable ISO, and a boot‑test script (`scripts/boot_test.ps1`) for CI.

## Quick Start
```powershell
# Build the ISO
.\scripts\build.ps1

# Run the VM (headless)
VBoxManage startvm RustOS --type headless
```
See [RUN_VIRTUALBOX.md](docs/RUN_VIRTUALBOX.md) for full instructions.

---
*All documentation, source code, and deliverables are packaged in the `deliverables/` directory as specified in the acceptance checklist.*