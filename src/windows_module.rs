use crate::quickapp::Event;
use std::sync::mpsc;
use std::thread;

pub fn exec() {
    let (tx, rx) = mpsc::channel::<Event>();
    let handles = [
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(500));
            tx.send(Event::Launcher(gpui::WindowKind::PopUp));
        }),
        thread::spawn(move || {
            crate::quickapp::app_run(rx);
        }),
    ];
    for handle in handles {
        handle.join();
    }
}
