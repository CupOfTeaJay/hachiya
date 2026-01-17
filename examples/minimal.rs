//! TODO: Document.

use bevy::prelude::*;
use hachiya::HachiyaPlugin;

fn main() {
    App::new()
        .add_plugins(HachiyaPlugin::new("./examples/mods/"))
        .run();
}
