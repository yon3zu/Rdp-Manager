//! Signs generated .rdp files with a local self-signed certificate so
//! Windows' "unknown publisher" security warning can be suppressed once the
//! certificate's thumbprint is trusted via Group Policy (see Settings page).
//! Windows-only: rdpsign.exe and cert store access don't exist elsewhere.
#![cfg(target_os = "windows")]

use std::path::Path;
use std::process::Command;

use crate::error::{AppError, AppResult};

const CERT_SUBJECT: &str = "CN=RDP Manager Local Signing";

fn run_powershell(script: &str) -> AppResult<String> {
    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(AppError::InvalidInput(format!(
            "powershell command failed: {stderr}"
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Returns the thumbprint of the local signing certificate, creating one
/// (self-signed, code-signing EKU, in the current user's cert store) if it
/// doesn't exist yet.
pub fn get_or_create_thumbprint() -> AppResult<String> {
    let script = format!(
        r#"
$cert = Get-ChildItem Cert:\CurrentUser\My -CodeSigningCert |
    Where-Object {{ $_.Subject -eq '{CERT_SUBJECT}' }} |
    Select-Object -First 1
if (-not $cert) {{
    $cert = New-SelfSignedCertificate -Subject '{CERT_SUBJECT}' -Type CodeSigningCert `
        -CertStoreLocation Cert:\CurrentUser\My -KeyUsage DigitalSignature `
        -KeyExportPolicy Exportable -NotAfter (Get-Date).AddYears(10)
}}
$cert.Thumbprint
"#
    );
    let thumbprint = run_powershell(&script)?;
    if thumbprint.is_empty() {
        return Err(AppError::InvalidInput(
            "failed to create or read signing certificate".into(),
        ));
    }
    Ok(thumbprint)
}

/// Signs the given .rdp file in place with the local signing certificate.
/// Callers should treat failure as non-fatal and fall back to launching the
/// unsigned file — a signing failure shouldn't block a connection.
pub fn sign_rdp_file(path: &Path, thumbprint: &str) -> AppResult<()> {
    let status = Command::new("rdpsign.exe")
        .args(["/sha256", thumbprint, "/q"])
        .arg(path)
        .status()?;

    if !status.success() {
        return Err(AppError::InvalidInput("rdpsign.exe failed".into()));
    }
    Ok(())
}
