use md5;
use std::error::Error;
use std::fs::{rename, File};
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileInfo {
    path: PathBuf,
    md5_hash: String,
    new_path: PathBuf,
}

impl FileInfo {
    pub fn new(path: PathBuf) -> Self {
        FileInfo {
            path,
            md5_hash: String::new(),
            new_path: PathBuf::new(),
        }
    }

    fn calculate_md5(&mut self) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(&self.path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let hash = md5::compute(buffer);
        self.md5_hash = format!("{:x}", hash);
        Ok(())
    }

    pub fn rename_based_on_md5(mut self) -> Result<FileInfo, Box<dyn Error>> {
        self.calculate_md5()?;
        let md5_path: PathBuf = self.path.with_file_name(format!(
            "{}.{}",
            self.md5_hash,
            self.path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
        ));
        if md5_path != self.path {
            rename(&self.path, &md5_path)?;
        }
        self.new_path = md5_path;
        Ok(self)
    }

    /// 重命名回撤
    #[allow(dead_code)]
    pub fn undo_rename(self) -> Result<(), Box<dyn Error>> {
        if &self.new_path != &self.path {
            rename(&self.new_path, &self.path)?;
        }
        Ok(())
    }
}
