///! TODO: Document.
use bevy::prelude::*;
use hachiya::DynamicApp;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
struct Foo;

#[stabby::export(canaries)]
fn main(app: &mut DynamicApp) {
    app.add_systems(PostUpdate, Foo, || {println!("Dingleboop!")});
}
