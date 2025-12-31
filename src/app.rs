use gpui::*;
use gpui_component::{Theme, ThemeRegistry};
use smol::channel::Receiver;
use way_quick::*;
pub async fn run(rx: Receiver<Event>) {
    let application = Application::new().with_quit_mode(QuitMode::Explicit);
    application.run(move |app| {
        gpui_component::init(app);
        let theme_name = SharedString::from("Catppuccin Macchiato");
        ThemeRegistry::watch_dir(std::path::PathBuf::from("./themes"), app, move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
                Theme::global_mut(cx).apply_config(&theme);
            }
        })
        .unwrap();

        let bounds = Bounds::centered(None, size(px(800.), px(420.)), app);
        *CENTER_POINTER.lock_blocking() = Some(bounds.origin);
        app.on_window_closed(on_closed).detach();

        let app = app.to_async();
        app.spawn(async move |app| {
            event_handle(app, rx).await;
        })
        .detach();
    });
}

async fn event_handle(app: &mut gpui::AsyncApp, rx: Receiver<Event>) {
    while let Ok(event) = rx.recv().await {
        match event {
            Event::Launcher => {
                app.spawn(async |app| {
                    launcher::start(app).await;
                })
                .detach();
            }
        }
    }
}

fn on_closed(app: &mut App) {
    let windows = app.windows();
    let mut launcher_window_handle = LAUNCHER_WINDOW_HANDLE.lock_blocking();
    let launcher_window_id = if launcher_window_handle.is_some() {
        launcher_window_handle.unwrap().window_id()
    } else {
        return;
    };
    if windows
        .iter()
        .filter(|window| {
            let Some(window_handle) = window.downcast::<WindowHandle<gpui_component::Root>>()
            else {
                return false;
            };
            launcher_window_id == window_handle.window_id()
        })
        .collect::<Vec<_>>()
        .len()
        == 0
    {
        *launcher_window_handle = None;
        std::mem::drop(launcher_window_handle);
    }
}
