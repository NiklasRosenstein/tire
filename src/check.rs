//! Implements the `tire check` command.

use crate::{
    profile::materialize_pyproject_toml_to_tmp,
    utils::{find_pyproject_toml, run_command_or_exit, string_vec},
};

pub fn check(files: Vec<String>) {
    // Load the project's pyproject.toml
    let pyproject_toml_file = find_pyproject_toml().unwrap();
    let pyproject_toml = materialize_pyproject_toml_to_tmp();

    // The dmypy status file should sit next to the pyproject.toml, to reuse the same daemon
    // for the same project even if run in a subdirectory.
    // TODO: Use the root `pyproject.toml` for a Uv workspace project if the current project is
    //       a workspace member.
    let status_file = pyproject_toml_file
        .parent()
        .unwrap()
        .join(".dmypy.json")
        .to_path_buf();

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
        pyproject_toml.path.to_string_lossy().to_string()
    ];
    if files.is_empty() {
        uv_command.push(".".to_owned());
    } else {
        uv_command.extend(files);
    }

    // Run the command
    run_command_or_exit(uv_command);
}
