use crate::*;
use gpui::*;
pub struct RootView {
    focus_handle: FocusHandle,
}

actions!(actions_namespace, [Esc]);

impl Render for RootView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl gpui::IntoElement {
        div()
            .size_full()
            .bg(gpui::white())
            .track_focus(&self.focus_handle)
            .on_action(|_: &Esc, window, app| {
                window.remove_window();
                #[cfg(target_os = "windows")]
                app.quit();
            })
    }
}

pub async fn start(app: &mut AsyncApp) {
    let mut launcher_window_handle = LAUNCHER_WINDOW_HANDLE.lock().await;
    if launcher_window_handle.is_some() {
        return;
    }
    let Ok(window_handle) = app.open_window(get_window_options(), build_root_view) else {
        return;
    };
    *launcher_window_handle = Some(window_handle);
}

fn build_root_view(window: &mut Window, app: &mut App) -> Entity<RootView> {
    app.bind_keys([KeyBinding::new("escape", Esc, None)]);
    let focus_handle = app.focus_handle();
    focus_handle.focus(window, app);
    app.new(|_cx| RootView { focus_handle })
}

fn get_window_options() -> WindowOptions {
    let origin = if let Some(origin) = *CENTER_POINTER.lock_blocking() {
        origin
    } else {
        Point::new(px(0.), px(0.))
    };
    let size = size(px(800.), px(420.));
    let window_bounds = Some(WindowBounds::Windowed(Bounds { origin, size }));
    #[cfg(target_os = "linux")]
    let kind = WindowKind::LayerShell(layer_shell::LayerShellOptions {
        layer: layer_shell::Layer::Overlay,
        keyboard_interactivity: layer_shell::KeyboardInteractivity::Exclusive,
        ..Default::default()
    });
    #[cfg(target_os = "windows")]
    let kind = WindowKind::PopUp;
    WindowOptions {
        kind,
        focus: true,
        show: true,
        is_movable: false,
        is_resizable: false,
        display_id: None,
        window_bounds,
        window_decorations: None,
        ..Default::default()
    }
}
