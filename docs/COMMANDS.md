# Command Reference

This document describes the shell commands implemented in **RustOS** (Section 6 of the specification). All commands run inside the OS and operate on the in‑memory filesystem.

| Command | Synopsis | Description |
|---------|----------|-------------|
| `cat &lt;file&gt;` | `cat README.md` | Prints the contents of a file to the screen. Errors are shown in **red** if the file does not exist.
| `bc` | `bc` (interactive) | Simple integer calculator. Supports `+ - * / % ^` and parentheses. Enter `quit` to return to the shell.
| `cut -c &lt;range&gt; &lt;file&gt;` | `cut -c 1-5 file.txt` | Prints the selected character columns from each line. Range can be a single number (`5`), a start‑end (`2-8`), or open‑ended (`3-`).
| `head -n &lt;count&gt; &lt;file&gt;` | `head -n 10 log.txt` | Shows the first *count* lines of the file. Default count is 10.
| `tail -n &lt;count&gt; &lt;file&gt;` | `tail -n 5 notes.txt` | Shows the last *count* lines of the file. Default count is 10.
| `wc &lt;file&gt;` | `wc data.txt` | Prints three numbers: **lines words bytes**.
| `date` | `date` | Displays the current date and time obtained from the RTC driver.
| `clear` | `clear` | Clears the screen (fills the VGA buffer with blanks).
| `help` | `help` | Shows a list of all available commands with a short description.
| `run &lt;app&gt;` | `run snake` | Directly launches an application (same as selecting it from the desktop menu). Valid apps: `shell`, `snake`, `snakes_and_ladders`, `car_dodge`, `deadlock_lab`.
| `exit` | `exit` | Exits the current application and returns to the desktop menu (or exits the shell entirely).

## Error Handling
- Errors are printed **red** (`attribute 0x04`).
- Unrecognized commands produce `"unknown command: <cmd>"`.
- Invalid arguments (e.g., missing file) display a helpful usage line.

## History
The shell maintains a **16‑entry** command history navigable with the **Up/Down** arrow keys.

---
*Future extensions may add more utilities; all new commands must follow the same error‑display conventions.*