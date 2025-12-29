use gpui::*;
use smol::channel::Receiver;
use way_quick::*;
pub async fn run(rx: Receiver<Event>) {
    let application = Application::new().with_quit_mode(QuitMode::Explicit);
    application.run(|app| {
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
