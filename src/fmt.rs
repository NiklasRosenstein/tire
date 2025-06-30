//! Implements the `tire check` command.

use crate::{
    profile::Profile,
    utils::{run_command_or_exit, string_vec},
};

pub fn fmt(files: Vec<String>, check: bool) {
    // Write the merged configuration to a temporary file
    let pyproject_toml = Profile::load(None).unwrap().materialize(None).unwrap();

    // TODO: Do not fail fast on the commands.

    // Run dmypy with the merged config file
    {
        let mut uv_command = string_vec![
            "uv",
            "run",
            "--with",
            "ruff",
            "ruff",
            "--config",
            pyproject_toml.to_string_lossy().to_string(),
            "format"
        ];

        if check {
            uv_command.push("--check".to_owned());
        }

        if files.is_empty() {
            uv_command.push(".".to_owned());
        } else {
            uv_command.extend(files.clone());
        }

        run_command_or_exit(uv_command);
    }

    // Check isort rules
    // TODO: Only if the profile includes `select = ["I"]` in the ruff config
    {
        let mut uv_command = string_vec![
            "uv",
            "run",
            "--with",
            "ruff",
            "ruff",
            "--config",
            pyproject_toml.to_string_lossy().to_string(),
            "check",
            "--select",
            "I"
        ];

        if !check {
            uv_command.push("--fix".to_owned());
        }

        if files.is_empty() {
            uv_command.push(".".to_owned());
        } else {
            uv_command.extend(files);
        }

        run_command_or_exit(uv_command);
    }
}
