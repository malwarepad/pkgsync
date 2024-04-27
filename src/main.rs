mod arguments;
mod config;
mod parser;
mod sanitychecks;
mod sync;

// PkgSync entry :)
// Copyright (C) 2024 Panagiotis

fn main() {
    sanitychecks::arch(); // pacman, yay

    match arguments::check_args() {
        arguments::OperationType::Help => {
            arguments::help();
        }
        arguments::OperationType::Version => {
            arguments::version();
        }
        arguments::OperationType::Info => {
            config::info();
        }
        arguments::OperationType::Prettier => {
            config::prettier();
        }
        arguments::OperationType::ConfigToSystem => {
            sync::config_to_system();
        }
        arguments::OperationType::SystemToConfig => {
            sync::system_to_config();
        }
    }
}
