/// Like `vec!`, but returning a `Vec<String>`.
macro_rules! string_vec {
    // match a list of expressions separated by comma:
    ($($str:expr),*) => ({
        // create a Vec with this list of expressions,
        // calling String::from on each:
        vec![$(String::from($str),)*] as Vec<String>
    });
}

use std::path::PathBuf;

pub(crate) use string_vec;

/// Run the given command. If the command exits with a non-zero status code, print to stderr
/// and exit the process.
pub fn run_command_or_exit(command: Vec<String>) {
    eprintln!("[tire] $ {command:?}");
    let program = &command[0];
    let mut proc = std::process::Command::new(program)
        .args(command[1..].iter())
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to run program `{program}`"));
    let status = proc
        .wait()
        .unwrap_or_else(|_| panic!("Failed to wait for program `{program}`"));
    if !status.success() {
        let code = status.code().unwrap();
        eprintln!("Command `{program}` exited with code {code}");
        std::process::exit(status.code().unwrap_or(1));
    }
}

/// Find the `pyproject.toml` in the current working directory or any of its parent directories.
pub fn find_pyproject_toml() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().unwrap();
    loop {
        let file = dir.join("pyproject.toml");
        if std::fs::exists(&file).unwrap() {
            return Some(file);
        }
        dir = match dir.parent() {
            Some(dir) => dir.to_path_buf(),
            None => return None,
        }
    }
}
