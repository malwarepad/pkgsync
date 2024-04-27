use inquire::Select;
use std::process::{exit, Command};

// Sanity checks & interfaces
// Copyright (C) 2024 Panagiotis

pub fn arch() {
    if Command::new("pacman").args(["--version"]).output().is_err() {
        println!("Pacman package manager not detected!");
        exit(1);
    }

    if Command::new("yay").args(["--version"]).output().is_err() {
        println!("Yay AUR helper not detected!");
        exit(1);
    }
}

pub fn choice(msg: String) -> bool {
    let options = vec!["Cancel", "Proceed"];
    let ans = Select::new(&msg, options)
        .prompt()
        .expect("Could not parse choice");

    ans == "Proceed"
}
