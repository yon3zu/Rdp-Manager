use tauri::State;

use crate::db::{groups_repo, DbState};
use crate::error::AppResult;
use crate::models::Group;

#[tauri::command]
pub fn list_groups(db: State<DbState>) -> AppResult<Vec<Group>> {
    let conn = db.0.lock().unwrap();
    groups_repo::list_all(&conn)
}

#[tauri::command]
pub fn create_group(
    db: State<DbState>,
    name: String,
    parent_id: Option<String>,
) -> AppResult<Group> {
    let conn = db.0.lock().unwrap();
    groups_repo::create(&conn, &name, parent_id.as_deref())
}

#[tauri::command]
pub fn rename_group(db: State<DbState>, id: String, name: String) -> AppResult<()> {
    let conn = db.0.lock().unwrap();
    groups_repo::rename(&conn, &id, &name)
}

#[tauri::command]
pub fn move_group(
    db: State<DbState>,
    id: String,
    new_parent_id: Option<String>,
    new_order: i64,
) -> AppResult<()> {
    let conn = db.0.lock().unwrap();
    groups_repo::move_group(&conn, &id, new_parent_id.as_deref(), new_order)
}

#[tauri::command]
pub fn delete_group(db: State<DbState>, id: String) -> AppResult<()> {
    let conn = db.0.lock().unwrap();
    groups_repo::delete(&conn, &id)
}
