use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::{env, fs};

/// Returns the path to the current working directory.
pub fn get_path() -> String {
    return env::current_dir()
        .expect("Could not read working directory!")
        .to_str()
        .unwrap()
        .to_string();
}

/// Move into a given directory, creating it if it doesn't already exist.
pub fn mkdir_and_cd(path: PathBuf) -> Result<(), std::io::Error> {
    fs::create_dir_all(&path)?;
    env::set_current_dir(path)
}

/// Creates a number of subdirectories (given an array of their names) if and
/// only if the current working directory is empty.
pub fn create_directories(directories: &[&str]) -> Result<(), Error> {
    let path = &get_path();

    if dir_is_empty(path)? {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Directory is not empty!"),
        ));
    }

    // Otherwise, create the given directories
    for dir in directories {
        fs::create_dir(format!("{}/{}", path, dir)).unwrap();
    }

    Ok(())
}

/// Returns whether a given directory is empty.
fn dir_is_empty(path: &str) -> Result<bool, Error> {
    Ok(fs::read_dir(path)?.count() != 0)
}
