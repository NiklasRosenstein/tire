//! Implements the `tire add` command.
//!
//! The `add` command is mostly an alias for `uv add`, but it does have an additional `--auto` flag
//! which makes Tire parse your Python codebase and search for imports that can be mapped back to
//! known Python packages.

pub fn add(args: Vec<String>, auto: bool) {
    if auto {
        panic!("tire add --auto is not currently implemented");
    }

    let mut uv_command: Vec<String> = vec!["uv", "add"].into_iter().map(String::from).collect();
    uv_command.extend(args);

    let mut proc = std::process::Command::new(&uv_command[0])
        .args(uv_command[1..].iter())
        .spawn()
        .expect("Failed to start uv command");
    let status = proc.wait().expect("Failed to wait for uv command");
    if !status.success() {
        eprintln!("uv command failed with status: {status}");
        std::process::exit(status.code().unwrap_or(1));
    }
}
