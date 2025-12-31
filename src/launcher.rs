use crate::*;
use gpui::*;
use gpui_component::StyledExt;
use gpui_component::{
    Root,
    input::{Input, InputEvent, InputState},
};

pub struct RootView {
    focus_handle: FocusHandle,
    input: Entity<InputState>,
}

impl RootView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Run:"));
        let subscription =
            cx.subscribe_in(
                &input,
                window,
                |view, state, event, window, cx| match event {
                    InputEvent::Change => {
                        let text = state.read(cx).value();
                        println!("Input changed: {}", text);
                    }
                    InputEvent::PressEnter { secondary } => {
                        println!("Enter pressed, secondary: {}", secondary);
                    }
                    InputEvent::Focus => println!("Input focused"),
                    InputEvent::Blur => {
                        println!("Input blurred");
                        view.input.update(cx, |input, cx| input.focus(window, cx));
                    }
                },
            );
        subscription.detach();
        input.update(cx, |input, cx| input.focus(window, cx));
        Self {
            focus_handle,
            input,
        }
    }
}

actions!(actions_namespace, [Esc]);

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        let input = Input::new(&self.input)
            .border_0()
            .border_b_1()
            .rounded_none()
            .margins(Edges {
                top: px(4.),
                bottom: px(4.),
                right: px(4.),
                left: px(4.),
            });

        let mut content = div()
            .size_full()
            .border_2()
            .border_color(gpui_component::Theme::global_mut(cx).colors.border)
            .track_focus(&self.focus_handle)
            .flex()
            .items_start()
            .justify_center()
            .child(input)
            .on_action(|_: &Esc, window: &mut Window, _app: &mut App| {
                window.remove_window();
                #[cfg(target_os = "windows")]
                _app.quit();
            });

        if let Some(layer) = sheet_layer {
            content = content.child(layer);
        }
        if let Some(layer) = dialog_layer {
            content = content.child(layer);
        }
        if let Some(layer) = notification_layer {
            content = content.child(layer);
        }

        content
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

fn build_root_view(window: &mut Window, app: &mut App) -> Entity<Root> {
    app.bind_keys([KeyBinding::new("escape", Esc, None)]);
    let launcher_view = app.new(|cx| RootView::new(window, cx));
    app.new(|cx| {
        let root = Root::new(launcher_view, window, cx);
        root
    })
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
    let kind = WindowKind::Normal;
    WindowOptions {
        kind,
        focus: true,
        show: true,
        is_movable: false,
        is_resizable: false,
        display_id: None,
        window_bounds,
        window_decorations: None,
        titlebar: None,
        window_background: WindowBackgroundAppearance::Transparent,
        ..Default::default()
    }
}
