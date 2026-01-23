use bevy::prelude::*;
use hachiya::Registrar;
use shared::Apple;

#[unsafe(no_mangle)]
fn main(registrar: &mut Registrar) {
    registrar.register_component::<Apple>();
    registrar.add_systems(PostUpdate, ((spawn_apples, another_one), (fizzbuzz, foobar), test));
}

fn spawn_apples(mut commands: Commands) {
    commands.spawn((Name::new("Red Delicious"), Apple {seeds: 7}));
    commands.spawn((Name::new("Granny Smith"), Apple {seeds: 9}));
    commands.spawn((Name::new("Fuji"), Apple {seeds: 4}));
}

fn another_one() {
    println!("dummy");
}

fn fizzbuzz() {
    println!("dummy");
}

fn foobar() {
    println!("dummy");
}

fn test() {
    println!("dummy");
}


