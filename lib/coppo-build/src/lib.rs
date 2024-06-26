//! The `coppo-build` crate is a Coppo addon that allows you to compile the current project.
//!
//! # Usage
//! ```sh
//! coppo build [options]
//! ```

#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;
use std::process;

use coppo_addons::prelude::*;
use coppo_logger::prelude::*;

/// The compile output will be stored in the `target` directory.
pub const COMPILE_OUTPUT: &str = "target";

/// The default compile backend.
/// It defaults to `clang++` with `llvm`.
pub const COMPILER: &str = "clang++";

/// The `Coppo build` add-on.
/// Compile the current project.
/// It will compile the current project.
/// The project must have a `Coppo.toml` file.
/// The `Coppo.toml` file must have the following fields:
/// - `name`: The name of the project.
/// - `version`: The version of the project.
pub struct CoppoBuildAddon;

impl_addon! {
    CoppoBuildAddon,
    name => "build",
    description => "Compile the current project",
    run => |config, matches| {
        info!("Building the project...");

        // Check if the project has a `Coppo.toml` file.
        if !Config::exists() {
            return Err("The project does not have a `Coppo.toml` file.".into());
        }

        // Check if the configuration have the project name and version.
        if config.is_empty() {
            return Err("The project name and version is needed".into());
        }

        // Check if the `src/main.cpp` file exists.
        if !Path::new("src/main.cpp").exists() {
            return Err("The `src/main.cpp` file does not exist.".into());
        }

        // Create the `target` directory if it does not exist.
        if !Path::new(COMPILE_OUTPUT).exists() {
            fs::create_dir(COMPILE_OUTPUT)?;
        }

        // Compile the project,
        // And store the output in the `target` directory.
        let output = process::Command::new(COMPILER)
            .args(&["src/main.cpp", "-o", &format!("{}/{}", COMPILE_OUTPUT, config.project.name)])
            .output()?;

        if output.status.success() {
            success!("The project has been built successfully.");
        } else {
            error!("The project failed to build.");
            error!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
