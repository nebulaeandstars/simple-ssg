use std::fs;
use std::io::{Error, ErrorKind};

use crate::defaults;
use crate::utils::get_path;

/// Attempts to create a new project in the current working directory.
pub fn init() -> Result<(), Error> {
    if let Err(error) = create_directories(&defaults::project_directories()) {
        Err(Error::new(error.kind(), "Working directory is not empty!"))
    } else {
        Ok(())
    }
}

/// Creates a number of subdirectories (given an array of their names) if and
/// only if the current working directory is empty.
fn create_directories(directories: &[&str]) -> Result<(), Error> {
    let path = &get_path();

    // If it isn't empty, return an error.
    if fs::read_dir(path).unwrap().count() != 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("{} is not empty!", path),
        ));
    }

    // Otherwise, create the given directories
    for dir in directories {
        fs::create_dir(format!("{}/{}", path, dir)).unwrap();
    }

    Ok(())
}
