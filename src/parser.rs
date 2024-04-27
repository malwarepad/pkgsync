use std::{
    collections::HashSet,
    process::{exit, Command},
};

// Archlinux explicitly installed package parser
// Copyright (C) 2024 Panagiotis

pub fn get_installed_pkgs() -> Vec<String> {
    let cmd = Command::new("pacman").args(["-Qett"]).output();
    if cmd.is_err() {
        println!("Failed to detect packages!");
        exit(1);
    }

    let cmd_res = cmd.unwrap();
    if !cmd_res.status.success() {
        println!("Failed to detect packages!");
        exit(1);
    }

    let str = String::from_utf8(cmd_res.stdout).expect("Invalid UTF-8");
    let mut pkgs_with_versions: Vec<&str> = str.split('\n').collect();
    pkgs_with_versions.truncate(pkgs_with_versions.len() - 1);
    let pkgs_wo_versions = pkgs_with_versions
        .iter()
        .map(|line| {
            let space_index = line.find(' ').unwrap_or(line.len());
            line[..space_index].to_string()
        })
        .collect();

    pkgs_wo_versions
}

pub fn get_extra(priority: &[String], extra: &[String]) -> Vec<String> {
    let set_a: HashSet<&String> = priority.iter().collect();
    let set_b: HashSet<&String> = extra.iter().collect();

    let difference: Vec<&String> = set_a.difference(&set_b).cloned().collect();
    let copied: Vec<String> = difference.iter().map(|v| v.to_string()).collect();

    copied
}
