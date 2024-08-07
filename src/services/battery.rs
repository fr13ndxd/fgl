use gtk4::glib::MainContext;
use notify::{Config, PollWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::time::Duration;

pub fn battery_status_changed<F>(l: F)
where
    F: Fn(String) + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let config = Config::default()
            .with_compare_contents(true)
            .with_poll_interval(Duration::from_secs(2));

        let path = Path::new("/sys/class/power_supply/BAT0/status");

        let mut watcher = PollWatcher::new(tx, config).unwrap();
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
    });

    MainContext::default().spawn_local(async move {
        l(crate::services::battery::get_battery_status());
        for res in rx {
            match res {
                Ok(_event) => {
                    let res =
                        std::fs::read_to_string(Path::new("/sys/class/power_supply/BAT0/status"))
                            .unwrap();
                    l(res);
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}

pub fn battery_percent_changed<F>(l: F)
where
    F: Fn(String) + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let config = Config::default()
            .with_compare_contents(true)
            .with_poll_interval(Duration::from_secs(2));
        let path = Path::new("/sys/class/power_supply/BAT0/capacity");

        let mut watcher = PollWatcher::new(tx, config).unwrap();
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
    });

    MainContext::default().spawn_local(async move {
        l(crate::services::battery::get_battery_capacity().to_string());
        for res in rx {
            match res {
                Ok(_event) => {
                    let res =
                        std::fs::read_to_string(Path::new("/sys/class/power_supply/BAT0/capacity"))
                            .unwrap();
                    l(res);
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}

pub fn battery_icon_changed<F>(l: F)
where
    F: Fn(String) + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let config = Config::default()
            .with_compare_contents(true)
            .with_poll_interval(Duration::from_secs(2));

        let path = Path::new("/sys/class/power_supply/BAT0/capacity");
        let path2 = Path::new("/sys/class/power_supply/BAT0/capacity");

        let mut watcher = PollWatcher::new(tx, config).unwrap();
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
        watcher.watch(path2, RecursiveMode::Recursive).unwrap();
    });

    MainContext::default().spawn_local(async move {
        l(get_battery_icon());
        for res in rx {
            match res {
                Ok(_event) => {
                    let capacity = get_battery_capacity();
                    let status = get_battery_status();

                    if (0..=100).contains(&capacity) {
                        let capacity = capacity - (capacity % 10);
                        let res = format!("battery-level-{}{}-symbolic", capacity, status);
                        l(res);
                    }

                    l(String::from("battery-level-unknown-symbolic"));
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}

pub fn get_battery_icon() -> String {
    let path = std::path::Path::new("/sys/class/power_supply/BAT0");
    if !path.exists() {
        return String::from("battery-level-unknown-symbolic");
    }

    let status = match get_battery_status().as_str() {
        "Charging" => "-charging",
        _ => "",
    };
    let capacity = get_battery_capacity();

    if (0..=100).contains(&capacity) {
        let capacity = capacity - (capacity % 10);
        return format!("battery-level-{}{}-symbolic", capacity, status);
    } else {
        return String::from("battery-level-unknown-symbolic");
    }
}

pub fn get_battery_status() -> String {
    let path = std::path::Path::new("/sys/class/power_supply/BAT0/status");

    let sp = fs::read_to_string(path).unwrap();

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
