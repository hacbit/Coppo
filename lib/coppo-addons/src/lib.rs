//! Provides an interface for Coppo add-ons.
//! You can create a new add-on by implementing the `Addon` trait.
//! And then add the add-on to the `Coppo-CLI` to extend Coppo's functionality.

#![forbid(unsafe_code)]

use coppo_config::Config;

/// The result for add-ons run.
pub type AddonResult = Result<(), Box<dyn std::error::Error>>;

/// The `Addon` trait provides an interface for Coppo add-ons.
/// You can create a new add-on by implementing the `Addon` trait.
///
/// The `Addon` trait has four methods:
/// - `name`: The name of the add-on.(required)
/// - `version`: The version of the add-on.
/// - `description`: The description of the add-on.
/// - `run`: The entry point of the add-on.(required)
///
/// # Example
///
/// ```rust
/// use coppo_addons::prelude::*;
///
/// struct MyAddon;
///
/// impl Addon for MyAddon {
///     fn name(&self) -> String {
///         "my-addon".to_string()
///     }
///
///     fn run(&self, _config: &mut Config) -> AddonResult {
///         println!("My add-on is running...");
///         Ok(())
///     }
/// }
/// ```
pub trait Addon {
    /// The name of the add-on.
    fn name(&self) -> String;
    /// The version of the add-on.
    /// The version of the add-on will get from the `CARGO_PKG_VERSION` environment variable.
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    /// The description of the add-on.
    /// If not specified, the description will be `None`.
    fn description(&self) -> Option<String> {
        None
    }
    /// The entry point of the add-on.
    fn run(&self, config: &mut Config) -> AddonResult;
}

/// The prelude module for Coppo add-ons.
/// It provides `Addon` trait and `AddonResult` type.
/// And `coppo-config`'s `Config` struct also included in the prelude.
pub mod prelude {
    pub use crate::{Addon, AddonResult};
    pub use coppo_config::Config;
}
