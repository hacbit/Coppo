//! A CLI tool for Coppo.
//! It will unite all the add-ons which are the subcommands of the main command.
//! And it will run the add-on which is specified by the user.

#![forbid(unsafe_code)]
#![feature(type_alias_impl_trait)]
#![allow(clippy::new_without_default)]

pub use coppo_addons::prelude::*;

/// The packings of the add-ons.
pub type Addons = Vec<Box<dyn Addon>>;

/// The `CoppoCli` struct provides an interface for Coppo CLI.
/// You can create a new Coppo CLI by calling the `new` method.
/// And then add the add-ons to the Coppo CLI by calling the `add_addon` or `add_addons` method.
pub struct CoppoCli {
    addons: Addons,
    command: Command,
    quiet: bool,
}

/// The `CoppoCli` implementation.
impl CoppoCli {
    /// Create a new `CoppoCli`.
    /// But `Default` is not implemented for `CoppoCli`.
    pub fn new(command: Command) -> Self {
        Self {
            addons: vec![],
            command,
            quiet: false,
        }
    }

    /// Add an add-on to the `CoppoCli`.
    /// # Example
    /// ```no_run
    /// use coppo_cli::CoppoCli;
    /// use coppo_addons::prelude::*;
    /// use your_addon::YourAddon;
    ///
    /// CoppoCli::new()
    ///     .add_addon(YourAddon)
    ///     .run(command!());
    /// ```
    ///
    pub fn add_addon<A: Addon + 'static>(&mut self, addon: A) -> &mut Self {
        self.addons.push(Box::new(addon));
        self
    }

    /// Add add-ons to the `CoppoCli`.
    /// Using the `addons!` macro to add multiple add-ons is recommended.
    /// # Example
    /// ```no_run
    /// use coppo_cli::CoppoCli;
    /// use coppo_addons::prelude::*;
    /// use your_addon::YourAddon;
    /// use other_addon::OtherAddon;
    ///
    /// CoppoCli::new()
    ///     .add_addons(addons![YourAddon, OtherAddon])
    ///     .run(command!());
    /// ```
    ///
    pub fn add_addons(&mut self, addons: Addons) -> &mut Self {
        self.addons.extend(addons);
        self
    }

    /// Run the `CoppoCli`.
    /// The `run` method will run the add-on which is specified by the user.
    /// the `command` arg is the main command of the CLI.
    /// you can use the `command!` macro to create the main command.
    /// # Example
    /// ```rust
    /// use coppo_cli::CoppoCli;
    /// use coppo_addons::prelude::*;
    ///
    /// CoppoCli::new().run(command!());
    /// ```
    ///
    pub fn run(&mut self) {
        self.command = self
            .command
            .clone()
            .after_help("See 'coppo help <command>' for more information on a specific command.")
            .subcommands(self.addons.iter().map(|addon| {
                Command::new(addon.name())
                    .version(addon.version())
                    .args(addon.args())
                    .about(addon.description().unwrap_or(""))
            }));

        let matches = self.command.clone().get_matches();
        let mut config = Config::from_file().unwrap_or_default();

        if let Some((name, matches)) = matches.subcommand() {
            for addon in self.addons.iter() {
                if name == addon.name() {
                    if let Err(e) = addon.run(&mut config, matches) {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        } else {
            // No subcommand was used.
            // Print the help message.
            if let Err(e) = self.command.print_help() {
                eprintln!("Error: {}", e);
            }
            println!();
        }
    }
}

/// The `addons!` macro is used to add multiple add-ons to the `CoppoCli`.
/// You can use this macro like `addons![Addon1, Addon2]`.
#[macro_export]
macro_rules! addons {
    (
        $(
            $addon:expr
        ),*$(,)?
    ) => {
        vec![
            $(
                Box::new($addon),
            )*
        ]
    };
}

/// Some built-in arguments for the Coppo CLI.
impl CoppoCli {
    pub fn invoke_builtin(&mut self) -> &mut Self {
        self.command = self
            .command
            .clone()
            .args(&[
                arg!(-q --quiet "Do not print Coppo log messages"),
            ]);
        self
    }
}
