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

    sessions::mark_started(&sessions.0, &profile_id);
    let _ = app.emit("session-started", profile_id.clone());

    let sessions_map = sessions.0.clone();
    let app_handle = app.clone();
    let watched_id = profile_id.clone();
    std::thread::spawn(move || {
        // Blocks until the RDP client process (mstsc/xfreerdp/sdl-freerdp) exits,
        // i.e. the user closed the window or the session disconnected.
        let _ = child.wait();
        if sessions::mark_ended(&sessions_map, &watched_id) {
            let _ = app_handle.emit("session-ended", watched_id);
        }
    });

    Ok(())
}

#[tauri::command]
pub fn list_active_sessions(sessions: State<SessionsState>) -> Vec<String> {
    sessions::active_profile_ids(&sessions.0)
}

#[tauri::command]
pub fn export_rdp_file(db: State<DbState>, profile_id: String, dest_path: String) -> AppResult<()> {
    let conn = db.0.lock().unwrap();
    let profile = profiles_repo::get(&conn, &profile_id)?;
    drop(conn);

    let content = rdpfile::generate(&profile);
    fs::write(dest_path, content)?;
    Ok(())
}
