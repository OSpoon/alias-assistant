use std::env;
use std::fs::{create_dir_all, read_to_string, write, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Alias {
    name: String,
    command: String,
}

// This function now points to our dedicated alias file, creating the directory if it doesn't exist.
fn get_alias_file_path() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or_else(|| "Failed to get home directory".to_string())?;
    let app_dir = home_dir.join(".alias-assistant");

    // Ensure the ~/.alias-assistant directory exists.
    create_dir_all(&app_dir).map_err(|e| format!("Failed to create app directory: {}", e))?;

    Ok(app_dir.join("aliases.sh"))
}

// This function determines the user's main shell configuration file.
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


#[tauri::command]
fn get_aliases() -> Result<Vec<Alias>, String> {
    let path = get_alias_file_path()?;
    // If the file doesn't exist, it's not an error, just means no aliases.
    let content = read_to_string(path).unwrap_or_default();
    
    let aliases = content
        .lines()
        .filter_map(|line| {
            if line.trim().starts_with("alias ") {
                let parts: Vec<&str> = line.trim()["alias ".len()..].splitn(2, '=').collect();
                if parts.len() == 2 {
                    let name = parts[0].to_string();
                    let command = parts[1].trim_matches('"').to_string();
                    return Some(Alias { name, command });
                }
            }
            None
        })
        .collect();
    Ok(aliases)
}

