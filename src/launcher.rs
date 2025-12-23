use crate::quickapp::QUICK_WINDOWS;
use gpui::*;

actions!(actions_namespace, [Esc]);
pub struct RootView {
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
                let Ok(mut quick_windows) = QUICK_WINDOWS.lock() else {
                    panic!("The global window cannot be accessed.");
                };
                quick_windows.launcher = None;
            })
    }
}

pub fn build_root_view(window: &mut Window, app: &mut App) -> Entity<RootView> {
    app.bind_keys([KeyBinding::new("escape", Esc, None)]);
    let focus_handle = app.focus_handle();
    focus_handle.focus(window, app);
    app.new(|_cx| RootView { focus_handle })
}
