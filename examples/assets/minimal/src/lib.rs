///! TODO: Document.
use bevy::prelude::*;
use hachiya::DynamicApp;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct FooSet;

#[stabby::export(canaries)]
fn main(app: &mut DynamicApp) {
    println!("flingfling");
    app.add_systems(PostUpdate, FooSet, || {println!("honghasdasdf")});
}
