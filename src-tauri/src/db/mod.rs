pub mod groups_repo;
pub mod profiles_repo;

use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

use crate::error::AppResult;

pub struct DbState(pub Mutex<Connection>);

const INIT_SQL: &str = include_str!("../../migrations/0001_init.sql");

pub fn open(db_path: &Path) -> AppResult<Connection> {
    let conn = Connection::open(db_path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;

    let user_version: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    if user_version == 0 {
        conn.execute_batch(INIT_SQL)?;
        conn.pragma_update(None, "user_version", 1)?;
    }

    Ok(conn)
}
