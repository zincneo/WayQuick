use crate::*;
use gpui::*;
pub struct RootView {
    focus_handle: FocusHandle,
}

actions!(actions_namespace, [Esc]);

impl Render for RootView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::div()
            .size_full()
            .bg(gpui::white())
            .track_focus(&self.focus_handle)
            .on_action(|action: &Esc, window, _| {
                println!("action name: {:?}", action.name());
                let mut global = GLOBAL.lock_blocking();
                global.launcher = None;
                window.remove_window();
            })
    }
}

pub async fn start(app: &mut AsyncApp) {
    let mut global = GLOBAL.lock().await;
    if global.launcher.is_some() {
        return;
    }
    let Ok(window_handle) = app.open_window(WindowOptions::default(), build_root_view) else {
        return;
    };
    global.launcher = Some(window_handle);
}

fn build_root_view(_window: &mut Window, app: &mut App) -> Entity<RootView> {
    app.bind_keys([KeyBinding::new("escape", Esc, None)]);
    let focus_handle = app.focus_handle();
    app.new(|_cx| RootView { focus_handle })
}
