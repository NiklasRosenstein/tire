use std::fs;
use tire::utils::find_workspace_root;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_find_workspace_root_with_workspace() {
        // Create a temporary directory structure
        let temp_dir = env::temp_dir().join("tire_test_workspace");
        let _ = fs::remove_dir_all(&temp_dir); // Clean up any previous test
        fs::create_dir_all(&temp_dir).unwrap();

        // Create workspace root pyproject.toml
        let workspace_root_dir = temp_dir.clone();
        let workspace_pyproject = workspace_root_dir.join("pyproject.toml");
        fs::write(&workspace_pyproject, r#"
[tool.uv.workspace]
members = ["packages/*"]
"#).unwrap();

        // Create a package directory
        let package_dir = temp_dir.join("packages").join("my-package");
        fs::create_dir_all(&package_dir).unwrap();

        // Create package pyproject.toml
        let package_pyproject = package_dir.join("pyproject.toml");
        fs::write(&package_pyproject, r#"
[project]
name = "my-package"
"#).unwrap();

        // Test that workspace root is found
        let result = find_workspace_root(&package_pyproject);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), workspace_pyproject);

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_find_workspace_root_without_workspace() {
        // Create a temporary directory structure
        let temp_dir = env::temp_dir().join("tire_test_no_workspace");
        let _ = fs::remove_dir_all(&temp_dir); // Clean up any previous test
        fs::create_dir_all(&temp_dir).unwrap();

        // Create a standalone pyproject.toml (no workspace)
        let standalone_pyproject = temp_dir.join("pyproject.toml");
        fs::write(&standalone_pyproject, r#"
[project]
name = "standalone-package"
"#).unwrap();

        // Test that no workspace root is found
        let result = find_workspace_root(&standalone_pyproject);
        assert!(result.is_none());

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_find_workspace_root_nested_structure() {
        // Create a temporary directory structure
        let temp_dir = env::temp_dir().join("tire_test_nested");
        let _ = fs::remove_dir_all(&temp_dir); // Clean up any previous test
        fs::create_dir_all(&temp_dir).unwrap();

        // Create workspace root pyproject.toml
        let workspace_root_dir = temp_dir.clone();
        let workspace_pyproject = workspace_root_dir.join("pyproject.toml");
        fs::write(&workspace_pyproject, r#"
[tool.uv.workspace]
members = ["apps/*", "libs/*"]
"#).unwrap();

        // Create a deeply nested package
        let deep_package_dir = temp_dir.join("apps").join("web").join("backend");
        fs::create_dir_all(&deep_package_dir).unwrap();

        // Create package pyproject.toml
        let package_pyproject = deep_package_dir.join("pyproject.toml");
        fs::write(&package_pyproject, r#"
[project]
name = "backend"
"#).unwrap();

        // Test that workspace root is found even from deeply nested package
        let result = find_workspace_root(&package_pyproject);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), workspace_pyproject);

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }
}