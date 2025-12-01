// Business logic modules
mod alias;
mod system;

// Re-export commands for use in invoke_handler
use alias::{
    add_alias, delete_alias, export_aliases, get_aliases, import_aliases_from_content,
};
use system::{copy_alias_name, ensure_sourcing_is_setup, open_terminal, restart_app};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            get_aliases,
            add_alias,
            delete_alias,
            ensure_sourcing_is_setup,
            open_terminal,
            export_aliases,
            import_aliases_from_content,
            copy_alias_name,
            restart_app
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
