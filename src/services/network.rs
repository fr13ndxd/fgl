use std::process::Command;

pub fn strength() -> i32 {
    let singal = Command::new("nmcli")
        .args(["-f", "IN-USE,SIGNAL", "dev", "wifi"])
        .output()
        .unwrap();

    let signal = String::from_utf8(singal.stdout).unwrap();

    for line in signal.lines() {
        if line.starts_with("*") {
            let line = line
                .strip_prefix("*")
                .unwrap()
                .replace(" ", "")
                .parse::<i32>()
                .unwrap();

            return line;
        }
    }

    -1
}

pub fn state() -> String {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "STATE", "general", "status"])
        .output()
        .expect("Failed to execute nmcli command");

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

    status
}

pub fn get_icon_name() -> String {
    let state = state();
    if state.to_string() == "connected" {
        return match strength() {
            0..=20 => String::from("network-wireless-signal-weak"),
            21..=40 => String::from("network-wireless-signal-ok"),
            41..=60 => String::from("network-wireless-signal-good"),
            61..=80 => String::from("network-wireless-signal-excellent"),
            81..=100 => String::from("network-wireless-signal-excellent"),
            _ => String::from("network-wireless-off"),
        };
    } else if state == "connecting".to_string() {
        return "network-wireless-acquiring-symbolic".to_string();
    } else if state == "disconnected".to_string() {
        return "network-wireless-offline-symbolic".to_string();
    }

    "network-wireless-disabled-symbolic".to_string()
}
