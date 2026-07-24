use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::error::{AppError, AppResult};
use crate::models::ConnectionProfile;

use super::{xfreerdp_args, LauncherReadiness, RdpLauncher};

pub struct MacLauncher;

/// Apps launched via Finder/Dock/Spotlight (not a terminal) get launchd's
/// minimal default PATH (`/usr/bin:/bin:/usr/sbin:/sbin`), which does NOT
/// include Homebrew's install dirs — so `which` alone misses Homebrew
/// installs unless the app happens to be launched from a terminal. Check
/// well-known install locations directly first, then fall back to PATH.
const KNOWN_BIN_DIRS: &[&str] = &[
    "/opt/homebrew/bin", // Homebrew on Apple Silicon
    "/usr/local/bin",    // Homebrew on Intel, or manual installs
    "/opt/local/bin",    // MacPorts
];

fn find_binary(name: &str) -> Option<PathBuf> {
    for dir in KNOWN_BIN_DIRS {
        let candidate = Path::new(dir).join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    which::which(name).ok()
}

/// `sdl-freerdp` uses FreeRDP's native SDL client (no XQuartz/X11 required)
/// and shares the same common cmdline flags as `xfreerdp`, so it's preferred.
/// `xfreerdp` (X11-based) is the fallback for installs that only ship it.
fn find_rdp_client() -> Option<PathBuf> {
    find_binary("sdl-freerdp")
        .or_else(|| find_binary("xfreerdp"))
        .or_else(|| find_binary("xfreerdp3"))
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
