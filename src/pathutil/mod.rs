use std::{error::Error, path::Path};

pub fn join_string_path(path: &str, to: &str) -> String {
    return String::from(
        Path::new(path)
            .join(to)
            .to_str()
            .expect("Failed to convert new path to string")
    );
}

pub fn is_file(path: &str) -> Result<bool, Box<dyn Error>> {
    let path_buf = Path::new(path);
    Ok(path_buf.is_file())
}

pub fn is_directory_empty(path: &str) -> Result<bool, Box<dyn Error>> {
    let path_buf = Path::new(path);

    if !path_buf.is_dir() {
        return Ok(true);
    }

    Ok(path_buf.read_dir()?.next().is_none())
}