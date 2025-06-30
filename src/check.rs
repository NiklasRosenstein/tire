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

    // Merge the profile and the pyproject.toml
    let merged = profile.merge(&pyproject_toml).unwrap();

    // Write the merged configuration to a temporary file
    let temp_file = std::env::temp_dir().join("tire_merged_config.toml");
    let merged_toml = toml::to_string(&merged).unwrap();

    // eprintln!("Final toml: {merged_toml}");
    std::fs::write(&temp_file, merged_toml).unwrap();

    // Run dmypy with the merged config file
    let temp_file_path = temp_file.to_string_lossy().to_string();
    let mut uv_command = string_vec![
        "uv",
        "run",
        "--with",
        "mypy",
        "dmypy",
        "run",
        "--",
        "--config-file",
        temp_file_path.clone()
    ];
    if files.is_empty() {
        uv_command.push(".".to_owned());
    } else {
        uv_command.extend(files);
    }

    // Run the command
    run_command_or_exit(uv_command);

    // Ensure the temp file is removed afterwards
    std::fs::remove_file(temp_file)
        .unwrap_or_else(|_| eprintln!("Failed to remove temporary file: {}", temp_file_path));
}
