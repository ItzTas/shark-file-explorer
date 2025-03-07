use std::{fs, io};

use super::file_item::FileItem;

pub fn list(path: &str) -> io::Result<Vec<FileItem>> {
    let mut entries: Vec<FileItem> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        let path = entry.path();

        entries.push(FileItem::new(path.to_string_lossy().into_owned())?);
    }

    Ok(entries)
}
