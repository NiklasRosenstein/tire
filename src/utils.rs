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

/// Find a `pyproject.toml` file starting from the specified *cwd* (or the processes' current dir
/// if [None] is specified), walking up the file system hierarchy until it is found or return
/// [None].
pub fn find_pyproject_toml(cwd: Option<PathBuf>) -> Option<PathBuf> {
    let mut dir = cwd.ok_or("").or_else(|_| std::env::current_dir()).unwrap();
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

/// Find the workspace root `pyproject.toml` if the given project is part of a UV workspace.
/// 
/// This function starts from the given `pyproject.toml` file and walks up the directory tree
/// looking for a workspace root (a `pyproject.toml` with a `[tool.uv.workspace]` section).
/// Returns the path to the workspace root `pyproject.toml` if found, otherwise [None].
pub fn find_workspace_root(project_pyproject_toml: &std::path::Path) -> Option<PathBuf> {
    let mut dir = project_pyproject_toml.parent()?;
    
    // Walk up the directory tree looking for a workspace root
    loop {
        let candidate = dir.join("pyproject.toml");
        
        // Skip if this is the same file we started with
        if candidate == project_pyproject_toml {
            dir = dir.parent()?;
            continue;
        }
        
        if std::fs::exists(&candidate).unwrap_or(false) {
            // Check if this pyproject.toml defines a workspace
            if let Ok(content) = std::fs::read_to_string(&candidate) {
                if let Ok(table) = content.parse::<toml::Table>() {
                    // Check for [tool.uv.workspace] section
                    if let Some(toml::Value::Table(tool_table)) = table.get("tool") {
                        if let Some(toml::Value::Table(uv_table)) = tool_table.get("uv") {
                            if uv_table.contains_key("workspace") {
                                return Some(candidate);
                            }
                        }
                    }
                }
            }
        }
        
        // Move up to the parent directory
        dir = dir.parent()?;
    }
}
