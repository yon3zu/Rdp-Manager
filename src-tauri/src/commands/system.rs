use crate::error::{AppError, AppResult};
use crate::launcher::{self, LauncherReadiness, RdpLauncher};

#[tauri::command]
pub fn check_launcher_status() -> LauncherReadiness {
    launcher::make_launcher().is_ready()
}

/// Creates (if needed) and returns the thumbprint of the local certificate
/// used to sign generated .rdp files, so mstsc's "unknown publisher"
/// warning can be suppressed once the thumbprint is trusted via Group
/// Policy. Windows only.
#[tauri::command]
pub fn get_signing_thumbprint() -> AppResult<String> {
    #[cfg(target_os = "windows")]
    {
        crate::signing::get_or_create_thumbprint()
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err(AppError::UnsupportedPlatform)
    }
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
