# Book (Rust console particle demo)

A small Rust console application that draws a bordered playfield and simulates a particle you can accelerate with the arrow keys. Built with `crossterm` for terminal input/output.

## Features
- Draws a border around the terminal area
- Simulates a single particle with subpixel-ish movement
- Keyboard controls with key press/hold handling

## Controls
- Arrow Up/Down/Left/Right: Apply acceleration to the particle
- q: Quit the application

Note: The app enables terminal raw mode while running and disables it on exit.

## Requirements
- Rust (stable) and Cargo installed: https://www.rust-lang.org/tools/install

## Build and Run
```bash
# In the project root
cargo run
```

If your terminal supports it, you can modify the code to detect dynamic terminal size. Currently the demo uses a fixed size of 45x15 (see `main.rs`).

## Project Structure
- src/main.rs: Program entry point and input loop
- src/console.rs: Console drawing utilities
- src/particle.rs: Particle data and update logic
- src/border.rs: Border drawing helpers
- src/spatial.rs: Coordinate utilities and constants

## Notes
- JetBrains IDE metadata is ignored via `.gitignore` (`.idea/`).
- Build artifacts (`target/`) and common local editor files are also ignored.
