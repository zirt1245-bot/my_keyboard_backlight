use std::fs;

pub enum BrightnessCmd {
    Absolute(u8),
    Relative(i16),
    Percent(f32),
    Max,
}

pub fn read_color(path: &str) -> (u8, u8, u8) {
    // узнаем  значение текущего цвета
    let content = fs::read_to_string(path).unwrap_or("0 0 0".to_string());
    let nums: Vec<u8> = content
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if nums.len() == 3 {
        (nums[0], nums[1], nums[2])
    } else {
        (0, 0, 0)
    }
}

pub fn parse_brightness(s: &str) -> Option<BrightnessCmd> {
    // разбиваем формат команд
    if s.to_lowercase() == "max" {
        return Some(BrightnessCmd::Max);
    }

    if s.ends_with("%") {
        let val: f32 = s.trim_end_matches("%").parse().ok()?;
        Some(BrightnessCmd::Percent(val))
    } else if s.starts_with('+') || s.starts_with('-') {
        let val: i16 = s.parse().ok()?;
        Some(BrightnessCmd::Relative(val))
    } else {
        let val: u8 = s.parse().ok()?;
        Some(BrightnessCmd::Absolute(val))
    }
}

pub fn apply_brightness(r: u8, g: u8, b: u8, cmd: &BrightnessCmd) -> (u8, u8, u8) {
    match cmd {
        BrightnessCmd::Max => {
            let max = r.max(g).max(b) as f32;
            if max == 0.0 {
                return (0, 0, 0);
            }
            let factor = 255.0 / max;
            let apply = |c: u8| (c as f32 * factor).clamp(0.0, 255.0) as u8;
            (apply(r), apply(g), apply(b))
        }

        BrightnessCmd::Absolute(v) => (*v, *v, *v),

        BrightnessCmd::Percent(pct) => {
            let factor = pct / 100.0;
            let apply = |c: u8| (c as f32 * factor).clamp(0.0, 255.0) as u8;
            (apply(r), apply(g), apply(b))
        }

        BrightnessCmd::Relative(delta) => {
            let max = r.max(g).max(b) as i16;
            let new_max = (max + delta).clamp(0, 255) as f32;
            if max == 0 {
                return (0, 0, 0);
            }
            let factor = new_max / max as f32;
            let apply = |c: u8| (c as f32 * factor).clamp(0.0, 255.0) as u8;
            (apply(r), apply(g), apply(b))
        }
    }
}
