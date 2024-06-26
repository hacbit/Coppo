//! The `coppo-build` crate is a Coppo addon that allows you to compile the current project.
//!
//! # Usage
//! ```sh
//! coppo build [options]
//! ```

#![forbid(unsafe_code)]

use coppo_addons::prelude::*;
use coppo_logger::prelude::*;

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
        error!("Unimplemented!");
    }
}
