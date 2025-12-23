use gpui::*;

actions!(actions_namespace, [Esc]);
struct RootView {
    focus_handle: FocusHandle,
}

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(white())
            .flex()
            .justify_center()
            .items_center()
            .track_focus(&self.focus_handle)
            .on_action(|action: &Esc, _window, app| {
                app.quit();
            })
    }
}

fn build_root_view(window: &mut Window, app: &mut App) -> Entity<RootView> {
    app.bind_keys([KeyBinding::new("escape", Esc, None)]);
    let focus_handle = app.focus_handle();
    focus_handle.focus(window, app);
    app.new(|_cx| RootView { focus_handle })
}

pub fn app_run(kind: WindowKind) {
    Application::new().run(|app: &mut App| {
        let bounds = Bounds::centered(None, size(px(400.), px(200.0)), app);
        let window_options = WindowOptions {
            titlebar: None,
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_decorations: None,
            kind,
            ..Default::default()
        };
        match app.open_window(window_options, build_root_view) {
            Err(e) => eprintln!("{:?}", e),
            _ => (),
        }
    });
}
