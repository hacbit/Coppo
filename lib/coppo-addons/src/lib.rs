//! Provides an interface for Coppo add-ons.
//! You can create a new add-on by implementing the `Addon` trait.
//! And then add the add-on to the `Coppo-CLI` to extend Coppo's functionality.

#![forbid(unsafe_code)]

use clap::{Arg, ArgMatches};
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
///     fn name(&self) -> &'static str {
///         "my-addon"
///     }
///
///     fn version(&self) -> &'static str {
///         env!("CARGO_PKG_VERSION")
///     }
///
///     fn run(&self, _config: &mut Config, _matches: &ArgMatches) -> AddonResult {
///         println!("My add-on is running...");
///         Ok(())
///     }
/// }
/// ```
pub trait Addon {
    /// The name of the add-on.
    fn name(&self) -> &'static str;

    /// The version of the add-on.
    /// Get version from the `CARGO_PKG_VERSION` environment variable is recommended.
    fn version(&self) -> &'static str;

    /// The description of the add-on.
    /// If not specified, the description will be `None`.
    fn description(&self) -> Option<&'static str> {
        None
    }

    /// The arguments of the add-on.
    fn args(&self) -> Vec<Arg> {
        vec![]
    }

    /// The entry point of the add-on.
    fn run(&self, config: &mut Config, matches: &ArgMatches) -> AddonResult;
}

/// The macro for implementing the `Addon` trait.
/// You can use this macro to implement the `Addon` trait for your add-on.
/// The macro requires the following fields:
/// - `name`: The name of the add-on.
/// - `run`: The entry point of the add-on.
///
/// And the following fields are optional:
/// - `version`: The version of the add-on.
/// - `description`: The description of the add-on.
/// - `args`: The arguments of the add-on.
///
/// You can not need to specify the `version` field,
/// if not specified, it will get the version from the `CARGO_PKG_VERSION` environment variable
/// in the crate where the add-on is implemented.
///
/// # Example
/// ```rust
/// use coppo_addons::prelude::*;
///
/// struct MyAddon;
///
/// impl_addon! {
///     MyAddon,
///     name => "my-addon",
///     run => |config, matches| {
///         println!("My add-on is running...");
///     }
/// }
/// ```
/// You can also specify the `version` field.
/// ```rust
/// use coppo_addons::prelude::*;
/// struct MyAddonWithVersion;
///
/// impl_addon! {
///     MyAddonWithVersion,
///     name => "my-addon-with-version",
///     version => "0.1.0",
///     run => |config, matches| {
///         println!("My add-on with version is running...");
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_addon {
    (
        $addon:ty,
        name => $name:expr,
        $(args => [$($args:expr),*$(,)?],)?
        run => |$config:ident, $matches:ident| $run:block$(,)?
    ) => {
        impl Addon for $addon {
            fn name(&self) -> &'static str {
                $name
            }

            fn version(&self) -> &'static str {
                env!("CARGO_PKG_VERSION")
            }

            fn description(&self) -> Option<&'static str> {
                Some(env!("CARGO_PKG_DESCRIPTION"))
            }

            $(fn args(&self) -> Vec<Arg> {
                vec![$($args),*]
            })?

            fn run(&self, config: &mut Config, matches: &ArgMatches) -> AddonResult {
                $run(config, matches);
                Ok(())
            }
        }
    };
    (
        $addon:ty,
        name => $name:expr,
        version => $version:expr,
        $(args => [$($args:expr),*$(,)?],)?
        run => |$config:ident, $matches:ident| $run:block$(,)?
    ) => {
        impl Addon for $addon {
            fn name(&self) -> &'static str {
                $name
            }

            fn version(&self) -> &'static str {
                $version
            }

            fn description(&self) -> Option<&'static str> {
                Some(env!("CARGO_PKG_DESCRIPTION"))
            }

            $(fn args(&self) -> Vec<Arg> {
                vec![$($args),*]
            })?

            fn run(&self, config: &mut Config, matches: &ArgMatches) -> AddonResult {
                $run(config, matches);
                Ok(())
            }
        }
    };
    (
        $addon:ty,
        name => $name:expr,
        $(version => $version:expr,)?
        description => $description:expr,
        $(args => [$($args:expr),*$(,)?],)?
        run => |$config:ident, $matches:ident| $run:block$(,)?
    ) => {
        impl Addon for $addon {
            fn name(&self) -> &'static str {
                $name
            }

            fn version(&self) -> &'static str {
                env!("CARGO_PKG_VERSION")
            }

            fn description(&self) -> Option<&'static str> {
                Some($description)
            }

            $(fn args(&self) -> Vec<Arg> {
                vec![$($args),*]
            })?

            fn run(&self, $config: &mut Config, $matches: &ArgMatches) -> AddonResult {
                $run($config, $matches);
                Ok(())
            }
        }
    };
    (
        $addon:ty,
        name => $name:expr,
        version => $version:expr,
        description => $description:expr,
        $(args => [$($args:expr),*$(,)?],)?
        run => |$config:ident, $matches:ident| $run:block$(,)?
    ) => {
        impl Addon for $addon {
            fn name(&self) -> &'static str {
                $name
            }

            fn version(&self) -> &'static str {
                $version
            }

            fn description(&self) -> Option<&'static str> {
                Some($description)
            }

            $(fn args(&self) -> Vec<Arg> {
                vec![$($args),*]
            })?

            fn run(&self, config: &mut Config, matches: &ArgMatches) -> AddonResult {
                $run(config, matches);
                Ok(())
            }
        }
    };
}

/// The prelude module for Coppo add-ons.
/// It provides `Addon` trait, `AddonResult` type and `impl_addon` macro.
/// `coppo-config`'s `Config` struct also included in the prelude.
/// It also includes some clap's re-exports.
pub mod prelude {
    pub use crate::{impl_addon, Addon, AddonResult};
    pub use clap::{arg, command, value_parser, Arg, ArgAction, ArgMatches, Command};
    pub use coppo_config::Config;
}
