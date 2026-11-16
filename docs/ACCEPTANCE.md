# Acceptance Checklist

- [ ] **Boot ISO**: `rustos.iso` boots in VirtualBox without errors.
- [ ] **GRUB menu** appears and loads the kernel.
- [ ] **Desktop menu** loads after boot and displays all applications.
- [ ] **Shell** starts correctly and displays the prompt.
- [ ] **Shell commands**: `cat`, `bc`, `cut`, `head`, `tail`, `wc`, `date`, `clear`, `help`, `run`, `exit` all work as documented.
- [ ] **Games**: `snake`, `snakes_and_ladders`, `car_dodge` launch, run, and return to the menu.
- [ ] **Deadlock Lab** runs all six patterns, displays chart, evaluates hypotheses, and writes `deadlock_results.txt`.
- [ ] **Result file** `deadlock_results.txt` exists in the in‑memory FS and can be displayed with `cat`.
- [ ] **Boot‑test script** `scripts/boot_test.ps1` runs autonomously and reports success.
- [ ] **Documentation**: All required docs (`README.md`, `ARCHITECTURE.md`, `COMMANDS.md`, `GAMES.md`, `DEADLOCK_LAB.md`, `RUN_VIRTUALBOX.md`, `ACCEPTANCE.md`, `REPORT_NOTES.md`) are present and complete.
- [ ] **Screenshots** folder contains images of GRUB, desktop, each game, deadlock lab UI, and successful boot‑test output.
- [ ] **Source archive** `source.zip` contains the full repository.
- [ ] **Deliverables package** includes `rustos.iso`, `source.zip`, `screenshots/`, all docs, and `results/deadlock_results.txt`.

---
*Mark each item as checked when verified.*