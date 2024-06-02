use crate::{config, parser, sanitychecks};
use std::process::Command;

// Sync the config file & backwards
// Copyright (C) 2024 Panagiotis

pub fn system_to_config() {
    config::prepare();
    let mut config = config::get();
    let pkgs = parser::get_installed_pkgs();

    let category = config
        .categories
        .iter_mut()
        .find(|o| o.name == "archlinux")
        .expect("Category with the name 'archlinux' could not be found");
    for pkg in pkgs {
        category.pkgs.push(pkg);
    }
    config::update(config);
}

pub fn config_to_system() {
    let config = config::get();

    let category = config
        .categories
        .iter()
        .find(|o| o.name == "archlinux")
        .expect("Category with the name 'archlinux' could not be found");

    let config_list = &category.pkgs;

    let system_list = parser::get_installed_pkgs();
    let install_list = parser::get_extra(&config_list, &system_list);
    let uninstall_list = parser::get_extra(&system_list, &config_list);

    let to_be_installed: usize = install_list.len();
    if to_be_installed > 0 {
        let mut str = format!("[installing:{}]: ", to_be_installed);
        for (i, pkg) in install_list.iter().enumerate() {
            str.push_str(pkg);
            if i != to_be_installed - 1 {
                str.push(' ');
            }
        }

        let mut args = install_list;
        args.insert(0, String::from("--noconfirm"));
        args.insert(0, String::from("-S"));
        if sanitychecks::choice(str) {
            takeover_helper_operation(args);
        } else {
            println!("Aborted installation successfuly!");
        }
    }

    let to_be_uninstalled: usize = uninstall_list.len();
    if to_be_uninstalled > 0 {
        let mut str = format!("[uninstalling:{}]: ", to_be_uninstalled);
        for (i, pkg) in uninstall_list.iter().enumerate() {
            str.push_str(pkg);
            if i != to_be_uninstalled - 1 {
                str.push(' ');
            }
        }

        let mut args = uninstall_list;
        args.insert(0, String::from("--noconfirm"));
        args.insert(0, String::from("-R"));
        if sanitychecks::choice(str) {
            takeover_helper_operation(args);
        } else {
            println!("Aborted uninstallation successfuly!");
        }
    }
}

fn takeover_helper_operation(args: Vec<String>) {
    let _ = Command::new("yay")
        .args(args)
        .spawn()
        .expect("Yay failed to execute")
        .wait_with_output();
}
