use std::fs;

const TMP_EXT: &str = ".tmp";

pub fn remove_file(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)
}

pub fn read_file(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file(path: &str, content: &str) -> std::io::Result<()> {
    let tmp_path = format!("{}{}", path, TMP_EXT);
    fs::write(&tmp_path, content)?;
    if let Err(e) = fs::rename(&tmp_path, path) {
        let _ = fs::remove_file(&tmp_path).ok();
        return Err(e);
    }
    Ok(())
}
