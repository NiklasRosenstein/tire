//! Implements the `tire check` command.

use crate::{
    profile::Profile,
    utils::{run_command_or_exit, string_vec, find_workspace_root},
};

pub fn check(files: Vec<String>) {
    // Load the project's pyproject.toml
    let pyproject_toml = Profile::load(None).unwrap().materialize(None).unwrap();

    // The dmypy status file should sit next to the pyproject.toml, to reuse the same daemon
    // for the same project even if run in a subdirectory.
    // If the current project is a workspace member, use the workspace root instead.
    let status_file_dir = match find_workspace_root(&pyproject_toml) {
        Some(workspace_root) => workspace_root.parent().unwrap().to_path_buf(),
        None => pyproject_toml.parent().unwrap().to_path_buf(),
    };
    let status_file = status_file_dir.join(".dmypy.json");

    // Run dmypy with the merged config file
    let mut uv_command = string_vec![
        "uv",
        "run",
        "--with",
        "mypy",
        "dmypy",
        "--status-file",
        status_file.to_string_lossy(),
        "run",
        "--",
        "--config-file",
        pyproject_toml.to_string_lossy().to_string()
    ];
    if files.is_empty() {
        uv_command.push(".".to_owned());
    } else {
        uv_command.extend(files);
    }

    // Run the command
    run_command_or_exit(uv_command);
}
