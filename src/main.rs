mod config;
mod sync;

use std::{
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
};

use clap::{arg, command, value_parser, Command};
use config::{TkConfig, TkProject};
use reqwest::StatusCode;
use sync::TkFont;

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
                .arg(arg!(-r --replace "Overwrite existing project id if it exists")),
        )
        .get_matches();

    match matches.subcommand() {
        None => sync_all(),
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

fn add(id: &str, name: &str, path: &Path, replace: bool) {
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

    let absolute_path = match std::fs::canonicalize(path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error creating absolute path.\nError: {e}");
            return;
        }
    };

    // check if directory has write permissions
    let metadata = match fs::metadata(&absolute_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!(
                "Error getting metadata for directory {}.\nError: {e}",
                absolute_path.display()
            );
            return;
        }
    };

    if metadata.permissions().readonly() {
        eprintln!(
            "Error syncing project to path {}: Permission Denied.",
            absolute_path.display()
        );
        return;
    }

    let url = format!("https://use.typekit.net/{id}.css");
    let res = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error checking for project {id}.\nError: {e}");
            return;
        }
    };

    if res.status() != StatusCode::OK {
        eprintln!(
            "Error adding typekit project id '{id}'.\nNo typekit project found with that Id."
        );
        return;
    }

    let project = TkProject {
        name: name.into(),
        path: absolute_path,
    };

    if let Err(e) = config.add_or_replace(id, project) {
        println!("There was an error adding the project.\nError: {e}")
    };
}

fn sync_all() {
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

    for (id, project) in config.iter() {
        sync_project(id, project);
    }

    println!("Sync Completed.");
}

fn sync_project(id: &String, project: &TkProject) {
    println!("Syncing project {id}-{}", project.name);
    let url = format!("https://use.typekit.net/{id}.css");

    let mut res = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error syncing project {id}.\nError: {e}");
            return;
        }
    };

    if res.status() != StatusCode::OK {
        eprintln!("Error syncing project {id}.\nCould not find typekit project with that Id.");
        return;
    }

    let mut css = String::new();
    if let Err(e) = res.read_to_string(&mut css) {
        eprintln!("Error parsing project {id}.\nError: {e}");
        return;
    };

    let sub_folder_name = format!("tksync-{}-{}", id, project.name);
    let sub_folder = Path::new(&sub_folder_name);
    let mut save_path = project.path.clone();

    if !save_path.exists() {
        eprintln!("Error syncing project {id}.\nProject path does not exist.");
        return;
    }

    if !save_path.is_absolute() {
        eprintln!("Error syncing project {id}.\nProject path is not absolute.");
        return;
    }

    if !save_path.is_dir() {
        eprintln!("Error syncing project {id}.\nProject path is not a directory.");
        return;
    }

    save_path.push(sub_folder);
    if let Err(e) = fs::create_dir_all(&save_path) {
        eprintln!("Error creating directory for {id}.\nError: {e}");
        return;
    }

    let fonts = TkFont::parse_css(&css);

    let files = match fs::read_dir(&save_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading files in directory for project {id}.\nError: {e}");
            return;
        }
    };

    for file in files {
        let file = match file {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error reading file in directory for project {id}.\nError: {e}");
                return;
            }
        };

        let os_name = file.file_name();
        let Ok(file_type) = file.file_type() else {
            continue;
        };

        if !file_type.is_file() {
            continue;
        }

        let name = match os_name.to_str() {
            Some(n) => match n.strip_suffix(".otf") {
                Some(n) => n,
                None => continue,
            },
            None => continue,
        };

        if !fonts.iter().any(|font| font.full_name() == name) {
            println!("Removing old font: {name}");
            if let Err(e) = fs::remove_file(file.path()) {
                eprintln!("Error deleting file {name}.\nError: {e}");
                return;
            }
        }
    }

    for font in fonts.iter() {
        save_font(font, &save_path);
    }
}

fn save_font(font: &TkFont, path: &Path) {
    let font_name = font.full_name();
    let font_path = path.to_owned().join(format!("{font_name}.otf"));
    if font_path.exists() {
        return;
    }

    println!("Downloading new font: {font_name}");
    let mut res = match reqwest::blocking::get(&font.opentype) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error downloading font {font_name}.\nError: {e}");
            return;
        }
    };

    let mut file = match File::create(&font_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "Error creating font file {}.\nError: {e}",
                font_path.display()
            );
            return;
        }
    };

    if let Err(e) = io::copy(&mut res, &mut file) {
        eprintln!("Error writing to file {}.\nError: {e}", font_path.display());
        return;
    };
}
