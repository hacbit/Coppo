use coppo_addons::prelude::*;

pub struct CoppoBuildAddon;

impl_addon! {
    CoppoBuildAddon,
    name => "build",
    description => "Compile the current project",
    run => |config, matches| {

    }
}
