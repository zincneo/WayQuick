#[cfg(target_os = "linux")]
mod linux_module;

#[cfg(target_os = "windows")]
mod windows_module;

fn main() {
    #[cfg(target_os = "linux")]
    {
        linux_module::exec();
    }

    #[cfg(target_os = "windows")]
    {
        windows_module::exec();
    }
}
