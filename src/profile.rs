use std::{
    error::Error,
    path::{Path, PathBuf},
};

use tempfile::TempDir;
use toml::value::*;

use crate::utils::find_pyproject_toml;

/// This contains the accepted `[tool.*]` sections that are actually supported by Tire. Any
/// additional tool sections are not allowed.
const ACCEPTED_TOOLS: [&str; 2] = ["mypy", "ruff"];

/// Represents a Tire profile configuration.
#[derive(Debug)]
pub struct Profile {
    pub root: Table,
}

impl Profile {
    /// Load a profile from the given TOML-encoded file.
    pub fn load_file(toml_file: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(Self::load_string(std::fs::read_to_string(toml_file)?)?)
    }

    /// Load a profile from the given TOML-encoded string.
    pub fn load_string(toml_text: String) -> Result<Self, toml::de::Error> {
        Ok(Profile {
            root: toml_text.parse::<Table>()?,
        })
    }

    /// Validate the profile.
    pub fn validate(&self) -> Result<(), String> {
        let disallowed_root_keys: Vec<&String> = self
            .root
            .keys()
            .filter(|k| (*k).ne(&"tool".to_owned()))
            .collect();
        if !disallowed_root_keys.is_empty() {
            return Err(format!(
                "Disallowed top-level keys found: {disallowed_root_keys:?}"
            ));
        }

        let tool_value = self.root.get(&"tool".to_owned());
        match tool_value {
            Some(Value::Table(table)) => {
                let disallowed_tool_keys: Vec<&String> = table
                    .keys()
                    .filter(|k| !ACCEPTED_TOOLS.contains(&k.as_str()))
                    .collect();
                if !disallowed_tool_keys.is_empty() {
                    return Err(format!(
                        "Disallowed [tool.*] keys found: {disallowed_tool_keys:?}"
                    ));
                }
            }
            _ => {
                return Err("Missing or invalid tool top-level key".to_owned());
            }
        }

        Ok(())
    }

    /// Merge the profile with a `pyproject.toml`, giving precedence to the values defined in
    /// the `pyproject.toml`.
    pub fn merge(&self, pyproject_toml: &Table) -> Result<Table, String> {
        // Helper function to recursively merge tables
        fn merge_tables(base: &Table, override_table: &Table) -> Result<Table, String> {
            let mut result = Table::new();

            // Process all keys from both tables
            let all_keys: Vec<_> = base.keys().chain(override_table.keys()).collect();

            for key in all_keys {
                // Get values from both tables
                let base_value = base.get(key);
                let override_value = override_table.get(key);

                // Handle different value combinations
                match (base_value, override_value) {
                    // If both are tables, recursively merge them
                    (Some(Value::Table(base_table)), Some(Value::Table(override_table))) => {
                        match merge_tables(base_table, override_table) {
                            Ok(merged_table) => {
                                result.insert(key.clone(), Value::Table(merged_table));
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    // If both are arrays, use the override array
                    (Some(Value::Array(_)), Some(Value::Array(override_array))) => {
                        result.insert(key.clone(), Value::Array(override_array.clone()));
                    }
                    // If only override has a value, use it
                    (_, Some(value)) => {
                        result.insert(key.clone(), value.clone());
                    }
                    // If only base has a value, use it
                    (Some(value), _) => {
                        result.insert(key.clone(), value.clone());
                    }
                    // If neither has a value, skip it (shouldn't happen)
                    (_, _) => {}
                }
            }

            Ok(result)
        }

        // Merge the root table with the pyproject_toml
        merge_tables(&self.root, pyproject_toml)
    }
}

/// Loads the default profile and merges it with the current project's `pyproject.toml`, returning
/// the final `pyproject.toml`.
pub fn materialize_pyproject_toml() -> Table {
    // TODO: Check if the pyproject_toml contains `tool.tire.inflated=true`. If yes, we don't
    //       actually want to merge the profile in.

    // Load the project's pyproject.toml
    let pyproject_toml_file = find_pyproject_toml().unwrap();
    let pyproject_toml = std::fs::read_to_string(pyproject_toml_file.clone()).unwrap();
    let pyproject_toml = pyproject_toml.parse::<toml::Table>().unwrap();

    // Load the profile for the project
    let profile = Profile::load_file(Path::new("../profiles/default.toml")).unwrap();
    profile.validate().unwrap();

    // Merge the profile and the pyproject.toml
    profile.merge(&pyproject_toml).unwrap()
}

/// Same as [`material_pyproject_toml`], but writes it to a temporary file.
pub fn materialize_pyproject_toml_to_tmp() -> TemporaryMaterializedPyprojectToml {
    let merged_toml = toml::to_string(&materialize_pyproject_toml()).unwrap();
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("pyproject.toml");
    std::fs::write(&path, merged_toml.as_bytes()).unwrap();
    TemporaryMaterializedPyprojectToml { _dir: dir, path }
}

pub struct TemporaryMaterializedPyprojectToml {
    _dir: TempDir,
    pub path: PathBuf,
}
