mod config;

use std::path::{Path, PathBuf};

use clap::{arg, command, value_parser, Command};
use config::{TkConfig, TkProject};

fn main() {
    let matches = command!()
        .subcommand(Command::new("ls").about("Prints the configuration file"))
        .subcommand(
            Command::new("add")
                .about("Adds a new project to be tracked")
                .arg(arg!(<ID> "Id of the typekit project").required(true))
                .arg(arg!(<NAME> "Name of typekit project").required(true))
                .arg(
                    arg!(<PATH> "Path to download project fonts to")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(arg!(-r --replace "Overwrite existing project id it exists")),
        )
        .get_matches();

    match matches.subcommand() {
        None => sync(),
        Some(("ls", _)) => list(),
        Some(("add", add_matches)) => {
            add(
                add_matches.get_one::<String>("ID").expect("required"),
                add_matches.get_one::<String>("NAME").expect("required"),
                add_matches.get_one::<PathBuf>("PATH").expect("required"),
                add_matches.get_flag("replace"),
            );
        }
        _ => unreachable!(),
    }
}

fn list() {
    let config = match TkConfig::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config file.\nError: {e}");
            return;
        }
    };

    for (id, project) in config.iter() {
        let name = &project.name;
        let path = &project.path.display();
        println!("{id}: {name} -> {path}");
    }
}

fn add(name: &str, id: &str, path: &Path, replace: bool) {
    let mut config = match TkConfig::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config file.\nError: {e}");
            return;
        }
    };

    if config.contains(id) && !replace {
        eprintln!(
            "Project ID already exists in config file\n\
            To replace it, use the --replace flag."
        );
        return;
    }

    let project = TkProject {
        name: name.into(),
        path: path.into(),
    };

    if let Err(e) = config.add_or_replace(id, project) {
        println!("There was an error adding the project.\nError: {e}")
    };
}

fn sync() {
    let config = match TkConfig::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config file.\nError: {e}");
            return;
        }
    };

    if config.is_empty() {
        println!(
            "No projects configured.\n\
            Use 'tksync add' to add a tracked project."
        );
        return;
    }

    println!("Starting Typekit Sync...");
    // TODO: Sync projects
    println!("Sync Completed.");
}
