use crate::utils::write_color;

use std::thread::sleep;
// работа со временем
use std::time::Duration;

pub fn hsv(pash_led: &str) {
    let mut hue = 0.0;

    loop {
        let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);

        write_color(pash_led, r, g, b);

        hue += 0.01;
        if hue > 1.0 {
            hue = 0.0;
        }
        sleep(Duration::from_millis(50));
    }
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - i as f32;

    let p = (v * (1.0 - s) * 255.0) as u8;
    let q = (v * (1.0 - s * f) * 255.0) as u8;
    let t = (v * (1.0 - s * (1.0 - f)) * 255.0) as u8;
    let v_u8 = (v * 255.0) as u8;

    match i % 6 {
        0 => (v_u8, t, p),
        1 => (q, v_u8, p),
        2 => (p, v_u8, t),
        3 => (p, q, v_u8),
        4 => (t, p, v_u8),
        _ => (v_u8, p, q),
    }
}
