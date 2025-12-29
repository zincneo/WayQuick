use gpui::WindowHandle;
use smol::lock::Mutex;
use std::sync::LazyLock;

pub mod launcher;
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Launcher,
}

#[derive(Default)]
pub struct Global {
    pub launcher: Option<WindowHandle<launcher::RootView>>,
}

pub static GLOBAL: LazyLock<Mutex<Global>> = LazyLock::new(|| Mutex::default());
