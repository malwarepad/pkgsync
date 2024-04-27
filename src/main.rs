use std::process::exit;

mod arguments;
mod config;
mod parser;
mod sanitychecks;
mod sync;

// PkgSync entry :)
// Copyright (C) 2024 Panagiotis

fn main() {
    sanitychecks::arch(); // pacman, yay

    let argument = arguments::check_args();
    match argument.as_str() {
        "--help" => {
            arguments::help();
        }
        "--version" => {
            arguments::version();
        }
        "--info" => {
            config::info();
        }
        "--prettier" => {
            config::prettier();
        }
        "--config-to-system" => {
            sync::config_to_system();
        }
        "--system-to-config" => {
            sync::system_to_config();
        }
        _ => {
            eprintln!("Invalid argument passed: {}\n", argument);
            arguments::help();
            exit(1);
        }
    }
}
