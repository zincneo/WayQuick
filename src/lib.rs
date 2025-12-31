use gpui::{Pixels, Point, WindowHandle};
use gpui_component::Root;
use smol::lock::Mutex;
use std::sync::LazyLock;

pub mod launcher;
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Launcher,
}

type Singleton<T> = LazyLock<Mutex<Option<T>>>;

pub static LAUNCHER_WINDOW_HANDLE: Singleton<WindowHandle<Root>> = LazyLock::new(|| Mutex::default());

pub static CENTER_POINTER: Singleton<Point<Pixels>> = LazyLock::new(|| Mutex::default());
