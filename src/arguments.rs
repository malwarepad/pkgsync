use std::{env, process::exit};

// Argv parser & multiple text-only implementations
// Copyright (C) 2024 Panagiotis

pub enum OperationType {
    ConfigToSystem,
    SystemToConfig,
    Prettier,
    Help,
    Info,
    Version,
}

pub fn help() {
    println!(
        "Usage:
  pkgsync [option]
  
MISCELLANEOUS
  --help                     show list of command-line options
  --info                     show saved config info
  --prettier                 improve readability of the json file

OPERATIONS
  --system-to-config         export system packages to the json file
  --config-to-system         apply the json file to the system"
    );
}

pub fn version() {
    println!("pkgsync - A program to help you keep track of packages explicitly installed in your system intuitively\nLicensed under GPLv3 - https://github.com/malwarepad/pkgsync");
}

fn summer_2022() {
    println!("The burden of memory is too heavy to handle. Everything I held dear has been changed, corrupted, vanished or died with time and now only exists in my mind. I live to carry those echoes within me into the future. Call it nostalgia, but I know in my heart that it must be more than that.");
}

pub fn check_args() -> OperationType {
    let arg_len = env::args().len();
    if arg_len > 2 {
        println!("Invalid amount of arguments/options!\n");
        help();
        exit(1);
    } else if arg_len < 2 {
        help();
        exit(0);
    }

    let argument = env::args().nth(1).expect("Could not parse first argument");

    if argument == "--config-to-system" {
        OperationType::ConfigToSystem
    } else if argument == "--system-to-config" {
        OperationType::SystemToConfig
    } else if argument == "--prettier" {
        OperationType::Prettier
    } else if argument == "--help" {
        OperationType::Help
    } else if argument == "--info" {
        OperationType::Info
    } else if argument == "--version" {
        OperationType::Version
    } else if argument == "--summer-2022" {
        summer_2022();
        exit(0);
    } else {
        println!("Invalid argument passed: {}\n", argument);
        help();
        exit(1);
    }
}
