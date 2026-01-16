use crate::commands::fs::{read_file, write_file, remove_file};
use diffy::{create_patch, Patch};
use sha2::{Digest, Sha256};

const FILE_EXT: &str = ".speaca";

pub fn diff_strings(a: &str, b: &str) -> std::io::Result<Patch> {
    Ok(create_patch(a, b))
}

pub fn diff_files(path_a: &str, path_b: &str) -> std::io::Result<Patch> {
    let content_a = read_file(path_a)?;
    let content_b = read_file(path_b)?;
    diff_strings(&content_a, &content_b)
}

pub fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn save_diff_to_file(patch: &Patch, path: &str) -> std::io::Result<()> {
    let patch_string = patch.to_string();
    write_file(path, &patch_string)
}

#[tauri::command]
pub fn save_version(uid: &str, content: &str) -> std::io::Result<String> {
    let time = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let hash = hash_content(content);

    let filename = format!("{}{}", uid, FILE_EXT);
    let old = match read_file(&filename) {
        Ok(data) => data,
        Err(_) => String::new(),
    };

    let patch = diff_strings(&old, content)?;
    if patch.hunks().is_empty() {
        return Ok("No changes detected".into());
    }
    let patch_path = format!("{}_{}_{}.patch", uid, time, &hash[..8]);
    save_diff_to_file(&patch, &patch_path)?;

    if let Err(e) = write_file(&filename, content) {
        let _ = remove_file(&patch_path).ok();
        return Err(format!("Failed to save file: {}", e).into());
    }

    Ok("Version saved successfully".into())
}





