use std::io;

use gtk4::{Application, ApplicationWindow};

use crate::filesystem::file_item::FileItem;

pub struct GlobalConfig<'a> {
    current_dir: FileItem,
    _application_window: &'a ApplicationWindow,
    _application: &'a Application,
}

impl<'a> GlobalConfig<'a> {
    pub fn new(
        current_dir: String,
        _application_window: &'a ApplicationWindow,
        _application: &'a Application,
    ) -> io::Result<Self> {
        let current_dir_item = FileItem::new(current_dir)?;
        if !current_dir_item.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid current directory",
            ));
        }

        Ok(GlobalConfig {
            current_dir: current_dir_item,
            _application_window,
            _application,
        })
    }
}
