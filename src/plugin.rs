//! TODO: Document.

use bevy::ecs::system::SystemState;
use bevy::prelude::*;

use crate::dylib::Dylib;
use crate::registrar::Registrar;

fn process_dylibs(world: &mut World, state: &mut SystemState<MessageMutator<AssetEvent<Dylib>>>) {
    let messages: Vec<AssetEvent<Dylib>> = state.get_mut(world).read().map(|r| r.clone()).collect();

    world.resource_scope(|world: &mut World, mut registrar: Mut<Registrar>| {
        world.resource_scope(|world: &mut World, mut assets: Mut<Assets<Dylib>>| {
            for message in messages {
                println!("{:#?}", message);
                match message {
                    AssetEvent::<Dylib>::Added { id } => {
                        registrar.register(world, &mut assets, &id);
                    }
                    AssetEvent::<Dylib>::Modified { id } => {
                        registrar.update(world, &mut assets, &id);
                    }
                    _ => (),
                }
            }
        });
    });
}

pub struct HachiyaPlugin;

impl Plugin for HachiyaPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Dylib>()
            .init_asset_loader::<Dylib>()
            .init_resource::<Registrar>()
            .add_systems(Last, {
                let mut state: Option<SystemState<MessageMutator<AssetEvent<Dylib>>>> = None;
                move |world: &mut World| {
                    let state = state.get_or_insert_with(|| SystemState::new(world));
                    process_dylibs(world, state);
                }
            });
    }
}
