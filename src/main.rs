use std::fs;
// работа с файловой системой
//use std::process::Command;
// команды для терминала
//use std::thread;
//use std::time::{Duration, Instant, SystemTime};
// работа со временем
use clap::Parser;
// команды терминала

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
}

fn main() {
    if !std::path::Path::new(LED).exists() {
        eprintln!("Error: Backlight control file not found. It appears the driver is not loaded");
        return;
    }

    let args = Arge::parse();

    if args.off {
        write_color("0 0 0");
    } else if let Some(col_rgb) = args.rgb {
        let color_string = format!("{} {} {}", col_rgb[0], col_rgb[1], col_rgb[2]);
        write_color(&color_string);
    } else if let Some(color) = args.color {
        match color.as_str() {
            "red" => write_color("255 0 0"),
            "green" => write_color("0 255 0"),
            "blue" => write_color("0 0 255"),
            "white" => write_color("255 255 255"),
            "yellow" => write_color("255 255 0"),
            "cyan" => write_color("0 255 255"),
            "magenta" => write_color("255 0 255"),
            "orange" => write_color("255 128 0"),
            "pink" => write_color("255 0 127"),
            "turquoise" => write_color("0 128 128"),
            "violet" => write_color("127 0 255"),
            "lime" => write_color("127 255 0"),
            "golden" => write_color("255 180 50"),
            "gray" => write_color("100 100 100"),
            _ => eprintln!("There is no such meaning"),
        }
    } else {
        eprintln!("use --rgb <R G B> or --off. Help: --help")
    }
}

fn write_color(color_data: &str) {
    match fs::write(LED, color_data) {
        Ok(_) => println!("Color: {}", color_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
