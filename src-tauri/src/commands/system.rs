use crate::launcher::{self, LauncherReadiness, RdpLauncher};

#[tauri::command]
pub fn check_launcher_status() -> LauncherReadiness {
    launcher::make_launcher().is_ready()
}

#[tauri::command]
pub fn get_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "other"
    }
}
