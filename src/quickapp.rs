use core::panic;
use gpui::*;
use std::sync::{LazyLock, Mutex, mpsc};

pub enum Event {
    Launcher(WindowKind),
}

pub static QUICK_WINDOWS: LazyLock<Mutex<QuickWindows>> =
    LazyLock::new(|| Mutex::new(QuickWindows::default()));

impl Event {
    fn launcher_start(kind: WindowKind, app: &mut App) {
        let Ok(mut quick_windows) = QUICK_WINDOWS.lock() else {
            panic!("The global window cannot be accessed.");
        };
        if let Some(_) = quick_windows.launcher {
            return;
        }
        let bounds = Bounds::centered(None, size(px(400.), px(200.0)), app);
        let window_options = WindowOptions {
            titlebar: None,
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_decorations: None,
            kind,
            ..Default::default()
        };
        let Ok(window_handle) = app.open_window(window_options, crate::launcher::build_root_view)
        else {
            return;
        };
        quick_windows.launcher = Some(window_handle);
    }
}

pub struct QuickWindows {
    pub launcher: Option<WindowHandle<crate::launcher::RootView>>,
}

impl Default for QuickWindows {
    fn default() -> Self {
        Self { launcher: None }
    }
}

pub fn app_run(receiver: mpsc::Receiver<Event>) {
    let on_finish_launching = move |app: &mut App| {
        while let Ok(event) = receiver.recv() {
            match event {
                Event::Launcher(kind) => Event::launcher_start(kind, app),
            }
        }
    };
    Application::new().run(on_finish_launching);
}
