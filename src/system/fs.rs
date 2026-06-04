use std::path::{Path, PathBuf};

pub fn scan_library(dir: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();

        if let Some(ext) = path.extension() {
            match ext.to_str() {
                Some("mp3" | "flac" | "wav" | "ogg") => {
                    files.push(path);
                }
                _ => {}
            }
        }
    }

    Ok(files)
}
