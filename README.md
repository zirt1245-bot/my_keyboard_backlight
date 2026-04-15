# Kbd-Backlight Control (Rust version)

Just a small personal utility to mess around with my keyboard backlight. It's written in Rust and talks directly to the `tuxedo_keyboard` driver.

I built this because I wanted a quicker way to change colors and brightness than editing sysfs files manually or using heavy apps.

## Features

* **Color by name:** Just type `red`, `green`, `blue`, `golden`, etc..
* **RGB support:** Set exact values if you're feeling picky.
* **Smart Brightness:** Supports percentages (50%), relative changes (+10, -20), or just "max" it out.
* **Rainbow Mode:** A simple HSV loop for when I want the "gamer" aesthetic.

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
The brightness logic is pretty cool: it keeps the color ratio while changing the intensity.
```bash
# Set to 50%
cargo run -- --brightness 50%

# Make it a bit brighter
cargo run -- --brightness +20

# Absolute value (0-255)
cargo run -- --brightness 150

# Maximize current color intensity
cargo run -- --brightness max
```

### Special Modes
```bash
# Start the rainbow loop
cargo run -- --rainbow

# Turn it off
cargo run -- --off
```

## Build
Standard Rust stuff:
```bash
cargo build --release
```
The binary will be in `target/release/`. You might want to move it to your `/usr/local/bin/` or set up an alias.

## Notes
* **Permissions:** You probably need `sudo` or a udev rule to write to the `/sys` LED file.
* If the driver isn't found, the app will let you know instead of crashing.

### Current Limitations (The "Cons")

Since I wrote this mostly for myself, there are a few things you should know:

* **Root Permissions:** Writing to `/sys/devices/...` usually requires `root` privileges. You'll likely need to run the tool with `sudo` every time or set up a specific udev rule.
* **Hardcoded Path:** The path to the LED control file is currently hardcoded for the Tuxedo driver. If your system mounting point is different, it simply won't work without a code change.
* **Blocking Rainbow Mode:** The rainbow effect is a simple infinite loop. If you start it, that terminal tab is effectively "taken" until you `Ctrl+C` to stop it.
* **Limited Color Palette:** The "color by name" feature only supports a specific set of predefined colors. If you want something exotic like "Peach" or "Lavender," you'll have to use the `--rgb` flag.
* **Basic Error Handling:** The code uses simple `unwrap` or basic `eprintln` calls. It’s not built to be bulletproof; it’s built to work on my machine.
* **No Daemon Mo
* de:** There is no background service. If you want a specific brightness or color to persist after a reboot, you'll have to run the command again or add it to your startup scripts.

*Have fun with the glow!* 🌈
