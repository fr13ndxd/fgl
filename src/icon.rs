use futures::stream::StreamExt;
use glib::interval_stream;
use gtk4::glib;
use gtk4::Image;
use std::time::Duration;

pub trait IconOptions {
    fn watch<F>(&self, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static;
}

impl IconOptions for Image {
    fn watch<F>(&self, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        let icon = self.clone();
        let stream = interval_stream(Duration::from_millis(100));

        icon.set_icon_name(Some(func().as_str()));

        glib::MainContext::default().spawn_local(async move {
            let mut stream = stream;
            let mut old = func();
            while let Some(_) = stream.next().await {
                let new_icon = func();
                if new_icon != old {
                    icon.set_icon_name(Some(&new_icon));
                    old = new_icon;
                }
            }
        });
    }
}
