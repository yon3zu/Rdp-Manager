use std::fs;

use tauri::{AppHandle, Emitter, State};

use crate::credentials;
use crate::db::{profiles_repo, DbState};
use crate::error::AppResult;
use crate::launcher::{self, RdpLauncher};
use crate::rdpfile;
use crate::sessions::SessionsState;

#[tauri::command]
pub fn launch_connection(
    app: AppHandle,
    db: State<DbState>,
    sessions: State<SessionsState>,
    profile_id: String,
) -> AppResult<()> {
    let profile = {
        let conn = db.0.lock().unwrap();
        profiles_repo::get(&conn, &profile_id)?
    };

    let password = credentials::get_password(&profile_id)?;
    let gateway_password = credentials::get_gateway_password(&profile_id)?;

    let launcher = launcher::make_launcher();
    let mut child = launcher.launch(&profile, password.as_deref(), gateway_password.as_deref())?;
    let pid = child.id();

    sessions.mark_started(&profile_id, pid);
    let _ = app.emit("session-started", profile_id.clone());

    let sessions_clone = sessions.inner().clone();
    let app_handle = app.clone();
    let watched_id = profile_id.clone();
    std::thread::spawn(move || {
        // Blocks until the RDP client process (mstsc/xfreerdp/sdl-freerdp) exits,
        // i.e. the user closed the window, disconnected, or we killed it via
        // disconnect_session.
        let _ = child.wait();
        if sessions_clone.mark_ended(&watched_id, pid) {
            let _ = app_handle.emit("session-ended", watched_id);
        }
    });

    Ok(())
}

#[tauri::command]
pub fn list_active_sessions(sessions: State<SessionsState>) -> Vec<String> {
    sessions.active_profile_ids()
}

/// Force-closes every RDP client process currently running for this profile.
/// The background waiter thread in `launch_connection` observes the exit and
/// emits `session-ended` as usual, so the frontend doesn't need a separate
/// optimistic update.
#[tauri::command]
pub fn disconnect_session(sessions: State<SessionsState>, profile_id: String) -> AppResult<()> {
    for pid in sessions.pids_for(&profile_id) {
        kill_pid(pid);
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn kill_pid(pid: u32) {
    let _ = std::process::Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/F"])
        .status();
}

#[cfg(not(target_os = "windows"))]
fn kill_pid(pid: u32) {
    let _ = std::process::Command::new("kill")
        .arg(pid.to_string())
        .status();
}

/// Brings the RDP client window for this profile's active session to the
/// foreground, so clicking a Dashboard card jumps straight to that session
/// instead of just selecting it in the editor.
#[tauri::command]
pub fn focus_session(sessions: State<SessionsState>, profile_id: String) -> AppResult<()> {
    let pids = sessions.pids_for(&profile_id);
    log_focus_attempt(&profile_id, &pids);
    if let Some(&pid) = pids.first() {
        focus_pid(pid);
    }
    Ok(())
}

/// Temporary diagnostic trail for the "wrong window gets focused" report —
/// lets us see exactly which pid(s) the backend resolved for a given click
/// without needing to reproduce it live. Safe to remove once that's closed out.
fn log_focus_attempt(profile_id: &str, pids: &[u32]) {
    use std::io::Write;
    let mut path = std::env::temp_dir();
    path.push("rdpmanager-focus.log");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let now = chrono::Utc::now().to_rfc3339();
        let _ = writeln!(f, "{now} profile_id={profile_id} pids={pids:?}");
    }
}

#[cfg(target_os = "macos")]
fn focus_pid(pid: u32) {
    // AppleScript's System Events "AXMinimized" approach requires the target
    // process to expose proper AX windows — SDL-based apps like sdl-freerdp
    // (launched as a raw binary, not a real .app bundle) don't, so that
    // approach can never deminiaturize them even with Accessibility granted.
    // NSRunningApplication.activateWithOptions(.activateAllWindows) is a
    // higher-level Cocoa API that reliably brings all of an app's windows
    // forward — including restoring minimized ones — regardless of whether
    // AX exposes them, and only needs ordinary process access (no special
    // permission prompt).
    use objc2_app_kit::{NSApplicationActivationOptions, NSRunningApplication};

    let Some(app) = NSRunningApplication::runningApplicationWithProcessIdentifier(pid as i32)
    else {
        log_focus_pid_result(pid, "no NSRunningApplication for pid");
        return;
    };
    let activated = app.activateWithOptions(NSApplicationActivationOptions::ActivateAllWindows);
    log_focus_pid_result(pid, &format!("activateWithOptions -> {activated}"));
}

/// Temporary diagnostic trail alongside log_focus_attempt.
fn log_focus_pid_result(pid: u32, detail: &str) {
    use std::io::Write;
    let mut path = std::env::temp_dir();
    path.push("rdpmanager-focus.log");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let now = chrono::Utc::now().to_rfc3339();
        let _ = writeln!(f, "{now} focus_pid={pid} {detail}");
    }
}

#[cfg(target_os = "windows")]
fn focus_pid(pid: u32) {
    // SetForegroundWindow from an unrelated process is subject to Windows'
    // foreground-lock restrictions and isn't 100% guaranteed to succeed in
    // every situation, but this is the standard approach and works in the
    // common case. ShowWindow(9) = SW_RESTORE, in case the window is minimized.
    let script = format!(
        r#"
$proc = Get-Process -Id {pid} -ErrorAction SilentlyContinue
if ($proc -and $proc.MainWindowHandle -ne 0) {{
    Add-Type -Name Win32ShowWindowAsync -Namespace Win32Functions -MemberDefinition '
        [DllImport("user32.dll")]
        public static extern bool SetForegroundWindow(IntPtr hWnd);
        [DllImport("user32.dll")]
        public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    '
    [Win32Functions.Win32ShowWindowAsync]::ShowWindow($proc.MainWindowHandle, 9)
    [Win32Functions.Win32ShowWindowAsync]::SetForegroundWindow($proc.MainWindowHandle)
}}
"#
    );
    let _ = std::process::Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .status();
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn focus_pid(_pid: u32) {}

#[tauri::command]
pub fn export_rdp_file(db: State<DbState>, profile_id: String, dest_path: String) -> AppResult<()> {
    let conn = db.0.lock().unwrap();
    let profile = profiles_repo::get(&conn, &profile_id)?;
    drop(conn);

    let content = rdpfile::generate(&profile);
    fs::write(dest_path, content)?;
    Ok(())
}
