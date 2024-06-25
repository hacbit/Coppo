//! A CLI tool for Coppo.
//!

#![forbid(unsafe_code)]
#![feature(type_alias_impl_trait)]
#![allow(clippy::new_without_default)]

use coppo_addons::prelude::*;

pub type Addons = Vec<Box<dyn Addon>>;

pub struct CoppoCli {
    addons: Addons,
}

impl CoppoCli {
    pub fn new() -> Self {
        Self { addons: vec![] }
    }

    pub fn add_addon<A: Addon + 'static>(&mut self, addon: A) -> &mut Self {
        self.addons.push(Box::new(addon));
        self
    }

    pub fn run(&self) {
        println!("Running Coppo CLI");
    }
}
