use std::io::Error;
use std::path;

use crate::defaults;
use crate::utils::{create_directories, mkdir_and_cd};

/// Attempts to create a new project in the current working directory.
pub fn init() -> Result<(), Error> {
    create_directories(&defaults::project_directories())
}

/// Attempts to create a new project in a given directory.
pub fn new(path: &str) -> Result<(), Error> {
    let path = path::PathBuf::from(path);
    mkdir_and_cd(path)?;
    init()
}
