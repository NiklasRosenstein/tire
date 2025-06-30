use std::fs;
use tire::profile::Profile;
use toml::Value;
use toml::value::Table;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        // Load the default profile
        let profile_content = fs::read_to_string("profiles/default.toml").unwrap();
        let profile_table: Table = profile_content.parse().unwrap();

        // Create a simple pyproject.toml
        let pyproject_content = r#"
            [tool.mypy]
            strict = false
            warn_unused_ignores = false

            [tool.ruff]
            line-length = 80

            [tool.ruff.lint]
            select = ["E4", "D"]
        "#;

        let pyproject_table: Table = pyproject_content.parse().unwrap();

        // Create a Profile instance
        let profile = Profile {
            name: "test".to_owned(),
            root: profile_table,
        };

        // Merge the profiles
        let merged = profile.merge(&pyproject_table);

        // Check that the merged profile has the expected values
        assert_eq!(
            merged.get("tool").unwrap()["mypy"]["strict"],
            Value::Boolean(false)
        );
        assert_eq!(
            merged.get("tool").unwrap()["mypy"]["warn_unused_ignores"],
            Value::Boolean(false)
        );
        assert_eq!(
            merged.get("tool").unwrap()["ruff"]["line-length"],
            Value::Integer(80)
        );
        assert_eq!(
            merged.get("tool").unwrap()["ruff"]["lint"]["select"],
            Value::Array(vec![
                Value::String("E4".to_string()),
                Value::String("D".to_string()),
            ])
        );

        // Check that original values are preserved when not overridden
        assert_eq!(
            merged.get("tool").unwrap()["mypy"]["warn_no_return"],
            Value::Boolean(true)
        );
        assert_eq!(
            merged.get("tool").unwrap()["mypy"]["python_version"],
            Value::String("${TIRE_MIN_PYTHON_VERSION}".to_string())
        );
    }
}
