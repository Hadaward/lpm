use std::path::Path;

pub fn join_string_path(path: &str, to: &str) -> String {
    return String::from(
        Path::new(path)
            .join(to)
            .to_str()
            .expect("Failed to convert new path to string")
    );
}