use clap::{app_from_crate, arg, App, AppSettings};
use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;
use std::process::exit;

fn main() -> Result<()> {
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
        Some(("set", m)) => {
            let key = m.value_of("KEY").unwrap();
            let mut store = KvStore::open(current_dir()?)?;
            let value = m.value_of("VALUE").unwrap();
            store.set(key.to_string(), value.to_string())?;
        }
        Some(("get", m)) => {
            let key = m.value_of("KEY").unwrap();
            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Some(("rm", m)) => {
            let key = m.value_of("KEY").unwrap();
            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(_) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
