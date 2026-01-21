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
    fn apply_systems(&mut self, world: &mut World) {
        world.resource_scope(|_world: &mut World, mut schedules: Mut<Schedules>| {
            for (label, systems) in self.registrar.drain_systems() {
                if let Some(schedule) = schedules.get_mut(label) {
                    schedule.add_systems(systems);
                }
            }
        });
    }

    /// TODO: Document.
    pub fn apply(&mut self, world: &mut World) {
        self.apply_systems(world);
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
