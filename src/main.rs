mod brightness;
mod name_color;
mod rainbow;
mod random_colors;
mod utils;

use clap::Parser;
use std::env;
use std::fs;
use std::process::Command;
// команды терминала
use brightness::{apply_brightness, parse_brightness, read_color};
use name_color::get_color_by_name;
use rainbow::hsv;
use random_colors::rand_color;
use utils::{off_on, write_color};

static LED: &str = "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/multi_intensity";
static OFF_ON: &str = "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/brightness";
static PID_FILE: &str = "/tmp/backlight_background.pid";

#[derive(Parser, Debug)]
#[command(
    name = "Backlight",
    version,
    about = "Changes the color of the keyboard"
)]
struct Arge {
    /// Your RGB: --rgb 0 0 0
    #[arg(long, num_args = 3, value_names = ["RED", "GREEN", "BLUE"])]
    rgb: Option<Vec<u8>>,

    /// Off backlight
    #[arg(long)]
    off: bool,

    /// On backlight
    #[arg(long)]
    on: bool,

    /// Your color: red, green, blue, etc
    #[arg(short, long)]
    color: Option<String>,

    /// Brightness: 50%, +10, =-20, or 255
    #[arg(short, long)]
    brightness: Option<String>,

    /// Rainbow
    #[arg(short, long)]
    rainbow: bool,

    /// Random colors: --random 10 (s)
    #[arg(long)]
    random: Option<u8>,

    /// Stop background effects
    #[arg(long)]
    stop: bool,

    #[arg(long, hide = true)]
    rainbow_daemon: bool,

    #[arg(long, hide = true)]
    random_daemon: bool,
}

fn main() {
    if !std::path::Path::new(LED).exists() {
        eprintln!("Error: Backlight control file not found. It appears the driver is not loaded");
        return;
    }

    let args = Arge::parse();

    if args.rainbow_daemon {
        hsv(LED);
        return;
    }

    if args.random_daemon {
        if let Some(s) = args.random {
            rand_color(LED, s);
        }
        return;
    }

    if args.off {
        off_on(OFF_ON, 0);
    } else if args.on {
        off_on(OFF_ON, 255);
    }

    let mut current_color = if let Some(rgb) = args.rgb {
        (rgb[0], rgb[1], rgb[2])
    } else if let Some(color_name) = args.color {
        get_color_by_name(&color_name)
    } else {
        read_color(LED)
    };

    if let Some(b_str) = args.brightness {
        match parse_brightness(&b_str) {
            Some(cmd) => {
                let (r, g, b) = current_color;
                current_color = apply_brightness(r, g, b, &cmd);
            }
            None => eprintln!("Invalid brightness format"),
        }
    }

    if !args.rainbow && args.random.is_none() {
        let (r, g, b) = current_color;
        write_color(LED, r, g, b);
    }

    if args.rainbow {
        stop_daemon();

        let exe = match env::current_exe() {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("Failed to get path to executable file: {}", e);
                return;
            }
        };

        let child = match Command::new(exe).arg("--rainbow-daemon").spawn() {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("Failed to start background process: {}", e);
                return;
            }
        };

        let _ = fs::write(PID_FILE, child.id().to_string());
        println!("Rainbow process started (PID: {})", child.id());
        return;
    }

    if let Some(mut s) = args.random {
        if s < 1 {
            s = 1;
        }

        stop_daemon();

        let exe = match env::current_exe() {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("Failed to get path to executable file: {}", e);
                return;
            }
        };

        let child = match Command::new(exe)
            .arg("--random-daemon")
            .arg("--random")
            .arg(s.to_string())
            .spawn()
        {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("Failed to start background process: {}", e);
                return;
            }
        };

        let _ = fs::write(PID_FILE, child.id().to_string());
        println!("Random colors process started (PID: {})", child.id());
        return;
    }

    if args.stop {
        stop_daemon();
        println!("Background process stopped");
    }
}

fn stop_daemon() {
    if let Ok(pid_str) = fs::read_to_string(PID_FILE) {
        if let Ok(pid) = pid_str.trim().parse::<i32>() {
            // убиваем процесс через системный kill
            let _ = Command::new("kill").arg("-9").arg(pid.to_string()).output();
        }
        let _ = fs::remove_file(PID_FILE);
    }
}
