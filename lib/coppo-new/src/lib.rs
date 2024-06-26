//! The `Coppo new` add-on.
//! This add-on is used to create a new Cpp project.
//!
//! Usage:
//! ```sh
//! coppo new <path> [options]
//! ```

#![forbid(unsafe_code)]

use std::{fs, path::PathBuf};

use coppo_addons::prelude::*;
use coppo_config::prelude::*;
use coppo_logger::prelude::*;

/// The `Coppo new` command options.
#[derive(Debug, Default)]
pub struct CoppoNew {
    /// The path where the project will be created.
    /// If specified `my_project`, then will create a new directory `my_project` in the current directory.
    /// And the project will be created in the `my_project` directory.
    pub path: PathBuf,
    /// The name of the project.
    /// If not specified, the name of the project will be same as the name of the directory.
    pub name: String,
}

/// The `Coppo new` add-on.
/// Create a new project.
/// The project will be created in the specified directory.
/// If the name of the project is not specified, the name of the project will be same as the name of the directory.
/// It will create the following files:
/// - src/main.cpp
/// - Coppo.toml
/// - .gitignore
pub struct CoppoNewAddon;

impl_addon! {
    CoppoNewAddon,
    name => "new",
    description => "Create a new project",
    args => [
        arg!(["path"] "The path where the project will be created")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        arg!(-n --name "The name of the project")
            .action(ArgAction::Set)
            .value_parser(value_parser!(String)),
    ],
    run => |config, matches| {
        let mut new = CoppoNew::default();
        if let Some(path) = matches.get_one::<PathBuf>("path") {
            new.path = path.to_owned();
        }
        if let Some(name) = matches.get_one::<String>("name") {
            new.name = name.to_owned();
        } else {
            // If the name is not specified, get the name of the directory.
            new.name = new
                .path
                .file_name()
                .ok_or("Failed to get the name of the directory.")?
                .to_str()
                .ok_or("Failed to convert the name of the directory to a string.")?
                .to_owned();
        }

        config.project.name = new.name.clone();
        config.project.version = "0.1.0".to_owned();

        // Create the project directory.
        fs::create_dir_all(&new.path)?;
        fs::create_dir(new.path.join("src"))?;

        // Create the src/main.cpp file.
        fs::write(new.path.join("src/main.cpp"), MAIN_CPP)?;

        // Create the configuration file.
        let toml = toml::to_string(&config)?;
        fs::write(new.path.join(CONFIG_FILE), toml)?;

        // Create the gitignore file.
        fs::write(new.path.join(".gitignore"), GITIGNORE)?;

        // Print the success message.
        success!("Created a new project at {}", new.path.canonicalize()?.display());
    }
}

const MAIN_CPP: &str = r#"#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
"#;

const GITIGNORE: &str = r#"/target
"#;
