use std::process::exit;

/// Similar to unwrap(), but excecutes a controlled crash with a clean
/// output. Used for notifying the user of a bad input, etc.
pub trait CleanUnwrap<T> {
    fn clean_unwrap(self) -> T;
}

impl CleanUnwrap<()> for Result<(), std::io::Error> {
    fn clean_unwrap(self) -> () {
        match self {
            Ok(value) => value,
            Err(error) => {
                println!("ERROR: {}", error);
                exit(std::io::ErrorKind::Other as i32)
            }
        }
    }
}
