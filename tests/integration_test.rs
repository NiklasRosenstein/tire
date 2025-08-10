use std::fs;
use tire::profile::Profile;

#[test]
fn test_materialize_with_python_version() {
    // Create a temporary directory for the test
    let temp_dir = std::env::temp_dir().join("tire_test_project");
    fs::create_dir_all(&temp_dir).unwrap();

    // Create a test pyproject.toml with requires-python
    let pyproject_content = r#"
[project]
name = "test-project"
version = "0.1.0"
requires-python = ">=3.9"

[tool.mypy]
strict = false
"#;
    
    fs::write(temp_dir.join("pyproject.toml"), pyproject_content).unwrap();

    // Load the default profile and materialize it
    let profile = Profile::load(None).unwrap();
    let materialized_path = profile.materialize(Some(temp_dir.clone())).unwrap();

    // Read the materialized content
    let content = fs::read_to_string(&materialized_path).unwrap();
    println!("Materialized content:\n{}", content);

    // Parse the materialized content as TOML
    let materialized_toml: toml::Table = content.parse().unwrap();

    // Check that the python_version was substituted correctly
    let python_version = materialized_toml
        .get("tool").unwrap()
        .get("mypy").unwrap()
        .get("python_version").unwrap()
        .as_str().unwrap();

    assert_eq!(python_version, "3.9");

    // Cleanup
    fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_materialize_with_different_python_version() {
    // Create a temporary directory for the test
    let temp_dir = std::env::temp_dir().join("tire_test_project_38");
    fs::create_dir_all(&temp_dir).unwrap();

    // Create a test pyproject.toml with different requires-python
    let pyproject_content = r#"
[project]
name = "test-project"
version = "0.1.0"
requires-python = ">=3.8,<4.0"

[tool.mypy]
strict = false
"#;
    
    fs::write(temp_dir.join("pyproject.toml"), pyproject_content).unwrap();

    // Load the default profile and materialize it
    let profile = Profile::load(None).unwrap();
    let materialized_path = profile.materialize(Some(temp_dir.clone())).unwrap();

    // Read the materialized content
    let content = fs::read_to_string(&materialized_path).unwrap();

    // Parse the materialized content as TOML
    let materialized_toml: toml::Table = content.parse().unwrap();

    // Check that the python_version was substituted correctly
    let python_version = materialized_toml
        .get("tool").unwrap()
        .get("mypy").unwrap()
        .get("python_version").unwrap()
        .as_str().unwrap();

    assert_eq!(python_version, "3.8");

    // Cleanup
    fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_materialize_without_requires_python() {
    // Create a temporary directory for the test
    let temp_dir = std::env::temp_dir().join("tire_test_project_no_python");
    fs::create_dir_all(&temp_dir).unwrap();

    // Create a test pyproject.toml without requires-python
    let pyproject_content = r#"
[project]
name = "test-project"
version = "0.1.0"

[tool.mypy]
strict = false
"#;
    
    fs::write(temp_dir.join("pyproject.toml"), pyproject_content).unwrap();

    // Load the default profile and materialize it
    let profile = Profile::load(None).unwrap();
    let materialized_path = profile.materialize(Some(temp_dir.clone())).unwrap();

    // Read the materialized content
    let content = fs::read_to_string(&materialized_path).unwrap();

    // Parse the materialized content as TOML
    let materialized_toml: toml::Table = content.parse().unwrap();

    // Check that the python_version defaults to 3.8
    let python_version = materialized_toml
        .get("tool").unwrap()
        .get("mypy").unwrap()
        .get("python_version").unwrap()
        .as_str().unwrap();

    assert_eq!(python_version, "3.8");

    // Cleanup
    fs::remove_dir_all(&temp_dir).unwrap();
}