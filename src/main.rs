mod brightness;
mod name_color;
mod rainbow;
mod utils;

//use std::process::Command;
// команды для терминала
use clap::Parser;
// команды терминала
use brightness::{apply_brightness, parse_brightness, read_color};
use name_color::{get_color_by_name, get_color_by_russian_name};
use rainbow::hsv;
use utils::write_color;

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

    /// Your color in russian: красный, зеленный, синий и т.д.
    #[arg(long)]
    color_r: Option<String>,

    /// Brightness: 50%, +10, =-20, or 255
    #[arg(short, long)]
    brightness: Option<String>,

    /// Rainbow
    #[arg(short, long)]
    rainbow: bool,
}

fn main() {
    if !std::path::Path::new(LED).exists() {
        // проверка на наличие нужного драйвера
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
    } else if let Some(color_r_name) = args.color_r {
        get_color_by_russian_name(&color_r_name)
    } else if args.rainbow {
        hsv(LED);
        return;
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
    write_color(LED, r, g, b);
}
