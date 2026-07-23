pub mod xfreerdp_args;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

use serde::Serialize;

use crate::error::AppResult;
use crate::models::ConnectionProfile;

#[derive(Debug, Clone, Serialize)]
pub struct LauncherReadiness {
    pub available: bool,
    pub detail: Option<String>,
}

pub trait RdpLauncher {
    fn launch(
        &self,
        profile: &ConnectionProfile,
        password: Option<&str>,
        gateway_password: Option<&str>,
    ) -> AppResult<std::process::Child>;

    fn is_ready(&self) -> LauncherReadiness;
}

#[cfg(target_os = "macos")]
pub fn make_launcher() -> impl RdpLauncher {
    macos::MacLauncher
}

#[cfg(target_os = "windows")]
pub fn make_launcher() -> impl RdpLauncher {
    windows::WindowsLauncher
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn make_launcher() -> impl RdpLauncher {
    struct UnsupportedLauncher;
    impl RdpLauncher for UnsupportedLauncher {
        fn launch(
            &self,
            _profile: &ConnectionProfile,
            _password: Option<&str>,
            _gateway_password: Option<&str>,
        ) -> AppResult<std::process::Child> {
            Err(crate::error::AppError::UnsupportedPlatform)
        }
        fn is_ready(&self) -> LauncherReadiness {
            LauncherReadiness {
                available: false,
                detail: Some("unsupported platform".into()),
            }
        }
    }
    UnsupportedLauncher
}
