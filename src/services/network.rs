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
    let nm = network_manager::NetworkManager::new();
    match nm.get_state() {
        Ok(state) => format!("{:?}", state),
        Err(_) => format!("error"),
    }
}

pub fn get_icon_name() -> String {
    let state = state();
    if state.to_string() == "ConnectedGlobal" {
        return match strength() {
            0..=20 => String::from("network-wireless-signal-weak-symbolic"),
            21..=40 => String::from("network-wireless-signal-ok-symbolic"),
            41..=60 => String::from("network-wireless-signal-good-symbolic"),
            61..=80 => String::from("network-wireless-signal-excellent-symbolic"),
            81..=100 => String::from("network-wireless-signal-excellent-symbolic"),
            _ => String::from("network-wireless-off-symbolic"),
        };
    } else if state == "Connecting".to_string() {
        return String::from("network-wireless-acquiring-symbolic");
    } else if state == "Disconnected".to_string() {
        return String::from("network-wireless-offline-symbolic");
    }

    String::from("network-wireless-disabled-symbolic")
}
