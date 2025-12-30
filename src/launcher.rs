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
            .on_action(|_: &Esc, window, _| {
                window.remove_window();
            })
    }
}

pub async fn start(app: &mut AsyncApp) {
    let mut launcher_window_handle = LAUNCHER_WINDOW_HANDLE.lock().await;
    if launcher_window_handle.is_some() {
        return;
    }
    let Ok(window_handle) = app.open_window(
        WindowOptions {
            kind: WindowKind::LayerShell(layer_shell::LayerShellOptions {
                layer: layer_shell::Layer::Overlay,
                keyboard_interactivity: layer_shell::KeyboardInteractivity::Exclusive,
                ..Default::default()
            }),
            focus: true,
            ..Default::default()
        },
        build_root_view,
    ) else {
        return;
    };
    *launcher_window_handle = Some(window_handle);
}

fn build_root_view(_window: &mut Window, app: &mut App) -> Entity<RootView> {
    app.bind_keys([KeyBinding::new("escape", Esc, None)]);
    let focus_handle = app.focus_handle();
    app.new(|_cx| RootView { focus_handle })
}
