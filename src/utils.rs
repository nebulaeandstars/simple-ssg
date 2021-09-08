use std::env;

/// Returns the path to the current working directory.
pub fn get_path() -> String {
    return env::current_dir()
        .expect("Could not read working directory!")
        .to_str()
        .unwrap()
        .to_string();
}
