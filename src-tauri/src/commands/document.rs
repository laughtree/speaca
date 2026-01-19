use tauri::{AppHandle, command, Runtime, Manager};

use crate::constants::{SAVE_FOLDER, FILE_EXT, META_EXT};
use crate::services::{versioning::{save_version, new_uid}, metadata::{workInfo, chapterReference, chapterHeader, chapter}};
use crate::utils::fs::{read_file};

#[command]
pub fn save_content<R: Runtime>(app: AppHandle<R>, mut uid: String, content: serde_json::Value) -> Result<String, String> {
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    if !save_dir.exists() {
        std::fs::create_dir_all(&save_dir).map_err(|e| format!("Failed to create save directory: {}", e))?;
    }

    if uid.is_empty() {
        uid = new_uid();
    }
    let filename : String = format!("{}{}", uid, FILE_EXT);

    let content_str = serde_json::to_string_pretty(&content).map_err(|_| "Stringfy Failed!".to_string())?;

    match save_version(&save_dir, &filename, &content_str) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn load_content<R: Runtime>(app: AppHandle<R>, uid: String) -> Result<chapter, String> {
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    let mut file_path = save_dir.clone();
    file_path.push(format!("{}{}", uid, FILE_EXT));
    match read_file(&file_path) {
        Ok(data) => serde_json::from_str::<chapter>(&data).map_err(|e| e.to_string()),
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
    let metadata = content;
    if metadata.is_empty() {
        return Err("Metadata content is empty".to_string());
    }

    match save_version(&save_dir, &metaname, &metadata) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub fn load_meta<R: Runtime>(app: AppHandle<R>, name: String) -> Result<workInfo, String> {
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push(SAVE_FOLDER);
    let mut meta_path = save_dir.clone();
    meta_path.push(format!("{}{}", name, META_EXT));

    match read_file(&meta_path) {
        Ok(data) => serde_json::from_str::<workInfo>(&data).map_err(|e| e.to_string()),
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

    let mut save_path = save_dir.clone();
    save_path.push(format!("{}{}", name, META_EXT));
    if save_path.exists() {
        return Err("Work existed!".to_string());
    }

    let work = workInfo::new(name.clone(), author);
    let first_chap_uid = work.chapters[0][0].uid.clone();

    let chap_content = chapter::new(&work, &work.chapters[0][0]);
    let chap_content_str = serde_json::to_string_pretty(&chap_content).map_err(|_| "Stringfy Failed!".to_string())?;

    match save_version(&save_dir, &format!("{}{}", first_chap_uid, FILE_EXT), &chap_content_str) {
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