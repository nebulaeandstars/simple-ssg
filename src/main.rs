use std::process::exit;

use clap::{load_yaml, App, AppSettings};

use crate::utils::get_path;

mod defaults;
mod init;
mod utils;

/// Similar to unwrap(), but excecutes a controlled crash with a clean output.
/// Used for notifying the user of a bad input, etc.
fn clean_unwrap<T>(result: Result<T, std::io::Error>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            println!("ERROR: {}", error);
            exit(std::io::ErrorKind::Other as i32)
        }
    }
}


fn main() {
    // parse command line arguments using clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .set_term_width(0)
        .settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DeriveDisplayOrder,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        clean_unwrap::<()>(init::init());
        println!("Created new project at {}", get_path())
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        if let Some(path) = matches.value_of("PATH") {
            clean_unwrap::<()>(init::new(path));
            println!("Created new project at {}", get_path())
        }
    }
}
