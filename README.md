# ShellShock Live Trainer
A simple (non intrusive) trainer for [ShellShock Live](http://www.shellshocklive.com)

**The project is only for exercise purposes and should not be used for cheating.**

My main interest is not the game itself. The goal of this project was to improve my knowledge of the Rust programming language and the Windows-API.

# Usage (Windows only)

1. Execute the trainer (installation see below).
2. Start "Shellshock Live" (the trainer automatically detects a running instance of "Shellshock Live").
3. There are four hardcoded keys
    * Key 1 (save current mouse position as position 1)
    * Key 2 (save current mouse position as position 2)
    * Key 3 (calculate different angle/speed combinations to hit the target (position 2))
    * Key 4 (clear positions)
    * Key 5 (switch calculation mode)

Example:
1. Move the mouse over your tank and press '1'.
2. Move the mouse over the enemy tank and press '2'.
3. Press '3'.

# Installation (Windows only)

## Install the [Rust](https://www.rust-lang.org) compiler toolchain

- [32-Bit](https://static.rust-lang.org/dist/rust-1.12.1-i686-pc-windows-gnu.msi) or
- [64-Bit](https://static.rust-lang.org/dist/rust-1.12.1-x86_64-pc-windows-gnu.msi)

## Download sources

Clone this repository with [Git for Windows](https://git-scm.com)
```
git clone [REPO]
```
    
Alternatively download this repository
```
Button "Clone or download" -> "Download ZIP"
```

## Build
1. Open a command line window and change to the directory (cloned or downloaded).
2. Build (output folder "target\release\shellshock-trainer.exe")
```
cargo build --release
```

3. Run
```
cargo run --release
```

or

```
cd target\release
shellshock-trainer.exe
```

# License
MIT