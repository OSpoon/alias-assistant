use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alias {
    pub name: String,
    pub command: String,
}

/// Get the path to the alias file, creating the directory if it doesn't exist.
pub fn get_alias_file_path() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or_else(|| "Failed to get home directory".to_string())?;
    let app_dir = home_dir.join(".alias-assistant");

    // Ensure the ~/.alias-assistant directory exists.
    create_dir_all(&app_dir).map_err(|e| format!("Failed to create app directory: {}", e))?;

    Ok(app_dir.join("aliases.sh"))
}

/// Read aliases from the alias file.
pub fn read_aliases() -> Result<Vec<Alias>, String> {
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

/// Write aliases to the alias file.
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

/// Parse aliases from content string.
pub fn parse_aliases_from_content(content: &str) -> Vec<Alias> {
    content
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
        .collect()
}

#[tauri::command]
pub fn get_aliases() -> Result<Vec<Alias>, String> {
    read_aliases()
}

#[tauri::command]
pub fn add_alias(name: String, command: String) -> Result<(), String> {
    let mut aliases = read_aliases()?;
    if aliases.iter().any(|a| a.name == name) {
        return Err("Alias with that name already exists".to_string());
    }
    aliases.push(Alias { name, command });
    write_aliases(aliases)
}

#[tauri::command]
pub fn delete_alias(name: String) -> Result<(), String> {
    let mut aliases = read_aliases()?;
    aliases.retain(|a| a.name != name);
    write_aliases(aliases)
}

#[tauri::command]
pub fn export_aliases() -> Result<String, String> {
    let path = get_alias_file_path()?;
    let content = read_to_string(path).unwrap_or_default();
    Ok(content)
}

#[tauri::command]
pub fn import_aliases_from_content(content: String) -> Result<(), String> {
    let mut existing_aliases = read_aliases()?;
    
    // Parse aliases from the imported content
    let imported_aliases = parse_aliases_from_content(&content);
    
    // Merge imported aliases with existing ones (imported aliases take precedence)
    for imported in imported_aliases {
        // Remove existing alias with same name if exists
        existing_aliases.retain(|a| a.name != imported.name);
        // Add imported alias
        existing_aliases.push(imported);
    }
    
    write_aliases(existing_aliases)
}

