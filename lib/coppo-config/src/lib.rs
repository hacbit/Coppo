//! Parse the configuration file.
//! the file name is `Coppo.toml`
//! and it should be in the root directory of the Cpp project.
//!
//! The configuration file should look like this:
//!
//! ```toml
//! [package]
//! name = "my_project"
//! version = "0.1.0"
//! authors = ["Your Name"]
//! description = "This is a simple project."
//! license = "MIT"
//!
//! [dependencies]
//!
//! ```

#![forbid(unsafe_code)]
#![feature(assert_matches)]
#![allow(clippy::should_implement_trait)]

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

/// configuration file name
pub const CONFIG_FILE: &str = "Coppo.toml";

/// Any error that can occur while parsing the configuration file.
type E = Box<dyn std::error::Error>;

/// The configuration of the Cpp project.
///
/// The structure of the configuration file is like follows:
/// # Example
/// ```toml
/// [project]
/// name = "my_project"
/// version = "0.1.0"
/// authors = ["John Doe"]
/// description = "This is a simple project."
/// license = "MIT"
///
/// [dependencies]
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
    pub dependencies: HashMap<String, Dependency>,
}

/// The project configuration.
///
/// It contains the following fields:
/// - `name`: The name of the project.
/// - `version`: The version of the project.
/// - `authors`: The authors of the project.
/// - `description`: The description of the project.
/// - `license`: The license of the project.
/// - `repository`: The repository of the project.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Project {
    /// The name of the project.
    /// It defaults to the name of the directory.
    /// But it's not necessary to be the same as the directory name.
    /// You can name it whatever you want.
    pub name: String,
    /// The version of the project.
    /// The format `x.y.z` is recommended.
    /// It can also be `x.y.z-alpha` or `x.y.z-beta`.
    pub version: String,
    /// The authors of the project.
    /// It should be in the format: `Name <email>` or just `Name`.
    pub authors: Vec<String>,
    /// The description of the project.
    pub description: Option<String>,
    /// The license of the project.
    pub license: Option<String>,
    /// The repository of the project.
    pub repository: Option<String>,
}

/// The dependency configuration.
///
/// It contains the following fields:
/// - `name`: The name of the dependency.
/// - `version`: The version of the dependency.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dependency {
    /// The name of the dependency.
    /// It should be the same as the name of the project.
    pub name: String,
    /// The version of the dependency.
    /// If it is not specified, it should be `*`.
    pub version: String,
}

impl Config {
    /// Parse the configuration file `Coppo.toml` in the root directory of the project.
    pub fn from_file() -> Result<Config, E> {
        let config_file = fs::read_to_string(CONFIG_FILE)?;

        Config::from_str(&config_file)
    }

    /// Parse the configuration file from a string.
    /// It's a packing function for `toml::from_str`.
    ///
    /// # Example
    /// ```rust
    /// use coppo_config::Config;
    /// let config = Config::from_str(r#"
    ///     [project]
    ///     name = "my_project"
    ///     version = "1.0.0-alpha"
    ///     authors = ["My name <my_email>"]
    ///     description = "This is a simple project."
    ///     license = "MIT"
    ///
    ///     [dependencies]
    /// "#).expect("Failed to parse config file.");
    ///
    /// assert_eq!(config.project.name, "my_project");
    /// assert_eq!(config.project.version, "1.0.0-alpha");
    /// assert_eq!(config.project.authors, vec!["My name <my_email>"]);
    /// assert_eq!(config.project.description, Some("This is a simple project.".to_string()));
    /// assert_eq!(config.project.license, Some("MIT".to_string()));
    /// assert_eq!(config.project.repository, None);
    /// ```
    pub fn from_str(config_str: &str) -> Result<Config, E> {
        toml::from_str(config_str).map_err(Into::into)
    }
}


pub mod prelude {
    pub use super::{Config, Dependency, Project, CONFIG_FILE};
    pub use toml;
}


#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_config() -> Result<(), E> {
        let config = Config::from_str(
            r#"
            [project]
            name = "my_project"
            version = "0.1.0"
            authors = ["John Doe <example@123.com>"]
            license = "MIT"

            [dependencies]
            "#,
        )?;

        assert_matches!(
            config,
            Config {
                project: Project {
                    name,
                    version,
                    authors,
                    description,
                    license,
                    repository,
                },
                dependencies,
            } if name == "my_project"
                && version == "0.1.0"
                && authors == vec![
                    "John Doe <example@123.com>".to_string()
                ]
                && description.is_none()
                && license == Some("MIT".to_string())
                && repository.is_none()
                && dependencies.is_empty()
        );

        Ok(())
    }
}
