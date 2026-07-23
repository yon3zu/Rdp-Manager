mod commands;
mod credentials;
mod db;
mod error;
mod launcher;
mod models;
mod rdpfile;
mod sessions;

use db::DbState;
use sessions::SessionsState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("rdpmanager.sqlite3");
            let conn = db::open(&db_path).expect("failed to open database");
            app.manage(DbState(Mutex::new(conn)));
            app.manage(SessionsState::default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::groups::list_groups,
            commands::groups::create_group,
            commands::groups::rename_group,
            commands::groups::move_group,
            commands::groups::delete_group,
            commands::profiles::list_profiles,
            commands::profiles::get_profile,
            commands::profiles::create_profile,
            commands::profiles::update_profile,
            commands::profiles::delete_profile,
            commands::profiles::duplicate_profile,
            commands::profiles::move_profile,
            commands::credentials::set_profile_password,
            commands::credentials::clear_profile_password,
            commands::credentials::has_profile_password,
            commands::credentials::set_gateway_password,
            commands::credentials::clear_gateway_password,
            commands::launch::launch_connection,
            commands::launch::export_rdp_file,
            commands::launch::list_active_sessions,
            commands::system::check_launcher_status,
            commands::system::get_platform,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
