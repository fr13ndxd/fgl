use futures::stream::StreamExt;
use glib::interval_stream;
use gtk4::glib;
use gtk4::prelude::ButtonExt;
use gtk4::Button;
use std::time::Duration;
use tokio::sync::watch;

pub trait ButtonOptions {
    fn poll<F>(&self, interval: u64, func: F)
    where
        F: Fn() -> String + 'static;

    fn watch<F>(&self, delay: u64, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static;
}

impl ButtonOptions for Button {
    fn poll<F>(&self, interval: u64, func: F)
    where
        F: Fn() -> String + 'static,
    {
        let button = self.clone();
        let stream = interval_stream(Duration::from_millis(interval));

        glib::MainContext::default().spawn_local(async move {
            let mut stream = stream;
            while let Some(_) = stream.next().await {
                let new_text = func();
                button.set_label(&new_text);
            }
        });
    }

    fn watch<F>(&self, delay: u64, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        let button = self.clone();
        let (tx, mut rx) = watch::channel(func());

        let func = std::sync::Arc::new(func);

        tokio::spawn({
            let func = func.clone();
            async move {
                let mut last = func();
                let _ = tx.send(last.clone());
                loop {
                    let current = func();
                    if current != last {
                        let _ = tx.send(current.clone());
                        last = current;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                }
            }
        });

        glib::MainContext::default().spawn_local(async move {
            loop {
                match rx.changed().await {
                    Ok(()) => button.set_label(rx.borrow().as_str()),
                    Err(_) => break,
                }
            }
        });
    }
}
