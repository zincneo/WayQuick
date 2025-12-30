use gpui::WindowHandle;
use smol::lock::Mutex;
use std::sync::LazyLock;

pub mod launcher;
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Launcher,
}

type Singleton<T> = LazyLock<Mutex<Option<T>>>;

pub static LAUNCHER_WINDOW_HANDLE: Singleton<WindowHandle<launcher::RootView>> =
    LazyLock::new(|| Mutex::default());
