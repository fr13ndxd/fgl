use gtk4::glib;
use gtk4::Image;
use tokio::sync::watch;

pub trait IconOptions {
    fn watch<F>(&self, delay: u64, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static;
}

impl IconOptions for Image {
    fn watch<F>(&self, delay: u64, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        let icon = self.clone();
        let (tx, mut rx) = watch::channel(String::new());

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
                let _ = rx.changed().await;
                icon.set_icon_name(Some(rx.borrow().clone().as_str()));
            }
        });
    }
}
