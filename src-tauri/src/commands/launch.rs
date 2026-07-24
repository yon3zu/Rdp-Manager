use std::fs;

use tauri::{AppHandle, Emitter, State};

use crate::credentials;
use crate::db::{profiles_repo, DbState};
use crate::error::AppResult;
use crate::launcher::{self, RdpLauncher};
use crate::rdpfile;
use crate::sessions::{self, SessionsState};

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

    sessions::mark_started(&sessions.0, &profile_id, pid);
    let _ = app.emit("session-started", profile_id.clone());

    let sessions_map = sessions.0.clone();
    let app_handle = app.clone();
    let watched_id = profile_id.clone();
    std::thread::spawn(move || {
        // Blocks until the RDP client process (mstsc/xfreerdp/sdl-freerdp) exits,
        // i.e. the user closed the window, disconnected, or we killed it via
        // disconnect_session.
        let _ = child.wait();
        if sessions::mark_ended(&sessions_map, &watched_id, pid) {
            let _ = app_handle.emit("session-ended", watched_id);
        }
    });

    Ok(())
}

#[tauri::command]
pub fn list_active_sessions(sessions: State<SessionsState>) -> Vec<String> {
    sessions::active_profile_ids(&sessions.0)
}

/// Force-closes every RDP client process currently running for this profile.
/// The background waiter thread in `launch_connection` observes the exit and
/// emits `session-ended` as usual, so the frontend doesn't need a separate
/// optimistic update.
#[tauri::command]
pub fn disconnect_session(sessions: State<SessionsState>, profile_id: String) -> AppResult<()> {
    for pid in sessions::pids_for(&sessions.0, &profile_id) {
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
    if let Some(&pid) = sessions::pids_for(&sessions.0, &profile_id).first() {
        focus_pid(pid);
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn focus_pid(pid: u32) {
    // Setting `frontmost` only needs Automation permission and deactivates
    // the process, but does NOT deminiaturize an already-minimized window —
    // that needs reading/writing the AXMinimized attribute, which requires
    // the separate (often not-yet-granted) Accessibility permission. Wrap
    // that part in `try` so a missing Accessibility grant doesn't also take
    // down the frontmost activation, which worked fine on its own before.
    let script = format!(
        r#"tell application "System Events"
    set targetProcess to first process whose unix id is {pid}
    try
        repeat with w in windows of targetProcess
            if value of attribute "AXMinimized" of w is true then
                set value of attribute "AXMinimized" of w to false
            end if
        end repeat
    end try
    set frontmost of targetProcess to true
end tell"#
    );
    let _ = std::process::Command::new("osascript")
        .args(["-e", &script])
        .status();
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
