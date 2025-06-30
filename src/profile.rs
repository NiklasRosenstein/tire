//! This module implements everything related to Tire profiles.
//!
//! A profile is a partial `pyproject.toml` that contains all relevant `[tool.*]` configuration
//! values for the tools that Tire supports. This is strictly limited to the supported tools, and
//! any `[tool.<KEY>]` that is not one of the [known tools][KNOWN_TOOLS] will cause trigger a
//! warning when loading a profile and be ignored when applied to the `pyproject.toml`.
//!
//! The [`default`][DEFAULT_PROFILE] is embedded into the Tire binary itself. However, other
//! profiles can be used by referring to them via a URL that returns the profile in TOML format.
//! HTTPS and HTTP URLs are supported.

use std::path::{Path, PathBuf};

use toml::value::*;

use crate::utils::find_pyproject_toml;

/// The default profile configuration that comes with Tire.
const DEFAULT_PROFILE: &str = include_str!("../profiles/default.toml");

/// This contains the names of all well-known `[tool.*]` sections for tools that Tire supports.
const KNOWN_TOOLS: [&str; 3] = ["mypy", "pytest", "ruff"];

/// Checks if the given string is contained in one of the [KNOWN_TOOLS].
pub fn is_known_tool<S: Into<String>>(tool: S) -> bool {
    let s: String = tool.into();
    let s = s.as_str();
    KNOWN_TOOLS.iter().any(|x| s.eq(*x))
}

/// Error type for loading a profile.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Ser(#[from] toml::ser::Error),

    #[error(transparent)]
    De(#[from] toml::de::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("invalid profile {0:?}")]
    InvalidProfile(String),
}

/// Represents a deserialized Tire profile.
#[derive(Debug)]
pub struct Profile {
    /// The name of the profile. `default` if [DEFAULT_PROFILE] was used, otherwise usually a
    /// URL pointing to the profile.
    pub name: String,

    /// The deserialized TOML file.
    pub root: Table,
}

impl Profile {
    /// Main entrypoint for loading a profile.
    ///
    /// If [None] or [Some] with value `"default"` is specified, the [DEFAULT_PROFILE] is loaded.
    /// Otherwise, a URL is expected and it is loaded from the network.
    ///
    /// TODO: Support caching already fetched URLs on disk.
    pub fn load(name: Option<String>) -> Result<Self, Error> {
        match name {
            None => Self::load_string("default".to_owned(), DEFAULT_PROFILE.to_owned()),
            Some(name) => {
                if name == *"default" {
                    Self::load_string("default".to_owned(), DEFAULT_PROFILE.to_owned())
                } else if name.starts_with("http://") || name.starts_with("https://") {
                    Self::load_url(name)
                } else {
                    Err(Error::InvalidProfile(name))
                }
            }
        }
    }

    /// Load a profile from the given URL.
    pub fn load_url(url: String) -> Result<Self, Error> {
        let content = reqwest::blocking::get(url.clone())?.text()?;
        Self::load_string(url, content)
    }

    /// Load a profile from the given TOML-encoded file.
    pub fn load_file(toml_file: &Path) -> Result<Self, Error> {
        Self::load_string(
            format!("file://{}", toml_file.to_string_lossy()),
            std::fs::read_to_string(toml_file)?,
        )
    }

    /// Load a profile from the given TOML-encoded string.
    pub fn load_string(name: String, toml_text: String) -> Result<Self, Error> {
        Ok(Profile {
            name,
            root: toml_text.parse::<Table>()?,
        })
    }

    /// Validate the profile, emitting warning logs if the contents look off, removing any
    /// keys from the profile that are unsupported.
    pub fn validate(&mut self) {
        // Check that there is only a `[tool]` section in the config.
        let unexpected_keys: Vec<String> = self
            .root
            .keys()
            .filter(|k| (*k).ne("tool"))
            .map(String::clone)
            .collect();
        if !unexpected_keys.is_empty() {
            log::warn!(
                "Unexpected top-level `[*]` keys found in profile `{}`: {}",
                self.name,
                unexpected_keys.join(", ")
            );
            unexpected_keys.iter().for_each(|k| {
                self.root.remove(k);
            });
        }

        // Check that the `[tool]` section contains no unexpected keys.
        match self.root.get(&"tool".to_owned()) {
            Some(Value::Table(table)) => {
                let unexpected_keys: Vec<String> = table
                    .keys()
                    .filter(|k| !is_known_tool(*k))
                    .map(String::clone)
                    .collect();
                if !unexpected_keys.is_empty() {
                    log::warn!(
                        "Unexpected `[tool.*]` keys found in profile `{}`: {}",
                        self.name,
                        unexpected_keys.join(", ")
                    );
                    unexpected_keys.iter().for_each(|k| {
                        self.root.remove(k);
                    });
                }
            }
            _ => {
                log::warn!("The `tool` key in profile `{}` is not a table.", self.name);
                self.root.remove("tool");
            }
        }
    }

    /// Merge the profile with a `pyproject.toml`, giving precedence to the values defined in
    /// the `pyproject.toml`.
    pub fn merge(&self, pyproject_toml: &Table) -> Table {
        // Helper function to recursively merge tables
        fn merge_tables(base: &Table, override_table: &Table) -> Table {
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
                        result.insert(
                            key.clone(),
                            Value::Table(merge_tables(base_table, override_table)),
                        );
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

            result
        }

        // Merge the root table with the pyproject_toml
        merge_tables(&self.root, pyproject_toml)
    }

    /// Writes the updated `pyproject.toml` to a `.tire/pyproject.toml` file in the project
    /// root directory of the given working directory. If the project has no `pyproject.toml`,
    /// the current working directory is assumed to be the project root.
    ///
    /// Returns the path to the `.tire/pyproject.toml` file.
    ///
    /// TODO: Support Uv workspaces (see https://github.com/NiklasRosenstein/tire/issues/2)
    pub fn materialize(&self, cwd: Option<PathBuf>) -> Result<PathBuf, Error> {
        let cwd = cwd.ok_or("").or_else(|_| std::env::current_dir())?;

        let pyproject_toml_file = find_pyproject_toml(Some(cwd));

        // The project root is where we place the `.tire/pyproject.toml` file.
        let project_root = if let Some(file) = pyproject_toml_file.clone() {
            file.parent().unwrap().to_path_buf()
        } else {
            std::env::current_dir()?
        };
        let out_file = project_root.join(".tire").join("pyproject.toml");
        std::fs::create_dir_all(out_file.parent().unwrap())?;

        // Load the project's configuration.
        let pyproject_toml = if let Some(file) = pyproject_toml_file {
            let content = std::fs::read_to_string(file).unwrap();
            content.parse::<toml::Table>()?
        } else {
            Table::new()
        };

        // Merge the configuration and write it to the output file.
        std::fs::write(
            out_file.clone(),
            toml::to_string(&self.merge(&pyproject_toml))?,
        )?;

        Ok(out_file)
    }
}
