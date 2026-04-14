use std::fs;
// работа с файловой системой

pub fn write_color(path: &str, r: u8, g: u8, b: u8) {
    let data = format!("{} {} {}", r, g, b);
    if let Err(e) = fs::write(path, data) {
        eprintln!("Ошибка записи в LED: {}", e);
    }
}
