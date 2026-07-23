use tauri::State;

use crate::credentials;
use crate::db::{profiles_repo, DbState};
use crate::error::AppResult;

#[tauri::command]
pub fn set_profile_password(db: State<DbState>, profile_id: String, password: String) -> AppResult<()> {
    credentials::set_password(&profile_id, &password)?;
    let conn = db.0.lock().unwrap();
    profiles_repo::set_has_saved_password(&conn, &profile_id, true)
}

#[tauri::command]
pub fn clear_profile_password(db: State<DbState>, profile_id: String) -> AppResult<()> {
    credentials::delete_password(&profile_id)?;
    let conn = db.0.lock().unwrap();
    profiles_repo::set_has_saved_password(&conn, &profile_id, false)
}

#[tauri::command]
pub fn has_profile_password(profile_id: String) -> AppResult<bool> {
    Ok(credentials::get_password(&profile_id)?.is_some())
}

#[tauri::command]
pub fn set_gateway_password(db: State<DbState>, profile_id: String, password: String) -> AppResult<()> {
    credentials::set_gateway_password(&profile_id, &password)?;
    let conn = db.0.lock().unwrap();
    profiles_repo::set_has_saved_gateway_password(&conn, &profile_id, true)
}

#[tauri::command]
pub fn clear_gateway_password(db: State<DbState>, profile_id: String) -> AppResult<()> {
    credentials::delete_gateway_password(&profile_id)?;
    let conn = db.0.lock().unwrap();
    profiles_repo::set_has_saved_gateway_password(&conn, &profile_id, false)
}
