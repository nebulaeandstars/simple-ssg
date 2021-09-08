use clap::{load_yaml, App, AppSettings};

fn main() {
    let yaml = load_yaml!("cli.yml");

    #[allow(unused_variables)]
    let matches = App::from_yaml(yaml)
        .set_term_width(0)
        .global_settings(&[
            AppSettings::DisableHelpSubcommand,
            AppSettings::DeriveDisplayOrder,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .get_matches();
}
