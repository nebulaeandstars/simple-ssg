use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::{env, fs};

use crate::defaults;

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

/// Creates a default .gitignore in the current working directory from a
/// template.
pub fn create_gitignore() -> Result<(), Error> {
    let path = &get_path();

    let gitignore_content = &defaults::gitignore_content();

    // open the .gitignore file in write-only mode.
    let mut file = fs::File::create(format!("{}/.gitignore", path))?;

    // then write the template content it.
    for string in gitignore_content {
        file.write(format!("{}\n", string).as_bytes())?;
    }

    Ok(())
}

/// Returns whether a given directory is empty.
fn dir_is_empty(path: &str) -> Result<bool, Error> {
    Ok(fs::read_dir(path)?.count() != 0)
}
