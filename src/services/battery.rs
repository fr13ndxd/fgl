use std::fs;

pub fn get_battery_status() -> String {
    let path = std::path::Path::new("/sys/class/power_supply/BAT0");

    let sp = path.join("status");
    let sp = fs::read_to_string(sp).unwrap();

    sp.trim().to_string()
}

pub fn get_battery_capacity() -> i32 {
    let path = std::path::Path::new("/sys/class/power_supply/BAT0/capacity");
    let capacity = fs::read_to_string(path)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    capacity
}

pub fn get_battery_icon() -> String {
    let path = std::path::Path::new("/sys/class/power_supply/BAT0");
    if path.exists() {
        let sp = match get_battery_status().as_str() {
            "Charging" => "-charging",
            _ => "",
        };
        let bc = get_battery_capacity();

        if (0..=100).contains(&bc) {
            let bc = bc - (bc % 10);
            return format!("battery-level-{}{}-symbolic", bc, sp);
        }
    }

    String::from("battery-level-unknown-symbolic")
}
