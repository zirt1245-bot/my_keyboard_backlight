mod brightness;

use std::fs;
// работа с файловой системой
//use std::process::Command;
// команды для терминала
//use std::thread;
//use std::time::{Duration, Instant, SystemTime};
// работа со временем
use clap::Parser;
// команды терминала
use brightness::{apply_brightness, parse_brightness, read_color};

static LED: &str = "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/multi_intensity";

#[derive(Parser)]
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

    /// Your color: red, green, blue, etc
    #[arg(short, long)]
    color: Option<String>,

    /// Brightness: 50%, +10, =-20, or 255
    #[arg(short, long)]
    brightness: Option<String>,
}

fn main() {
    if !std::path::Path::new(LED).exists() {
        eprintln!("Error: Backlight control file not found. It appears the driver is not loaded");
        return;
    }

    let args = Arge::parse();

    let mut current_color = if args.off {
        (0, 0, 0)
    } else if let Some(rgb) = args.rgb {
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

    let (r, g, b) = current_color;
    write_color(&format!("{} {} {}", r, g, b));
}

fn write_color(color_data: &str) {
    match fs::write(LED, color_data) {
        Ok(_) => println!("Color: {}", color_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn get_color_by_name(name: &str) -> (u8, u8, u8) {
    match name.to_lowercase().as_str() {
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "blue" => (0, 0, 255),
        "white" => (255, 255, 255),
        "yellow" => (255, 255, 0),
        "cyan" => (0, 255, 255),
        "magenta" => (255, 0, 255),
        "orange" => (255, 128, 0),
        "pink" => (255, 0, 127),
        "turquoise" => (0, 128, 128),
        "violet" => (127, 0, 255),
        "lime" => (127, 255, 0),
        "golden" => (255, 180, 50),
        "gray" => (100, 100, 100),
        _ => {
            eprintln!("Unknown color: {}, using white", name);
            (255, 255, 255)
        }
    }
}
