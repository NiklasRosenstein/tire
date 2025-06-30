//! Implements the `tire check` command.

use crate::{
    profile::materialize_pyproject_toml_to_tmp,
    utils::{run_command_or_exit, string_vec},
};

pub fn fmt(files: Vec<String>, check: bool) {
    // Write the merged configuration to a temporary file
    let pyproject_toml = materialize_pyproject_toml_to_tmp();

    // Run dmypy with the merged config file
    let mut uv_command = string_vec![
        "uv",
        "run",
        "--with",
        "ruff",
        "ruff",
        "--config",
        pyproject_toml.path.to_string_lossy().to_string(),
        "format"
    ];

    if check {
        uv_command.push("--check".to_owned());
    }

    if files.is_empty() {
        uv_command.push(".".to_owned());
    } else {
        uv_command.extend(files);
    }

    // Run the command
    run_command_or_exit(uv_command);
}
