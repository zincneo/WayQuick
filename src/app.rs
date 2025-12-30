use gpui::*;
use smol::channel::Receiver;
use way_quick::*;
pub async fn run(rx: Receiver<Event>) {
    let application = Application::new().with_quit_mode(QuitMode::Explicit);
    application.run(|app| {
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
    if windows
        .iter()
        .filter(|window| {
            window
                .downcast::<WindowHandle<launcher::RootView>>()
                .is_none()
        })
        .collect::<Vec<_>>()
        .len()
        == 0
    {
        let mut launcher_window_handle = LAUNCHER_WINDOW_HANDLE.lock_blocking();
        *launcher_window_handle = None;
        std::mem::drop(launcher_window_handle);
    }
}
