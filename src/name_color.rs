use colored::Colorize;

pub fn get_color_by_name(name: &str) -> (u8, u8, u8) {
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
            println!(
                "Colors: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, gray",
                "red".red(),
                "green".green(),
                "blue".blue(),
                "white".truecolor(255, 255, 255),
                "yellow".yellow(),
                "cyan".cyan(),
                "magenta".magenta(),
                "orange".truecolor(255, 128, 0),
                "pink".truecolor(255, 0, 127),
                "turquoise".truecolor(0, 128, 128),
                "violet".truecolor(127, 0, 255),
                "lime".truecolor(127, 255, 0),
                "golden".truecolor(255, 180, 50),
            );
            (255, 255, 255)
        }
    }
}
