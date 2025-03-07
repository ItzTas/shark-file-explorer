use std::fs;
use std::io;

use super::utils::count_dir_entries;

pub struct FileItem {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
}

impl FileItem {
    pub fn new(path: String) -> io::Result<Self> {
        let metadata = fs::metadata(&path).map_err(|e| e)?;

        let is_dir = metadata.is_dir();
        let size = if is_dir {
            count_dir_entries(path.clone())?
        } else {
            metadata.len()
        };

        let name = path.split("/").last().unwrap_or("").to_string();

        Ok(FileItem {
            name,
            path,
            is_dir,
            size,
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
