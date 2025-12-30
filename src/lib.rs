use gpui::WindowHandle;
use smol::lock::Mutex;
use std::sync::LazyLock;

pub mod launcher;
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Launcher,
}

pub static LAUNCHER_WINDOW_HANDLE: LazyLock<Mutex<Option<WindowHandle<launcher::RootView>>>> =
    LazyLock::new(|| Mutex::default());
