//! Implements the `tire check` command.

use std::path::Path;

use crate::{
    profile::Profile,
    utils::{find_pyproject_toml, run_command_or_exit, string_vec},
};

pub fn check(files: Vec<String>) {
    // TODO: Check if the pyproject_toml contains `tool.tire.inflated=true`.

    // Load the project's pyproject.toml
    let pyproject_toml = find_pyproject_toml().unwrap();
    let pyproject_toml = std::fs::read_to_string(pyproject_toml).unwrap();
    let pyproject_toml = pyproject_toml.parse::<toml::Table>().unwrap();

    // Load the profile for the project
    let profile = Profile::load_file(Path::new("../profiles/default.toml")).unwrap();
    profile.validate().unwrap();
    eprintln!("========");
    eprintln!("profile: {profile:?}");

    // Merge the profile and the pyproject.toml
    let merged = profile.merge(&pyproject_toml);
    eprintln!("merged: {merged:?}");
    eprintln!("========");
    eprintln!("========");

    // TODO: Get Mypy
    let mut uv_command = string_vec!["uv", "run", "--with", "mypy", "dmypy", "run"];
    if files.is_empty() {
        uv_command.push(".".to_owned());
    } else {
        uv_command.extend(files);
    }

    run_command_or_exit(uv_command)
}
