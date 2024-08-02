use std::fs;

pub fn get_battery_status() -> String {
    let path = std::path::Path::new("/sys/class/power_supply/BAT0");

    let sp = path.join("status");
    let sp = fs::read_to_string(sp).unwrap();

    if sp.trim() == "Charging" {
        return String::from("-charging");
    } else {
        return String::new();
    }
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
        let sp = get_battery_status();
        let cp = get_battery_capacity();
        let cp = cp - (cp % 10);

        if (0..=100).contains(&cp) {
            return format!("battery-level-{}{}-symbolic", cp, sp);
        }
    }

    String::from("battery-level-unknown-symbolic")
}
