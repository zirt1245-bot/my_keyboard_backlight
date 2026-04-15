use std::fs;
// работа с файловой системой
use std::io::Write;
use std::process::{Command, Stdio};

pub fn write_color(path: &str, r: u8, g: u8, b: u8) {
    let data = format!("{} {} {}", r, g, b);
    if let Err(e) = fs::write(path, data) {
        eprintln!("Recording error: {}", e);
    }
}

pub fn off_on(path: &str, b: u8) {
    let mut child = match Command::new("sudo")
        .arg("tee")
        .arg(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
    {
        Ok(ok) => ok,
        Err(e) => {
            eprintln!("Error command: {}", e);
            return;
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        match stdin.write_all(b.to_string().as_bytes()) {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("Error stdin: {}", e);
                return;
            }
        }
    }

    if let Err(e) = child.wait() {
        eprintln!("Process waiting error: {}", e);
    }
}
