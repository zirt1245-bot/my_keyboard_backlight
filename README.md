# Kbd-Backlight Control (Rust version)

Just a small personal utility to mess around with my keyboard backlight. It's written in Rust and talks directly to the `tuxedo_keyboard` driver.

I built this because I wanted a quicker way to change colors and brightness than editing sysfs files manually or using heavy apps.

## Features

* **Color by name:** Just type `red`, `green`, `blue`, `golden`, etc..
* **RGB support:** Set exact values if you're feeling picky.
* **Smart Brightness:** Supports percentages (50%), relative changes (+10, -20), or just "max" it out.
* **Daemon Mode:** Background effects are now spawned as separate processes so they don't block your terminal.
* **Rainbow Mode:** A smooth HSV loop for the "gamer" aesthetic.
* **Random Colors:** Automatically cycle through random colors at a set interval.

## Prerequisites

This tool is specifically designed for laptops that use the **tuxedo-keyboard** driver (Clevo, TUXEDO, etc.). 
You need to have the driver installed and loaded so that this path exists:
`/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/multi_intensity`.

## How to use

Since it's built with `clap`, you can always run `--help` to see the options. Here are some examples:

### Change Color
```bash
# By name
cargo run -- --color orange

# By RGB values
cargo run -- --rgb 255 0 125
```

### Adjust Brightness
The brightness logic keeps the color ratio while changing the intensity.
```bash
# Set to 50%
cargo run -- --brightness 50%

# Make it a bit brighter
cargo run -- --brightness +20

# Maximize current color intensity
cargo run -- --brightness max
```

### Background Effects (Daemon Mode)
The program automatically stops any previously running effect before starting a new one.
```bash
# Start the rainbow loop in the background
cargo run -- --rainbow

# Cycle random colors every 5 seconds
cargo run -- --random 5

# Stop all background effects
cargo run -- --stop

# Turn backlight off/on
cargo run -- --off
cargo run -- --on
```

## Build
```bash
cargo build --release
```
The binary will be in `target/release/`.

## Notes
* **Permissions:** You likely need `sudo` or a udev rule to write to the `/sys` LED file.
* **PID Management:** The tool uses `/tmp/backlight_background.pid` to track and manage background processes.

*Have fun with the glow!* 🌈
