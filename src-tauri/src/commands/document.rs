use tauri::{AppHandle, command, Runtime, Manager};

use crate::constants::{SAVE_FOLDER, FILE_EXT, META_EXT};
use crate::services::{versioning::{save_version, new_uid}, metadata::{workInfo, chapterReference, chapterHeader}};
use crate::utils::fs::{read_file};

#[command]
pub fn save_content<R: Runtime>(app: AppHandle<R>, mut uid: String, content: String) -> Result<String, String> {
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    if !save_dir.exists() {
        std::fs::create_dir_all(&save_dir).map_err(|e| format!("Failed to create save directory: {}", e))?;
    }

    if uid.is_empty() {
        uid = new_uid();
    }
    let filename : String = format!("{}{}", uid, FILE_EXT);

    match save_version(&save_dir, &filename, &content) {
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

#[command]
pub fn save_meta<R: Runtime>(app: AppHandle<R>, name: String, content: String) -> Result<String, String> {
    if name.is_empty() {
        return Err("Work name cannot be empty".to_string());
    }

    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    if !save_dir.exists() {
        std::fs::create_dir_all(&save_dir).map_err(|e| format!("Failed to create save directory: {}", e))?;
    }

    let metaname : String = format!("{}{}", name, META_EXT);
    let mut metadata = content;
    if metadata.is_empty() {
        return Err("Metadata content is empty".to_string());
    }

    match save_version(&save_dir, &metaname, &metadata) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn load_meta<R: Runtime>(app: AppHandle<R>, name: String) -> Result<String, String> {
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    let mut meta_path = save_dir.clone();
    meta_path.push(format!("{}{}", name, META_EXT));
    match read_file(&meta_path) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn create_new_work<R: Runtime>(app: AppHandle<R>, name: String, author: String) -> Result<workInfo, String> {
    if name.is_empty() {
        return Err("Work name cannot be empty".to_string());
    }
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    if !save_dir.exists() {
        std::fs::create_dir_all(&save_dir).map_err(|e| format!("Failed to create save directory: {}", e))?;
    }

    let work = workInfo::new(name.clone(), author);
    let first_chap_uid = work.chapters[0][0].uid.clone();

    match save_version(&save_dir, &format!("{}{}", first_chap_uid, FILE_EXT), "{}") {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()),
    }

    let metaname : String = format!("{}{}", name, META_EXT);
    let metadata = serde_json::to_string(&work)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))?;

    match save_version(&save_dir, &metaname, &metadata) {
        Ok(_) => Ok(work),
        Err(e) => Err(e.to_string()),
    }
}