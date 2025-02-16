pub struct FileItem {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
}

impl FileItem {
    fn new(path: String) -> Result<Self> {}
}
