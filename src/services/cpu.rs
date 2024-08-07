use gtk4::glib::MainContext;
use notify::{Config, PollWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::time::Duration;

pub fn cpu_temp_changed<F>(l: F)
where
    F: Fn(String) + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let config = Config::default()
            .with_compare_contents(true)
            .with_poll_interval(Duration::from_secs(2));

        // for i in 0..=20 {}
        // let path = Path::new("/sys/thermal/thermal_zone{}/temp");
        let paths: Vec<String> = (0..=10)
            .filter_map(|i| Some(format!("/sys/class/thermal/thermal_zone{}/temp", i)))
            .collect();

        let mut watcher = PollWatcher::new(tx, config).unwrap();

        for path in paths {
            watcher
                .watch(Path::new(&path), RecursiveMode::Recursive)
                .unwrap();
        }
    });

    MainContext::default().spawn_local(async move {
        l(get_cpu_temp());
        for res in rx {
            match res {
                Ok(_event) => l(get_cpu_temp()),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}

pub fn get_cpu_temp() -> String {
    let temps: Vec<f64> = (1..=10)
        .filter_map(|i| {
            let path = format!("/sys/class/thermal/thermal_zone{}/temp", i);
            fs::read_to_string(&path)
                .ok()
                .and_then(|temp| temp.trim().parse::<f64>().ok())
                .map(|temp| temp / 1000.0)
        })
        .collect();

    if temps.is_empty() {
        String::new()
    } else {
        let average_temp = temps.iter().sum::<f64>() / temps.len() as f64;
        format!("{:.1}°C", average_temp)
    }
}
