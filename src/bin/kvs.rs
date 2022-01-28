use clap::{app_from_crate, arg, App, AppSettings};
use std::process::exit;

fn main() {
    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(
            App::new("set")
                .about("Set the value of a string key to a string")
                .arg(arg!(<KEY> "A string key"))
                .arg(arg!(<VALUE> "The string value of the key")),
        )
        .subcommand(
            App::new("get")
                .about("Get the string value of a given string key")
                .arg(arg!(<KEY> "A string key")),
        )
        .subcommand(
            App::new("rm")
                .about("Remove a given key")
                .arg(arg!(<KEY> "A string key")),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("set", _m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("get", _m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("rm", _m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
