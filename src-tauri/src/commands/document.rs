use tauri::{AppHandle, command, Runtime, Manager};

use crate::constants::{SAVE_FOLDER, FILE_EXT};
use crate::services::versioning::{save_version};
use crate::utils::fs::{read_file};

#[command]
pub fn save_content<R: Runtime>(app: AppHandle<R>, uid: String, content: String) -> Result<String, String> {
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    if !save_dir.exists() {
        std::fs::create_dir_all(&save_dir).map_err(|e| format!("Failed to create save directory: {}", e))?;
    }
    match save_version(&save_dir, &uid, &content) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn load_content<R: Runtime>(app: AppHandle<R>, uid: String) -> Result<String, String> {
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    let mut file_path = save_dir.clone();
    file_path.push(format!("{}{}", uid, FILE_EXT));
    match read_file(&file_path) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}