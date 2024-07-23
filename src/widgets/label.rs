use gtk4::glib;
use gtk4::Label;
use tokio::sync::watch;

pub trait LabelOptions {
    fn watch<F>(&self, delay: u64, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static;
}

impl LabelOptions for Label {
    fn watch<F>(&self, delay: u64, func: F)
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        let label = self.clone();
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
                    Ok(()) => label.set_label(rx.borrow().as_str()),
                    Err(_) => break,
                }
            }
        });
    }
}
