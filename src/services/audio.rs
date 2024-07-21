use std::process::Command;

pub fn is_muted() -> bool {
    let output = Command::new("pactl")
        .args(["get-sink-mute", "@DEFAULT_SINK@"])
        .output()
        .unwrap();
    let output = std::str::from_utf8(&output.stdout).unwrap();

    output.contains("yes")
}

pub fn get_volume() -> i32 {
    let output = Command::new("pactl")
        .args(["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
        .unwrap();

    let output = std::str::from_utf8(&output.stdout).unwrap();

    for line in output.lines() {
        if let Some(vol_str) = line.split_whitespace().nth(4) {
            let vol = vol_str
                .trim_end_matches('%')
                .parse::<i32>()
                .expect("Failed to parse volume");
            return vol;
        }
    }

    0
}

pub fn get_audio_icon() -> String {
    if is_muted() {
        return String::from("audio-volume-muted-symbolic");
    }

    let volume = get_volume();

    return match volume {
        0 => String::from("audio-volume-muted-symbolic"),
        1..=34 => String::from("audio-volume-low-symbolic"),
        35..=67 => String::from("audio-volume-medium-symbolic"),
        68..=100 => String::from("audio-volume-high-symbolic"),
        _ => String::from("audio-volume-overamplified-symbolic"),
    };
}
