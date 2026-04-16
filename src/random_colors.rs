use crate::utils::write_color;

use rand::random;
use std::thread;
use std::time::Duration;

pub fn rand_color(path: &str, s: u8) {
    loop {
        let r = random::<u8>();
        let g = random::<u8>();
        let b = random::<u8>();

        write_color(path, r, g, b);

        thread::sleep(Duration::from_secs(s.into()));
    }
}
