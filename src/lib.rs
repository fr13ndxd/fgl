use futures::stream::StreamExt;
use glib::interval_stream;
use gtk4::glib;
use gtk4::Label;
use std::time::Duration;

pub trait LabelPoll {
    fn poll<F>(&self, interval: u64, func: F)
    where
        F: Fn() -> String + 'static;

    fn watch<F>(&self, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static;
}

impl LabelPoll for Label {
    fn poll<F>(&self, interval: u64, func: F)
    where
        F: Fn() -> String + 'static,
    {
        let label = self.clone();
        let stream = interval_stream(Duration::from_millis(interval));

        glib::MainContext::default().spawn_local(async move {
            let mut stream = stream;
            while let Some(_) = stream.next().await {
                let new_text = func();
                label.set_text(&new_text);
            }
        });
    }

    fn watch<F>(&self, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        let label = self.clone();
        let stream = interval_stream(Duration::from_millis(100));

        glib::MainContext::default().spawn_local(async move {
           let mut stream = stream;
           let mut old = func();
           while let Some(_) = stream.next().await {
               let new_text = func();
               if new_text != old {
                   label.set_text(&new_text);
                   old = new_text;
               }
           }
        });
    }
}
