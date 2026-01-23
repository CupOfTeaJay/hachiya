//! TODO: Document.

use bevy::prelude::*;

use crate::registrar::Registrar;

/// TODO: Document.
pub struct Applicator {
    /// TODO: Document.
    registrar: Registrar,
}

impl Applicator {
    /// TODO: Document.
    fn add_systems(&mut self, world: &mut World) -> &mut Self {
        world.resource_scope(|_world: &mut World, mut schedules: Mut<Schedules>| {
            for (name, systems) in self.registrar.drain_systems() {
                if let Some(schedule) = schedules.get_mut(systems.0) {
                    println!("Scheduling {}", name);
                    schedule.add_systems(systems.1);
                }
            }
        });
        self
    }

    /// TODO: Document.
    fn register_components(&mut self, world: &mut World) -> &mut Self {
        for component in self.registrar.drain_components() {
            world.register_component_with_descriptor(component);
        }
        self
    }

    /// TODO: Document.
    fn register_resources(&mut self, world: &mut World) -> &mut Self {
        for resource in self.registrar.drain_resources() {
            world.register_resource_with_descriptor(resource);
        }
        self
    }

    /// TODO: Document.
    pub fn apply(&mut self, world: &mut World) {
        self.register_resources(world)
            .register_components(world)
            .add_systems(world);
    }

    /// TODO: Document.
    pub fn new() -> Self {
        Applicator {
            registrar: Registrar::new(),
        }
    }

    /// TODO: Document.
    pub fn registrar(&mut self) -> &mut Registrar {
        &mut self.registrar
    }
}
