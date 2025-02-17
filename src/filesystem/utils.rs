use std::{
    fs::{self},
    io, u64,
};

pub fn count_dir_entries(path: String) -> io::Result<u64> {
    let mut count = 0;

    for entry in fs::read_dir(path)? {
        match entry {
            Ok(_) => count += 1,
            Err(_) => continue,
        }
    }

    Ok(count)
}
