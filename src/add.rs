//! Implements the `tire add` command.
//!
//! The `add` command is mostly an alias for `uv add`, but it does have an additional `--auto` flag
//! which makes Tire parse your Python codebase and search for imports that can be mapped back to
//! known Python packages.

use crate::utils::{run_command_or_exit, string_vec};

pub fn add(args: Vec<String>, auto: bool) {
    if auto {
        panic!("tire add --auto is not currently implemented");
    }

    let mut uv_command = string_vec!["uv", "add"];
    uv_command.extend(args);

    run_command_or_exit(uv_command)
}
