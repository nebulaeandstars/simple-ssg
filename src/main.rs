mod defaults;
mod init;
mod result;
mod utils;

use std::process::exit;

use clap::{load_yaml, App, AppSettings};

use crate::result::CleanUnwrap;
use crate::utils::get_path;

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
        init::init().clean_unwrap();
        println!("Created new project at {}", get_path());
        exit(0);
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        if let Some(path) = matches.value_of("PATH") {
            init::new(path).clean_unwrap();
            println!("Created new project at {}", get_path());
            exit(0);
        }
    }
}
