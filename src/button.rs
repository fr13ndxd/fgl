use futures::stream::StreamExt;
use glib::interval_stream;
use gtk4::glib;
use gtk4::prelude::ButtonExt;
use gtk4::Button;
use std::time::Duration;

pub trait ButtonOptions {
    fn poll<F>(&self, interval: u64, func: F)
    where
        F: Fn() -> String + 'static;

    fn watch<F>(&self, func: F)
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

    fn watch<F>(&self, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        let button = self.clone();
        let stream = interval_stream(Duration::from_millis(100));

        glib::MainContext::default().spawn_local(async move {
            button.set_label(&func());

            let mut stream = stream;
            let mut old = func();
            while let Some(_) = stream.next().await {
                let new_text = func();
                if new_text != old {
                    button.set_label(&new_text);
                    old = new_text;
                }
            }
        });
    }
}
