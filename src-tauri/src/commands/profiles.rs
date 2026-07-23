use tauri::State;

use crate::credentials;
use crate::db::{profiles_repo, DbState};
use crate::error::AppResult;
use crate::models::{ConnectionProfile, ConnectionProfileInput};

#[tauri::command]
pub fn list_profiles(
    db: State<DbState>,
    group_id: Option<String>,
) -> AppResult<Vec<ConnectionProfile>> {
    let conn = db.0.lock().unwrap();
    profiles_repo::list(&conn, group_id.as_deref())
}

#[tauri::command]
pub fn get_profile(db: State<DbState>, id: String) -> AppResult<ConnectionProfile> {
    let conn = db.0.lock().unwrap();
    profiles_repo::get(&conn, &id)
}

#[tauri::command]
pub fn create_profile(
    db: State<DbState>,
    input: ConnectionProfileInput,
) -> AppResult<ConnectionProfile> {
    let conn = db.0.lock().unwrap();
    profiles_repo::create(&conn, &input)
}

#[tauri::command]
pub fn update_profile(
    db: State<DbState>,
    id: String,
    input: ConnectionProfileInput,
) -> AppResult<ConnectionProfile> {
    let conn = db.0.lock().unwrap();
    profiles_repo::update(&conn, &id, &input)
}

#[tauri::command]
pub fn delete_profile(db: State<DbState>, id: String) -> AppResult<()> {
    let conn = db.0.lock().unwrap();
    profiles_repo::delete(&conn, &id)?;
    drop(conn);
    // Best-effort: profile row is already gone even if keyring cleanup fails.
    let _ = credentials::delete_all_for_profile(&id);
    Ok(())
}

#[tauri::command]
pub fn duplicate_profile(db: State<DbState>, id: String) -> AppResult<ConnectionProfile> {
    let conn = db.0.lock().unwrap();
    profiles_repo::duplicate(&conn, &id)
}

#[tauri::command]
pub fn move_profile(
    db: State<DbState>,
    id: String,
    group_id: Option<String>,
    order: i64,
) -> AppResult<()> {
    let conn = db.0.lock().unwrap();
    profiles_repo::move_profile(&conn, &id, group_id.as_deref(), order)
}
