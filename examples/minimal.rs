//! TODO: Document.

use bevy::prelude::*;
use hachiya::{HachiyaPlugin, LoadMods};

// Taken from `bevy/examples/ui/tab_navigation.rs`.
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// TODO: Document.
fn button() -> impl Bundle {
    (
        Button,
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new("Load Mods"),
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}

/// TODO: Document.
fn load(this: On<Pointer<Release>>, mut commands: Commands) {
    commands.write_message(LoadMods);
}

/// TODO: Document.
fn root() -> impl Bundle {
    Node {
        width: percent(100),
        height: percent(100),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

/// TODO: Document.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let button: Entity = commands
        .spawn(button())
        .observe(load)
        .observe(
            |this: On<Pointer<Press>>, mut query: Query<&mut BackgroundColor>| {
                *query.get_mut(this.entity).unwrap() = PRESSED_BUTTON.into();
            },
        )
        .observe(
            |this: On<Pointer<Over>>, mut query: Query<&mut BackgroundColor>| {
                *query.get_mut(this.entity).unwrap() = HOVERED_BUTTON.into();
            },
        )
        .observe(
            |this: On<Pointer<Out>>, mut query: Query<&mut BackgroundColor>| {
                *query.get_mut(this.entity).unwrap() = NORMAL_BUTTON.into();
            },
        )
        .id();
    commands.spawn(root()).add_child(button);
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HachiyaPlugin::new("./examples/mods/")))
        .add_systems(Startup, setup)
        .run();
}
