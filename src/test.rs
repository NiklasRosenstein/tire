//! Implements the `tire test` command.

use crate::{
    profile::Profile,
    utils::{run_command_or_exit, string_vec},
};

pub fn test(
    files: Vec<String>,
    _allow_no_tests: bool,
    parallel: Option<i32>,
    filter: Option<String>,
) {
    // Write the merged configuration to a temporary file
    let pyproject_toml = Profile::load(None).unwrap().materialize(None).unwrap();

    // Run dmypy with the merged config file
    let mut uv_command = string_vec![
        "uv",
        "run",
        "--with",
        "pytest",
        "--with",
        "pytest-xdist",
        "pytest",
        "--config-file",
        pyproject_toml.to_string_lossy().to_string()
    ];

    uv_command.push("-n".to_owned());
    uv_command.push(
        parallel
            .map(|x| x.to_string())
            .unwrap_or("auto".to_string()),
    );

    if let Some(filter) = filter {
        uv_command.push("-k".to_owned());
        uv_command.push(filter);
    }

    if files.is_empty() {
        uv_command.push(".".to_owned());
    } else {
        uv_command.extend(files);
    }

    // Run the command
    // TODO: Check if exit code == 5 (no tests found) and don't error if allow_no_tests is enabled
    run_command_or_exit(uv_command);
}
