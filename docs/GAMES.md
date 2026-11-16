# Game Reference

RustOS includes three interactive text‑mode games. All games run in the same VGA text‑mode buffer and use the same keyboard driver as the shell.

## Snake (`snake`)
- **Controls**: Arrow keys – move the snake head up/down/left/right.
- **Goal**: Eat the randomly‑generated food (`*`) to grow. Colliding with the walls or the snake body ends the game.
- **Scoring**: Each food eaten increments the score displayed at the top‑right corner.
- **Exit**: Press `Esc` to quit and return to the desktop menu.

## Snakes & Ladders (`snakes_and_ladders`)
- **Controls**: `Enter` to roll the die, arrow keys to move the cursor on the board.
- **Board**: 10×10 grid numbered 1‑100. Ladders move you up; snakes move you down.
- **Goal**: Reach square 100 exactly.
- **Exit**: Press `Esc` at any time to return to the menu.

## Car Dodge (`car_dodge`)
- **Controls**: `A` to move left, `D` to move right, `Space` to pause.
- **Gameplay**: Cars appear at the top of a three‑lane road and move downwards. Avoid collisions.
- **Scoring**: Survive as long as possible; the current score is shown in the status bar.
- **High‑Score**: The highest score achieved in the session is stored in a static `AtomicUsize` and displayed after each run.
- **Exit**: Press `Esc` to quit and go back to the menu.

All games share the same `run_game` helper that:
1. Saves the current VGA buffer.
2. Clears the screen.
3. Executes the game loop until the user quits.
4. Restores the saved buffer before returning to the menu.

---
*Screenshots of each game will be added to `docs/screenshots/` once the final ISO is built.*