use std::env;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use crate::alias;

/// Get the user's shell configuration file path.
fn get_user_shell_config_file() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or_else(|| "Failed to get home directory".to_string())?;
    let shell = env::var("SHELL").unwrap_or_default();
    
    let config_file_name = if shell.ends_with("bash") {
        ".bashrc"
    } else if shell.ends_with("zsh") {
        ".zshrc"
    } else {
        // Fallback for unknown shells. We can default to .bashrc or ask the user.
        // For now, let's return an error if we can't determine the shell.
        return Err("Unsupported shell detected. Only bash and zsh are currently supported for auto-configuration.".to_string());
    };

    Ok(home_dir.join(config_file_name))
}

/// Copy text to clipboard (macOS only).
fn copy_to_clipboard(text: &str) -> Result<(), String> {
    Command::new("pbcopy")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(text.as_bytes())?;
                stdin.flush()?;
            }
            Ok(())
        })
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;

    Ok(())
}

/// Open a new terminal window (macOS only).
fn open_terminal_window() -> Result<(), String> {
    // Use osascript to open a new Terminal window instead of activating existing one
    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Terminal\" to do script \"\"")
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn ensure_sourcing_is_setup() -> Result<(), String> {
    let user_config_file = get_user_shell_config_file()?;
    let alias_file = alias::get_alias_file_path()?;
    
    let source_command = format!("\nsource \"{}\"", alias_file.to_str().unwrap_or(""));
    if source_command.contains("= \"\"") {
        // Basic check if path conversion failed
        return Err("Could not construct source command.".to_string());
    }

    let content = read_to_string(&user_config_file).unwrap_or_default();

    if !content.contains(&source_command) {
        // The sourcing line is not present. Append it.
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true) // Create if it doesn't exist
            .open(&user_config_file)
            .map_err(|e| format!("Failed to open user config file: {}", e))?;

        file.write_all(source_command.as_bytes())
            .map_err(|e| format!("Failed to write to user config file: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub fn open_terminal(alias_name: String) -> Result<(), String> {
    // Copy alias name to clipboard (just the name, not the full alias definition)
    let aliases = alias::read_aliases()?;
    if let Some(alias) = aliases.iter().find(|a| a.name == alias_name) {
        copy_to_clipboard(&alias.name)?;
    }

    // Open terminal
    open_terminal_window()?;

    Ok(())
}

#[tauri::command]
pub fn copy_alias_name(alias_name: String) -> Result<(), String> {
    let aliases = alias::read_aliases()?;
    if let Some(alias) = aliases.iter().find(|a| a.name == alias_name) {
        copy_to_clipboard(&alias.name)?;
    }
    Ok(())
}

#[tauri::command]
pub fn restart_app(app: tauri::AppHandle) {
    app.restart();
}

