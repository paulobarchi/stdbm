mod settings;

use clap::{arg, Command};
use settings::Settings;

fn cli() -> Command<'static> {
    Command::new("stdbm")
        .about("Sentences-Tags DataBase Manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("loadall") // requires --force arg
        )
        .subcommand(
            Command::new("load")
        )
        .subcommand(
            Command::new("add")
        )
        .subcommand(
            Command::new("update")
        )
        .subcommand(
            Command::new("list")
        )
        .subcommand(
            Command::new("filter")
        )
        .subcommand(
            Command::new("rm")
        )
        .subcommand(
            Command::new("savedb")
        )
}

fn main() {
    let settings = Settings::new();

    // // Print out our settings
    // println!("{:?}", settings);

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("load", sub_matches)) => {
            println!(
                "Loading {}",
                sub_matches.value_of("REMOTE").expect("required")
            );
        }

}
