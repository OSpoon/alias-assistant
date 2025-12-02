use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GistConfig {
    pub token: Option<String>,
    pub gist_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GistFile {
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GistFiles {
    #[serde(rename = "aliases.sh")]
    aliases: GistFile,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateGistRequest {
    description: String,
    public: bool,
    files: GistFiles,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateGistRequest {
    description: String,
    files: GistFiles,
}

#[derive(Debug, Serialize, Deserialize)]
struct GistResponse {
    id: String,
    files: GistFiles,
}

/// Get the path to the config file, creating the directory if it doesn't exist.
fn get_config_file_path() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or_else(|| "Failed to get home directory".to_string())?;
    let app_dir = home_dir.join(".alias-assistant");

    // Ensure the ~/.alias-assistant directory exists.
    create_dir_all(&app_dir).map_err(|e| format!("Failed to create app directory: {}", e))?;

    Ok(app_dir.join("gist_config.json"))
}

/// Read Gist configuration from file.
pub fn read_gist_config() -> Result<GistConfig, String> {
    let path = get_config_file_path()?;
    let content = read_to_string(path).unwrap_or_default();
    
    if content.is_empty() {
        return Ok(GistConfig {
            token: None,
            gist_id: None,
        });
    }

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
}

/// Write Gist configuration to file.
fn write_gist_config(config: &GistConfig) -> Result<(), String> {
    let path = get_config_file_path()?;
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}

/// Set GitHub token.
#[tauri::command]
pub fn set_gist_token(token: String) -> Result<(), String> {
    let mut config = read_gist_config()?;
    config.token = Some(token);
    write_gist_config(&config)
}

/// Get GitHub token.
#[tauri::command]
pub fn get_gist_token() -> Result<Option<String>, String> {
    let config = read_gist_config()?;
    Ok(config.token)
}

/// Set Gist ID.
#[tauri::command]
pub fn set_gist_id(gist_id: String) -> Result<(), String> {
    let mut config = read_gist_config()?;
    config.gist_id = Some(gist_id);
    write_gist_config(&config)
}

/// Get Gist ID.
#[tauri::command]
pub fn get_gist_id() -> Result<Option<String>, String> {
    let config = read_gist_config()?;
    Ok(config.gist_id)
}

/// Create a new Gist with aliases content.
async fn create_gist(token: &str, content: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = "https://api.github.com/gists";

    let request = CreateGistRequest {
        description: "Alias Assistant - Shell Aliases".to_string(),
        public: false,
        files: GistFiles {
            aliases: GistFile {
                content: content.to_string(),
            },
        },
    };

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "Alias-Assistant")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("GitHub API error: {}", error_text));
    }

    let gist: GistResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(gist.id)
}

/// Update an existing Gist with new content.
async fn update_gist(token: &str, gist_id: &str, content: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/gists/{}", gist_id);

    let request = UpdateGistRequest {
        description: "Alias Assistant - Shell Aliases".to_string(),
        files: GistFiles {
            aliases: GistFile {
                content: content.to_string(),
            },
        },
    };

    let response = client
        .patch(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "Alias-Assistant")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("GitHub API error: {}", error_text));
    }

    Ok(())
}

/// Get content from an existing Gist.
async fn get_gist_content(token: &str, gist_id: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/gists/{}", gist_id);

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "Alias-Assistant")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("GitHub API error: {}", error_text));
    }

    let gist: GistResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(gist.files.aliases.content)
}

/// Sync aliases to Gist (push).
#[tauri::command]
pub async fn sync_aliases_to_gist(content: String) -> Result<String, String> {
    let config = read_gist_config()?;
    let token = config.token.ok_or_else(|| "GitHub token not set".to_string())?;

    if let Some(gist_id) = config.gist_id {
        // Update existing Gist
        update_gist(&token, &gist_id, &content).await?;
        Ok(format!("Successfully synced to Gist: {}", gist_id))
    } else {
        // Create new Gist
        let gist_id = create_gist(&token, &content).await?;
        set_gist_id(gist_id.clone())?;
        Ok(format!("Successfully created and synced to Gist: {}", gist_id))
    }
}

/// Sync aliases from Gist (pull).
#[tauri::command]
pub async fn sync_aliases_from_gist() -> Result<String, String> {
    let config = read_gist_config()?;
    let token = config.token.ok_or_else(|| "GitHub token not set".to_string())?;
    let gist_id = config.gist_id.ok_or_else(|| "Gist ID not set".to_string())?;

    let content = get_gist_content(&token, &gist_id).await?;
    Ok(content)
}

