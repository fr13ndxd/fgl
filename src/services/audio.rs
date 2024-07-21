use std::process::Command;

pub fn get_volume() -> i32 {
    let output = Command::new("pactl")
        .args(["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(output.stdout());

    for line in output.lines() {
        if let Some(volume_str) = line.split_whitespace().nth(1) {
            let volume_percentage: u32 = volume_str
                .trim_end_matches('%')
                .parse()
                .expect("Failed to parse volume");
            return volume_percentage;
        }
    }

    -1
}

pub fn get_audio_icon() -> String {
    let volume = get_volume();

    return match volume {
        0 => String::from("audio-volume-muted-symbolic"),
        1..34 => String::from("audio-volume-low-symbolic"),
        34..67 => String::from("audio-volume-high-symbolic"),
        67.. => String::from("audio-volume-overamplified-symbolic"),
    };
}
