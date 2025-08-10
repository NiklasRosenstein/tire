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

/// Parse the `requires-python` field from a pyproject.toml and extract the minimum Python version.
/// 
/// Examples:
/// - ">=3.9" -> Some("3.9")
/// - ">=3.8,<4.0" -> Some("3.8") 
/// - "~=3.9.0" -> Some("3.9")
/// - ">3.8" -> Some("3.9") (bumped to next minor version)
/// 
/// Returns None if the field is not present or cannot be parsed.
pub fn extract_min_python_version(requires_python: Option<&str>) -> Option<String> {
    let requires_python = requires_python?.trim();
    
    // Split on comma to handle multiple constraints and take the first one
    let first_constraint = requires_python.split(',').next()?.trim();
    
    // Handle >=X.Y format
    if let Some(version_part) = first_constraint.strip_prefix(">=") {
        let version_part = version_part.trim();
        // Extract major.minor from version like "3.9" or "3.9.0"
        let parts: Vec<&str> = version_part.split('.').collect();
        if parts.len() >= 2 {
            return Some(format!("{}.{}", parts[0], parts[1]));
        }
    }
    
    // Handle ~=X.Y format (compatible release)
    if let Some(version_part) = first_constraint.strip_prefix("~=") {
        let version_part = version_part.trim();
        let parts: Vec<&str> = version_part.split('.').collect();
        if parts.len() >= 2 {
            return Some(format!("{}.{}", parts[0], parts[1]));
        }
    }
    
    // Handle >X.Y format (greater than - bump to next minor version)
    if let Some(version_part) = first_constraint.strip_prefix(">") {
        let version_part = version_part.trim();
        let parts: Vec<&str> = version_part.split('.').collect();
        if parts.len() >= 2 {
            if let (Ok(major), Ok(minor)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                return Some(format!("{}.{}", major, minor + 1));
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_min_python_version() {
        // Test >=X.Y format
        assert_eq!(extract_min_python_version(Some(">=3.9")), Some("3.9".to_string()));
        assert_eq!(extract_min_python_version(Some(">=3.8")), Some("3.8".to_string()));
        assert_eq!(extract_min_python_version(Some(">=3.10")), Some("3.10".to_string()));
        
        // Test with patch version
        assert_eq!(extract_min_python_version(Some(">=3.9.0")), Some("3.9".to_string()));
        
        // Test with spaces
        assert_eq!(extract_min_python_version(Some(">= 3.9")), Some("3.9".to_string()));
        assert_eq!(extract_min_python_version(Some(">=  3.8  ")), Some("3.8".to_string()));
        
        // Test multiple constraints (should use first one)
        assert_eq!(extract_min_python_version(Some(">=3.8,<4.0")), Some("3.8".to_string()));
        assert_eq!(extract_min_python_version(Some(">=3.9, !=3.9.7")), Some("3.9".to_string()));
        
        // Test ~= format (compatible release)
        assert_eq!(extract_min_python_version(Some("~=3.9.0")), Some("3.9".to_string()));
        assert_eq!(extract_min_python_version(Some("~=3.8")), Some("3.8".to_string()));
        
        // Test > format (should bump to next minor version)
        assert_eq!(extract_min_python_version(Some(">3.8")), Some("3.9".to_string()));
        assert_eq!(extract_min_python_version(Some(">3.7")), Some("3.8".to_string()));
        
        // Test edge cases
        assert_eq!(extract_min_python_version(None), None);
        assert_eq!(extract_min_python_version(Some("")), None);
        assert_eq!(extract_min_python_version(Some("invalid")), None);
        assert_eq!(extract_min_python_version(Some("==3.9")), None); // Not supported format
    }
}
