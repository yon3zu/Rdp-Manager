use chrono::Utc;
use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::profile::{AudioMode, CertTrustBehavior, GatewayUsage, ScreenMode};
use crate::models::{AdvancedSettings, ConnectionProfile, ConnectionProfileInput};

const SELECT_COLUMNS: &str = "id, group_id, name, protocol, host, port, username, domain,
    has_saved_password, has_saved_gateway_password, sort_order,
    screen_mode, desktop_width, desktop_height, dynamic_resolution, color_depth,
    multi_monitor, selected_monitors, admin_session, redirect_drives, redirect_printers,
    redirect_clipboard, audio_mode, mic_redirection, gateway_hostname, gateway_port,
    gateway_username, gateway_domain, gateway_usage, cert_trust_behavior,
    connection_timeout_ms, notes, created_at, updated_at";

fn row_to_profile(row: &rusqlite::Row) -> rusqlite::Result<ConnectionProfile> {
    Ok(ConnectionProfile {
        id: row.get("id")?,
        group_id: row.get("group_id")?,
        name: row.get("name")?,
        protocol: row.get("protocol")?,
        host: row.get("host")?,
        port: row.get("port")?,
        username: row.get("username")?,
        domain: row.get("domain")?,
        has_saved_password: row.get::<_, i64>("has_saved_password")? != 0,
        has_saved_gateway_password: row.get::<_, i64>("has_saved_gateway_password")? != 0,
        sort_order: row.get("sort_order")?,
        notes: row.get("notes")?,
        advanced: AdvancedSettings {
            screen_mode: ScreenMode::from_str(&row.get::<_, String>("screen_mode")?),
            desktop_width: row.get("desktop_width")?,
            desktop_height: row.get("desktop_height")?,
            dynamic_resolution: row.get::<_, i64>("dynamic_resolution")? != 0,
            color_depth: row.get("color_depth")?,
            multi_monitor: row.get::<_, i64>("multi_monitor")? != 0,
            selected_monitors: row.get("selected_monitors")?,
            admin_session: row.get::<_, i64>("admin_session")? != 0,
            redirect_drives: row.get::<_, i64>("redirect_drives")? != 0,
            redirect_printers: row.get::<_, i64>("redirect_printers")? != 0,
            redirect_clipboard: row.get::<_, i64>("redirect_clipboard")? != 0,
            audio_mode: AudioMode::from_str(&row.get::<_, String>("audio_mode")?),
            mic_redirection: row.get::<_, i64>("mic_redirection")? != 0,
            gateway_hostname: row.get("gateway_hostname")?,
            gateway_port: row.get("gateway_port")?,
            gateway_username: row.get("gateway_username")?,
            gateway_domain: row.get("gateway_domain")?,
            gateway_usage: GatewayUsage::from_str(&row.get::<_, String>("gateway_usage")?),
            cert_trust_behavior: CertTrustBehavior::from_str(
                &row.get::<_, String>("cert_trust_behavior")?,
            ),
            connection_timeout_ms: row.get("connection_timeout_ms")?,
        },
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

pub fn list(conn: &Connection, group_id: Option<&str>) -> AppResult<Vec<ConnectionProfile>> {
    let sql = format!(
        "SELECT {SELECT_COLUMNS} FROM connection_profiles
         WHERE (?1 IS NULL OR group_id = ?1)
         ORDER BY sort_order, name"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![group_id], row_to_profile)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub fn get(conn: &Connection, id: &str) -> AppResult<ConnectionProfile> {
    let sql = format!("SELECT {SELECT_COLUMNS} FROM connection_profiles WHERE id = ?1");
    conn.query_row(&sql, params![id], row_to_profile)
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(format!("profile {id}")),
            other => AppError::Db(other),
        })
}

pub fn create(conn: &Connection, input: &ConnectionProfileInput) -> AppResult<ConnectionProfile> {
    if input.host.trim().is_empty() {
        return Err(AppError::InvalidInput("host is required".into()));
    }
    if input.name.trim().is_empty() {
        return Err(AppError::InvalidInput("name is required".into()));
    }

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let a = &input.advanced;

    let next_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order) + 1, 0) FROM connection_profiles WHERE group_id IS ?1",
        params![input.group_id],
        |row| row.get(0),
    )?;

    conn.execute(
        "INSERT INTO connection_profiles (
            id, group_id, name, protocol, host, port, username, domain,
            has_saved_password, has_saved_gateway_password, sort_order,
            screen_mode, desktop_width, desktop_height, dynamic_resolution, color_depth,
            multi_monitor, selected_monitors, admin_session, redirect_drives, redirect_printers,
            redirect_clipboard, audio_mode, mic_redirection, gateway_hostname, gateway_port,
            gateway_username, gateway_domain, gateway_usage, cert_trust_behavior,
            connection_timeout_ms, notes, created_at, updated_at
        ) VALUES (
            ?1, ?2, ?3, 'rdp', ?4, ?5, ?6, ?7,
            0, 0, ?8,
            ?9, ?10, ?11, ?12, ?13,
            ?14, ?15, ?16, ?17, ?18,
            ?19, ?20, ?21, ?22, ?23,
            ?24, ?25, ?26, ?27,
            ?28, ?29, ?30, ?30
        )",
        params![
            id,
            input.group_id,
            input.name,
            input.host,
            input.port,
            input.username,
            input.domain,
            next_order,
            a.screen_mode.as_str(),
            a.desktop_width,
            a.desktop_height,
            a.dynamic_resolution as i64,
            a.color_depth,
            a.multi_monitor as i64,
            a.selected_monitors,
            a.admin_session as i64,
            a.redirect_drives as i64,
            a.redirect_printers as i64,
            a.redirect_clipboard as i64,
            a.audio_mode.as_str(),
            a.mic_redirection as i64,
            a.gateway_hostname,
            a.gateway_port,
            a.gateway_username,
            a.gateway_domain,
            a.gateway_usage.as_str(),
            a.cert_trust_behavior.as_str(),
            a.connection_timeout_ms,
            input.notes,
            now,
        ],
    )?;

    get(conn, &id)
}

