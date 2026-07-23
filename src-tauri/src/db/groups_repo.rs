use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::Group;

fn row_to_group(row: &rusqlite::Row) -> rusqlite::Result<Group> {
    Ok(Group {
        id: row.get("id")?,
        name: row.get("name")?,
        parent_id: row.get("parent_id")?,
        sort_order: row.get("sort_order")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

pub fn list_all(conn: &Connection) -> AppResult<Vec<Group>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, parent_id, sort_order, created_at, updated_at
         FROM groups ORDER BY parent_id IS NOT NULL, sort_order, name",
    )?;
    let rows = stmt.query_map([], row_to_group)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn create(conn: &Connection, name: &str, parent_id: Option<&str>) -> AppResult<Group> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let next_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order) + 1, 0) FROM groups WHERE parent_id IS ?1",
        params![parent_id],
        |row| row.get(0),
    )?;

    conn.execute(
        "INSERT INTO groups (id, name, parent_id, sort_order, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
        params![id, name, parent_id, next_order, now],
    )?;

    Ok(Group {
        id,
        name: name.to_string(),
        parent_id: parent_id.map(|s| s.to_string()),
        sort_order: next_order,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn rename(conn: &Connection, id: &str, name: &str) -> AppResult<()> {
    let now = Utc::now().to_rfc3339();
    let affected = conn.execute(
        "UPDATE groups SET name = ?1, updated_at = ?2 WHERE id = ?3",
        params![name, now, id],
    )?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("group {id}")));
    }
    Ok(())
}

pub fn move_group(
    conn: &Connection,
    id: &str,
    new_parent_id: Option<&str>,
    new_order: i64,
) -> AppResult<()> {
    if let Some(parent) = new_parent_id {
        if parent == id {
            return Err(AppError::InvalidInput(
                "a group cannot be its own parent".into(),
            ));
        }
    }
    let now = Utc::now().to_rfc3339();
    let affected = conn.execute(
        "UPDATE groups SET parent_id = ?1, sort_order = ?2, updated_at = ?3 WHERE id = ?4",
        params![new_parent_id, new_order, now, id],
    )?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("group {id}")));
    }
    Ok(())
}

pub fn delete(conn: &Connection, id: &str) -> AppResult<()> {
    // ON DELETE CASCADE handles subgroups; profiles in this subtree get group_id = NULL
    // via ON DELETE SET NULL (they become "ungrouped" rather than silently deleted).
    let affected = conn.execute("DELETE FROM groups WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("group {id}")));
    }
    Ok(())
}
