use std::fs::read_dir;
use std::io::Error;
use std::path::PathBuf;

pub fn is_video_file(path: &PathBuf) -> Result<bool, Error> {
    if path.is_dir() {
        return Ok(false);
    }
    let extension = path
        .extension()
        .and_then(|os_str| os_str.to_str())
        .or(Some(""))
        .unwrap_or_default();

    Ok(matches!(
        extension.to_lowercase().as_str(),
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm"
    ))
}

pub fn loop_dir(dir_path: &str) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    let entries = read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry.path());
        }
    }
    Ok(files)
}
