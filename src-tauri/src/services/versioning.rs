use std::{path::{Path, PathBuf}};
use diffy::{create_patch, Patch};
use sha2::{Digest, Sha256};
use uuid::{Uuid};

use crate::constants::{FILE_EXT, PATCH_EXT, META_EXT};
use crate::utils::fs::{read_file, remove_file, write_file};

pub fn diff_strings<'a>(a: &'a str, b: &'a str) -> std::io::Result<Patch<'a, str>> {
    Ok(create_patch(a, b))
}

pub fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn new_uid() -> String {
    Uuid::now_v7().to_string()
}

pub fn save_diff<P: AsRef<Path>>(patch: &Patch<'_, str>, path: P) -> std::io::Result<()> {
    let patch_string = patch.to_string();
    write_file(path, &patch_string)
}

pub fn save_version(save_dir: &PathBuf, filename: &str, content: &str) -> std::io::Result<String> {
    let time = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let hash = hash_content(content);

    let mut filepath = save_dir.clone();
    filepath.push(filename);
    let old = match read_file(&filepath) {
        Ok(data) => data,
        Err(_) => String::new(),
    };

    let patch = diff_strings(&old, content)?;
    if patch.hunks().is_empty() {
        return Ok("No changes detected".into());
    }
    let mut patch_path = save_dir.clone();
    patch_path.push(format!("{}_{}_{}{}", filename , time, hash, PATCH_EXT));
    save_diff(&patch, &patch_path)?;

    if let Err(e) = write_file(&filepath, content) {
        let _ = remove_file(&patch_path).ok();
        return Err(e);
    }

    Ok("Version saved successfully".into())
}





