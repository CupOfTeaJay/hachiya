use bevy::prelude::*;
use hachiya::Registrar;

#[unsafe(no_mangle)]
fn main(registrar: &mut Registrar) {
    registrar.add_systems(Update, hello);
}

fn hello() {
    println!("Hello from Mod A");
}