pub fn update(
    conn: &Connection,
    id: &str,
    input: &ConnectionProfileInput,
) -> AppResult<ConnectionProfile> {
    if input.host.trim().is_empty() {
        return Err(AppError::InvalidInput("host is required".into()));
    }
    if input.name.trim().is_empty() {
        return Err(AppError::InvalidInput("name is required".into()));
    }

    let now = Utc::now().to_rfc3339();
    let a = &input.advanced;

    let affected = conn.execute(
        "UPDATE connection_profiles SET
            group_id = ?1, name = ?2, host = ?3, port = ?4, username = ?5, domain = ?6,
            screen_mode = ?7, desktop_width = ?8, desktop_height = ?9, dynamic_resolution = ?10,
            color_depth = ?11, multi_monitor = ?12, selected_monitors = ?13, admin_session = ?14,
            redirect_drives = ?15, redirect_printers = ?16, redirect_clipboard = ?17,
            audio_mode = ?18, mic_redirection = ?19, gateway_hostname = ?20, gateway_port = ?21,
            gateway_username = ?22, gateway_domain = ?23, gateway_usage = ?24,
            cert_trust_behavior = ?25, connection_timeout_ms = ?26, notes = ?27, updated_at = ?28
         WHERE id = ?29",
        params![
            input.group_id,
            input.name,
            input.host,
            input.port,
            input.username,
            input.domain,
            a.screen_mode.as_str(),
            a.desktop_width,
            a.desktop_height,
            a.dynamic_resolution as i64,
            a.color_depth,
            a.multi_monitor as i64,
            a.selected_monitors,
            a.admin_session as i64,
            a.redirect_drives as i64,
            a.redirect_printers as i64,
            a.redirect_clipboard as i64,
            a.audio_mode.as_str(),
            a.mic_redirection as i64,
            a.gateway_hostname,
            a.gateway_port,
            a.gateway_username,
            a.gateway_domain,
            a.gateway_usage.as_str(),
            a.cert_trust_behavior.as_str(),
            a.connection_timeout_ms,
            input.notes,
            now,
            id,
        ],
    )?;

    if affected == 0 {
        return Err(AppError::NotFound(format!("profile {id}")));
    }

    get(conn, id)
}

pub fn delete(conn: &Connection, id: &str) -> AppResult<()> {
    let affected = conn.execute("DELETE FROM connection_profiles WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("profile {id}")));
    }
    Ok(())
}

pub fn duplicate(conn: &Connection, id: &str) -> AppResult<ConnectionProfile> {
    let original = get(conn, id)?;
    let input = ConnectionProfileInput {
        group_id: original.group_id,
        name: format!("{} (copy)", original.name),
        host: original.host,
        port: original.port,
        username: original.username,
        domain: original.domain,
        notes: original.notes,
        advanced: original.advanced,
    };
    // Note: saved passwords are intentionally NOT copied to the keyring here;
    // caller (IPC layer) decides whether to prompt the user to re-enter secrets.
    create(conn, &input)
}

pub fn set_has_saved_password(conn: &Connection, id: &str, value: bool) -> AppResult<()> {
    conn.execute(
        "UPDATE connection_profiles SET has_saved_password = ?1 WHERE id = ?2",
        params![value as i64, id],
    )?;
    Ok(())
}

pub fn set_has_saved_gateway_password(conn: &Connection, id: &str, value: bool) -> AppResult<()> {
    conn.execute(
        "UPDATE connection_profiles SET has_saved_gateway_password = ?1 WHERE id = ?2",
        params![value as i64, id],
    )?;
    Ok(())
}

pub fn move_profile(
    conn: &Connection,
    id: &str,
    group_id: Option<&str>,
    order: i64,
) -> AppResult<()> {
    let now = Utc::now().to_rfc3339();
    let affected = conn.execute(
        "UPDATE connection_profiles SET group_id = ?1, sort_order = ?2, updated_at = ?3 WHERE id = ?4",
        params![group_id, order, now, id],
    )?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("profile {id}")));
    }
    Ok(())
}
