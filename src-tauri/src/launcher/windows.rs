use std::fs;
use std::process::Command;

use crate::error::AppResult;
use crate::models::ConnectionProfile;
use crate::rdpfile;

use super::{LauncherReadiness, RdpLauncher};

pub struct WindowsLauncher;

impl RdpLauncher for WindowsLauncher {
    fn launch(
        &self,
        profile: &ConnectionProfile,
        _password: Option<&str>,
        _gateway_password: Option<&str>,
    ) -> AppResult<std::process::Child> {
        // mstsc has no supported way to accept a password non-interactively;
        // it will prompt, or auto-fill if Windows Credential Manager already
        // has a saved TERMSRV/<host> entry (from a prior mstsc "remember me").
        let content = rdpfile::generate(profile);

        let mut tmp_dir = std::env::temp_dir();
        tmp_dir.push("rdpmanager");
        fs::create_dir_all(&tmp_dir)?;

        let file_name = format!("{}-{}.rdp", profile.id, chrono::Utc::now().timestamp());
        let file_path = tmp_dir.join(file_name);
        fs::write(&file_path, content)?;

        // Signing is best-effort: if it fails (no cert yet, rdpsign missing),
        // fall back to launching the unsigned file rather than blocking the
        // connection. The user just sees the "unknown publisher" prompt.
        if let Ok(thumbprint) = crate::signing::get_or_create_thumbprint() {
            let _ = crate::signing::sign_rdp_file(&file_path, &thumbprint);
        }

        let child = Command::new("mstsc.exe").arg(&file_path).spawn()?;

        Ok(child)
    }

    fn is_ready(&self) -> LauncherReadiness {
        LauncherReadiness {
            available: true,
            detail: Some("mstsc.exe (built-in)".into()),
        }
    }
}
