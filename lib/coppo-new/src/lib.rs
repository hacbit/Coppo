#![forbid(unsafe_code)]

use coppo_addons::prelude::*;

/// The `Coppo new` command options.
#[derive(Debug, Default)]
pub struct CoppoNew {
    /// The path where the project will be created.
    /// If specified `my_project`, then will create a new directory `my_project` in the current directory.
    /// And the project will be created in the `my_project` directory.
    pub path: String,
    /// The name of the project.
    /// If not specified, the name of the project will be same as the name of the directory.
    pub name: Option<String>,
}

pub struct CoppoNewAddon;

impl_addon! {
    CoppoNewAddon,
    name => "new",
    run => |config, matches| {
        println!("Coppo new add-on is running...");
    }
}
