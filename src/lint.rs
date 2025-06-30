//! Implements the `tire check` command.

use crate::{
    profile::Profile,
    utils::{run_command_or_exit, string_vec},
};

pub fn lint(files: Vec<String>, fix: bool, unsafe_fixes: bool) {
    // Write the merged configuration to a temporary file
    let pyproject_toml = Profile::load(None).unwrap().materialize(None).unwrap();

    // Run dmypy with the merged config file
    let mut uv_command = string_vec![
        "uv",
        "run",
        "--with",
        "ruff",
        "ruff",
        "--config",
        pyproject_toml.to_string_lossy().to_string(),
        "check"
    ];

    if fix {
        uv_command.push("--fix".to_owned());
        if unsafe_fixes {
            uv_command.push("--unsafe-fixes".to_owned());
        }
    }
    // TODO: Error if unsafe_fixes is set but not fix?

    if files.is_empty() {
        uv_command.push(".".to_owned());
    } else {
        uv_command.extend(files);
    }

    // Run the command
    run_command_or_exit(uv_command);
}
