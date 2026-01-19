use std::{fs, path::Path, io::Write};

const TMP_EXT: &str = ".tmp";

pub fn remove_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::remove_file(path)
}

pub fn read_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> std::io::Result<()> {
    let tmp_path = format!("{}{}", path.as_ref().display(), TMP_EXT);

    let mut file = fs::File::create(&tmp_path)?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;

    if let Err(e) = fs::rename(&tmp_path, path) {
        let _ = fs::remove_file(&tmp_path).ok();
        return Err(e);
    }
    Ok(())
}

pub fn import_file<P: AsRef<Path>>(src: P, dest: P) -> std::io::Result<()> {
    fs::copy(src, dest).map(|_| ())
}