// This function now simply overwrites the dedicated alias file.
fn write_aliases(aliases: Vec<Alias>) -> Result<(), String> {
    let path = get_alias_file_path()?;
    
    let mut lines = Vec::new();
    lines.push("# This file is managed by Alias-Assistant. Manual edits might be overwritten.".to_string());
    
    for alias in aliases {
        lines.push(format!("alias {}=\"{}\"", alias.name, alias.command));
    }
    
    let content = lines.join("\n") + "\n";
    write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn ensure_sourcing_is_setup() -> Result<(), String> {
    let user_config_file = get_user_shell_config_file()?;
    let alias_file = get_alias_file_path()?;
    
    let source_command = format!("\nsource \"{}\"", alias_file.to_str().unwrap_or(""));
    if source_command.contains("= \"\"") { // Basic check if path conversion failed
        return Err("Could not construct source command.".to_string());
    }

    let content = read_to_string(&user_config_file).unwrap_or_default();

    if !content.contains(&source_command) {
        // The sourcing line is not present. Append it.!
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
fn open_terminal(alias_name: String) -> Result<(), String> {
    // Copy alias name to clipboard (just the name, not the full alias definition)
    let aliases = get_aliases()?;
    if let Some(alias) = aliases.iter().find(|a| a.name == alias_name) {
        let alias_text = alias.name.clone();
        
        #[cfg(target_os = "macos")]
        {
            Command::new("pbcopy")
                .stdin(std::process::Stdio::piped())
                .spawn()
                .and_then(|mut child| {
                    use std::io::Write;
                    if let Some(mut stdin) = child.stdin.take() {
                        stdin.write_all(alias_text.as_bytes())?;
                        stdin.flush()?;
                    }
                    Ok(())
                })
                .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
        }

        #[cfg(target_os = "windows")]
        {
            // Use PowerShell Set-Clipboard for better reliability
            Command::new("powershell")
                .arg("-Command")
                .arg(format!("Set-Clipboard -Value '{}'", alias_text.replace("'", "''")))
                .spawn()
                .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
        }

        #[cfg(target_os = "linux")]
        {
            // Try xclip first, fallback to xsel
            let xclip_result = Command::new("xclip")
                .arg("-selection")
                .arg("clipboard")
                .stdin(std::process::Stdio::piped())
                .spawn();
            
            if let Ok(mut child) = xclip_result {
                use std::io::Write;
                if let Some(mut stdin) = child.stdin.take() {
                    stdin.write_all(alias_text.as_bytes())
                        .map_err(|e| format!("Failed to write to xclip: {}", e))?;
                    stdin.flush()
                        .map_err(|e| format!("Failed to flush xclip: {}", e))?;
                }
            } else {
                // Fallback to xsel
                Command::new("xsel")
                    .arg("--clipboard")
                    .arg("--input")
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        use std::io::Write;
                        if let Some(mut stdin) = child.stdin.take() {
                            stdin.write_all(alias_text.as_bytes())?;
                            stdin.flush()?;
                        }
                        Ok(())
                    })
                    .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
            }
        }
    }

    // Open terminal
    #[cfg(target_os = "macos")]
    {
        // Use osascript to open a new Terminal window instead of activating existing one
        Command::new("osascript")
            .arg("-e")
            .arg("tell application \"Terminal\" to do script \"\"")
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .arg("/c")
            .arg("start")
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        // x-terminal-emulator is a common default, but might not be universal.
        Command::new("x-terminal-emulator")
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}


#[tauri::command]
fn add_alias(name: String, command: String) -> Result<(), String> {
    let mut aliases = get_aliases()?;
    if aliases.iter().any(|a| a.name == name) {
        return Err("Alias with that name already exists".to_string());
    }
    aliases.push(Alias { name, command });
    write_aliases(aliases)
}

#[tauri::command]
fn delete_alias(name: String) -> Result<(), String> {
    let mut aliases = get_aliases()?;
    aliases.retain(|a| a.name != name);
    write_aliases(aliases)
}

#[tauri::command]
fn copy_alias_name(alias_name: String) -> Result<(), String> {
    let aliases = get_aliases()?;
    if let Some(alias) = aliases.iter().find(|a| a.name == alias_name) {
        let alias_text = alias.name.clone();
        
        #[cfg(target_os = "macos")]
        {
            Command::new("pbcopy")
                .stdin(std::process::Stdio::piped())
                .spawn()
                .and_then(|mut child| {
                    use std::io::Write;
                    if let Some(mut stdin) = child.stdin.take() {
                        stdin.write_all(alias_text.as_bytes())?;
                        stdin.flush()?;
                    }
                    Ok(())
                })
                .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("powershell")
                .arg("-Command")
                .arg(format!("Set-Clipboard -Value '{}'", alias_text.replace("'", "''")))
                .spawn()
                .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
        }

        #[cfg(target_os = "linux")]
        {
            let xclip_result = Command::new("xclip")
                .arg("-selection")
                .arg("clipboard")
                .stdin(std::process::Stdio::piped())
                .spawn();
            
            if let Ok(mut child) = xclip_result {
                use std::io::Write;
                if let Some(mut stdin) = child.stdin.take() {
                    stdin.write_all(alias_text.as_bytes())
                        .map_err(|e| format!("Failed to write to xclip: {}", e))?;
                    stdin.flush()
                        .map_err(|e| format!("Failed to flush xclip: {}", e))?;
                }
            } else {
                Command::new("xsel")
                    .arg("--clipboard")
                    .arg("--input")
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        use std::io::Write;
                        if let Some(mut stdin) = child.stdin.take() {
                            stdin.write_all(alias_text.as_bytes())?;
                            stdin.flush()?;
                        }
                        Ok(())
                    })
                    .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn export_aliases() -> Result<String, String> {
    let path = get_alias_file_path()?;
    let content = read_to_string(path).unwrap_or_default();
    Ok(content)
}

#[tauri::command]
fn import_aliases_from_content(content: String) -> Result<(), String> {
    let mut existing_aliases = get_aliases()?;
    
    // Parse aliases from the imported content
    let imported_aliases: Vec<Alias> = content
        .lines()
        .filter_map(|line| {
            if line.trim().starts_with("alias ") {
                let parts: Vec<&str> = line.trim()["alias ".len()..].splitn(2, '=').collect();
                if parts.len() == 2 {
                    let name = parts[0].to_string();
                    let command = parts[1].trim_matches('"').to_string();
                    return Some(Alias { name, command });
                }
            }
            None
        })
        .collect();
    
    // Merge imported aliases with existing ones (imported aliases take precedence)
    for imported in imported_aliases {
        // Remove existing alias with same name if exists
        existing_aliases.retain(|a| a.name != imported.name);
        // Add imported alias
        existing_aliases.push(imported);
    }
    
    write_aliases(existing_aliases)
}

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
            copy_alias_name
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
