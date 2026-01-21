use bevy::prelude::*;
use hachiya::Registrar;
use shared::Apple;

#[unsafe(no_mangle)]
fn main(registrar: &mut Registrar) {
    registrar.add_systems(PostUpdate, count_apples);
}

fn count_apples(query: Query<(&Name, &Apple)>) {
    for (name, apple) in query.iter() {
        println!("{} with {} seeds", name, apple.seeds)
    }
}

