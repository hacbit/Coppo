//! This is a simple package manager for Cpp projects.
//! It is written in pure Rust and is designed like Rust's Cargo.
//! It is a work in progress and is not yet ready for use.

#![forbid(unsafe_code)]
#![allow(unused_imports)]

use coppo_cli::{addons, command, CoppoCli};
use coppo_new::CoppoNewAddon;

fn main() {
    CoppoCli::new(command!())
        .invoke_builtin()
        .add_addons(addons![CoppoNewAddon])
        .run()
}
