# Terminal Snake (Rust)

A terminal-based Snake-inspired game written in Rust using the GermTerm rendering engine.(not an actual snake a red dot which is the main character why>??? im not smart)

## Features

* Smooth terminal rendering
* Collectible apples
* Particle explosion effects when apples are collected
* Keyboard controls
* FPS counter
* Colorful terminal graphics

## Controls

| Key | Action     |
| --- | ---------- |
| W   | Move Up    |
| A   | Move Left  |
| S   | Move Down  |
| D   | Move Right |
| Q   | Quit Game  |

## Installation

### Option 1: Run the Prebuilt Executable

1. Download the project.
2. Extract the ZIP file.
3. Open the extracted folder.
4. Navigate to the executable.
5. Run `terminal-snake-rs.exe`.
note:if it says terminal size too small what u can do is open cmd in fullscreen or maximised and then navigate to the root dir of the project and run
`terminal-snake-rs.exe`


### Option 2: Build from Source

#### Requirements

* Rust
* Cargo

#### Build

```bash
cargo build --release
```

#### Run

```bash
cargo run --release
```

## Gameplay

Move around the terminal and collect all apples to win the game. Each collected apple triggers a particle explosion effect. Clear the board to achieve victory.

## Screenshots
![Gameplay Screenshot](gallery/Screenshot%202026-06-19%20203244.png)
