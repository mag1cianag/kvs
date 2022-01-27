use std::path::{Path};
use clap::{app_from_crate, arg, App};

fn main() {
    let matches = app_from_crate!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(arg!( -c --config <FILE> "Sets a custom config file")
                 .required(false)
                 .allow_invalid_utf8(true),
        )
        .arg(arg!(-d --debug ... "Turn debugging information on"))
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg(arg!(
                    -l --list "list test values"
                )),
        )
        .get_matches();
    if let Some(name) = matches.value_of("name") {
        println!("Value for name: {}", name);
    }
    if let Some(raw_config) = matches.value_of_os("config") {
        let config_path = Path::new(raw_config);
        println!("Value for config: {}", config_path.display());
    }

    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy")
    }
    if let Some(matches)  = matches.subcommand_matches("test") {
       if matches.is_present("list") {
           println!("Printing testing lists...");
       } else {
           println!("Not printing testing lists...");
       }
    }
}