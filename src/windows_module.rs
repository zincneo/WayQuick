use crate::launcher;

pub fn exec() {
    launcher::app_run(gpui::WindowKind::Dialog);
}
