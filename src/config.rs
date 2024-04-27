use chrono::Local;
use serde::{Deserialize, Serialize};

use std::path::Path;
use std::{env, fs};

// JSON config helpers
// Copyright (C) 2024 Panagiotis

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub pkgs: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub last_updated: String,
    pub categories: Vec<Category>,
}

fn path(name: &str) -> String {
    let home_path = env::var("HOME").unwrap();
    let folder_path = Path::new(&home_path).join(".config/pkgsync");
    fs::create_dir_all(&folder_path).expect("Could not create .config directory");

    let json_path = folder_path.join(name);

    let output = json_path.to_str().expect("Could not assemble path");
    output.to_string()
}

pub fn get() -> Config {
    let exists = Path::new(&path("pkgs.json")).exists();
    if !exists {
        fix();
        return get();
    }

    let read_cmd = fs::read_to_string(path("pkgs.json"));
    if read_cmd.is_err() {
        fix();
        return get();
    }
    let raw_json = read_cmd.unwrap();
    let parse_json_req: Result<Config, serde_json::Error> = serde_json::from_str(&raw_json);
    if parse_json_req.is_err() {
        // does field checking
        fix();
        return get();
    }

    let target: Config = parse_json_req.unwrap();
    target
}

pub fn update(input: Config) {
    let date = Local::now().format("%Y-%m-%dT%H%M%S").to_string();

    let mut target = input;
    target.last_updated = date;

    fs::write(
        path("pkgs.json"),
        serde_json::to_string_pretty(&target).expect("Could not update config"),
    )
    .expect("Could not generate default config");
}

pub fn prepare() {
    let exists = Path::new(&path("pkgs.json")).exists();
    if exists {
        backup();
    }

    generate();
}

pub fn info() {
    let exists = Path::new(&path("pkgs.json")).exists();
    if !exists {
        println!("No configuration file exists!");
        return;
    }

    let config = get();
    print!(
        "Location: {}\nLast update: {}\n\nPackages: ",
        path("pkgs.json"),
        config.last_updated
    );

    for category in config.categories {
        for pkg in category.pkgs {
            print!("{} ", pkg);
        }
    }
    println!();
}

pub fn prettier() {
    let exists = Path::new(&path("pkgs.json")).exists();
    if !exists {
        println!("No configuration file exists!");
        return;
    }

    let mut config = get();

    for category in &mut config.categories {
        category.pkgs.sort();
    }

    update(config);

    println!("Operation completed successfuly!");
}

fn backup() {
    println!("Backing up config...");
    let date = Local::now().format("%Y-%m-%dT%H%M%S").to_string();

    fs::rename(
        path("pkgs.json"),
        path(&format!("pkgs-backup-{}.json", date)),
    )
    .expect("Could not backup config!");
}

fn generate() {
    println!("Generating new config...");
    let date = Local::now().format("%Y-%m-%dT%H%M%S").to_string();
    let default = Config {
        last_updated: date,
        categories: vec![Category {
            name: String::from("other"),
            pkgs: vec![],
        }],
    };

    fs::write(
        path("pkgs.json"),
        serde_json::to_string_pretty(&default).expect("SOURCE FAULT! Outdated default config"),
    )
    .expect("Could not generate default config");
}

fn fix() {
    let exists = Path::new(&path("pkgs.json")).exists();
    if !exists {
        generate();
        return;
    }

    let raw_json = fs::read_to_string(path("pkgs.json")).expect("Unable to read file");
    let parse_json_req: Result<Config, serde_json::Error> = serde_json::from_str(&raw_json);
    if parse_json_req.is_err() {
        // does field checking
        println!("Bad config file! Fixing...");
        backup();
        generate();
    }
}
