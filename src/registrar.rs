///! TODO: Document.
use std::collections::{HashMap, HashSet};

use bevy::{ecs::schedule::ScheduleCleanupPolicy, prelude::*};

use crate::dylib::Dylib;
use crate::dynapp::{DynamicApp, SystemsRegistrationContext};

#[derive(Default, Resource)]
pub struct Registrar {
    contexts: HashMap<AssetId<Dylib>, HashSet<SystemsRegistrationContext>>,
}

impl Registrar {
    fn register_systems(
        &mut self,
        world: &mut World,
        dynapp: &mut DynamicApp,
        id: &AssetId<Dylib>,
    ) {
        world.resource_scope(|_world: &mut World, mut schedules: Mut<Schedules>| {
            for registration in dynapp.systems() {
                if let Some(schedule) = schedules.get_mut(registration.context.schedule) {
                    schedule.add_systems(registration.configs);
                    self.contexts
                        .get_mut(id)
                        .unwrap()
                        .insert(registration.context);
                }
            }
        });
    }

    fn unregister(&mut self, world: &mut World, assets: &mut Assets<Dylib>, id: &AssetId<Dylib>) {
        world.resource_scope(|world: &mut World, mut schedules: Mut<Schedules>| {
            for context in self.contexts.get(id).unwrap() {
                if let Some(schedule) = schedules.get_mut(context.schedule) {
                    schedule
                        .remove_systems_in_set(
                            context.set,
                            world,
                            ScheduleCleanupPolicy::RemoveSystemsOnly,
                        )
                        .unwrap();
                }
            }
        });
        assets.get_mut_untracked(*id).unwrap().refresh();
    }

    pub fn register(&mut self, world: &mut World, assets: &mut Assets<Dylib>, id: &AssetId<Dylib>) {
        let mut dynapp: DynamicApp = DynamicApp::new();
        assets.get_mut_untracked(*id).expect("no hook!").hook()(&mut dynapp);
        self.contexts.insert(*id, HashSet::new());
        self.register_systems(world, &mut dynapp, id);
    }

    pub fn update(&mut self, world: &mut World, assets: &mut Assets<Dylib>, id: &AssetId<Dylib>) {
        self.unregister(world, assets, id);
        self.register(world, assets, id);
    }
}
