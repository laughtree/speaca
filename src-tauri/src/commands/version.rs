use crate::commands::fs::{read_file, write_file, remove_file};
use diffy::{create_patch, Patch};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::path::Path;
use tauri::Manager;

const FILE_EXT: &str = ".speaca";

pub fn diff_strings<'a>(a: &'a str, b: &'a str) -> std::io::Result<Patch<'a, str>> {
    Ok(create_patch(a, b))
}

pub fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn save_diff_to_file<P: AsRef<Path>>(patch: &Patch<'_, str>, path: P) -> std::io::Result<()> {
    let patch_string = patch.to_string();
    write_file(path, &patch_string)
}

#[tauri::command]
pub fn save_content(app: tauri::AppHandle, uid: String, content: String) -> Result<String, String> {
    let mut save_dir = app.path().document_dir().map_err(|_| "Could not find documents directory".to_string())?;
    save_dir.push("speaca_saves");
    if !save_dir.exists() {
        std::fs::create_dir_all(&save_dir).map_err(|e| format!("Failed to create save directory: {}", e))?;
    }
    match save_version(&save_dir, &uid, &content) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(e.to_string()),
    }
}

pub fn save_version(save_dir: &PathBuf, uid: &str, content: &str) -> std::io::Result<String> {
    let time = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let hash = hash_content(content);

    let mut filename = save_dir.clone();
    filename.push(format!("{}{}", uid, FILE_EXT));
    let old = match read_file(&filename) {
        Ok(data) => data,
        Err(_) => String::new(),
    };

    let patch = diff_strings(&old, content)?;
    if patch.hunks().is_empty() {
        return Ok("No changes detected".into());
    }
    let mut patch_path = save_dir.clone();
    patch_path.push(format!("{}_{}_{}.patch", uid, time, hash));
    save_diff_to_file(&patch, &patch_path)?;

    if let Err(e) = write_file(&filename, content) {
        let _ = remove_file(&patch_path).ok();
        return Err(e);
    }

    Ok("Version saved successfully".into())
}





