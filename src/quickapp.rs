use core::panic;
use gpui::*;
use std::sync::{LazyLock, Mutex, mpsc};

pub enum Event {
    Launcher(WindowKind),
}

pub struct QuickWindows {
    pub launcher: Option<WindowHandle<crate::launcher::RootView>>,
}

impl Default for QuickWindows {
    fn default() -> Self {
        Self { launcher: None }
    }
}

pub static QUICK_WINDOWS: LazyLock<Mutex<QuickWindows>> =
    LazyLock::new(|| Mutex::new(QuickWindows::default()));

pub fn app_run(receiver: mpsc::Receiver<Event>) {
    Application::new().run(move |app: &mut App| {
        loop {
            let Ok(event) = receiver.recv() else {
                break;
            };
            match event {
                Event::Launcher(kind) => {
                    let Ok(mut quick_windows) = QUICK_WINDOWS.lock() else {
                        panic!("The global window cannot be accessed.");
                    };
                    if let None = quick_windows.launcher {
                        let bounds = Bounds::centered(None, size(px(400.), px(200.0)), app);
                        let window_options = WindowOptions {
                            titlebar: None,
                            window_bounds: Some(WindowBounds::Windowed(bounds)),
                            window_decorations: None,
                            kind,
                            ..Default::default()
                        };
                        if let Ok(window_handle) =
                            app.open_window(window_options, crate::launcher::build_root_view)
                        {
                            quick_windows.launcher = Some(window_handle);
                        }
                    }
                }
            }
        }
    });
}
