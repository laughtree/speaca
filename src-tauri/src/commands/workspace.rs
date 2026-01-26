use tauri::{AppHandle, command, Runtime, Manager};
use std::fs;
use serde::Serialize;

use crate::constants::{SAVE_FOLDER, FILE_EXT};

#[command]
pub fn get_works<R: Runtime>(app: AppHandle<R>) -> Result<Vec<String>, String> {
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    if !save_dir.exists() {
        std::fs::create_dir_all(&save_dir).map_err(|e| format!("Failed to create save directory: {}", e))?;
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(&save_dir).map_err(|e| e.to_string())?;
    let works = entries
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().map_or(false, |ext| ext == "speaca"))
        .filter_map(|p| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        })
        .collect();

    Ok(works)
}