use std::{fs, path::PathBuf};

use clap::{arg, command, value_parser, ArgMatches};

fn main() {
    let matches = clap_matches();

    // if a custom config was given, use it. Otherwise use the default config.yml
    match matches.get_one::<PathBuf>("config") {
        Some(file) => sync(file),
        None => {
            let file = PathBuf::from("config.yml");
            sync(&file);
        }
    };
}

fn clap_matches() -> ArgMatches {
    command!()
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file for this run")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches()
}

fn sync(config_file: &PathBuf) {
    let config_str = match fs::read_to_string(config_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading config file.\n{e}");
            return;
        }
    };

    println!("Syncing! \n{config_str}");
}
