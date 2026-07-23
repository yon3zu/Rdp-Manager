use std::process::{Command, Stdio};

use crate::error::{AppError, AppResult};
use crate::models::ConnectionProfile;

use super::{xfreerdp_args, LauncherReadiness, RdpLauncher};

pub struct MacLauncher;

/// `sdl-freerdp` uses FreeRDP's native SDL client (no XQuartz/X11 required)
/// and shares the same common cmdline flags as `xfreerdp`, so it's preferred.
/// `xfreerdp` (X11-based) is the fallback for installs that only ship it.
fn find_rdp_client() -> Option<std::path::PathBuf> {
    which::which("sdl-freerdp")
        .or_else(|_| which::which("xfreerdp"))
        .or_else(|_| which::which("xfreerdp3"))
        .ok()
}

impl RdpLauncher for MacLauncher {
    fn launch(
        &self,
        profile: &ConnectionProfile,
        password: Option<&str>,
        gateway_password: Option<&str>,
    ) -> AppResult<std::process::Child> {
        let binary = find_rdp_client().ok_or(AppError::XfreerdpNotFound)?;
        let args = xfreerdp_args::build(profile, password, gateway_password)?;

        let child = Command::new(binary)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        Ok(child)
    }

    fn is_ready(&self) -> LauncherReadiness {
        match find_rdp_client() {
            Some(path) => LauncherReadiness {
                available: true,
                detail: Some(path.display().to_string()),
            },
            None => LauncherReadiness {
                available: false,
                detail: Some(
                    "No RDP client found on PATH. Install with: brew install freerdp".into(),
                ),
            },
        }
    }
}
