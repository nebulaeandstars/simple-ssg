use std::process::exit;

use clap::{load_yaml, App, AppSettings};

use crate::utils::get_path;

mod defaults;
mod init;
mod utils;

fn main() -> Result<(), std::io::Error> {
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
        if let Err(error) = init::init() {
            println!("{}", error);
            exit(std::io::ErrorKind::Other as i32);
        } else {
            println!("Created new project at {}", get_path())
        }
    }

    Ok(())
}
