//! TODO: Document.

use bevy::log::LogPlugin;
use bevy::prelude::*;
use hachiya::{HachiyaPlugin, LoadMods};

fn main() {
    App::new()
        .add_plugins((
            HachiyaPlugin::new("./examples/mods/"),
            LogPlugin {
                filter: "hachiya=debug".into(),
                ..default()
            },
        ))
        .add_systems(Startup, |mut commands: Commands| {
            commands.write_message(LoadMods);
        })
        .run();
}
