//! TODO: Document.

use std::vec::Drain;

use bevy::ecs::{
    component::ComponentDescriptor,
    intern::Interned,
    schedule::{ScheduleConfigs, ScheduleLabel},
    system::ScheduleSystem,
};
use bevy::prelude::*;

// TODO: Document.
type SystemPair = (Interned<dyn ScheduleLabel>, ScheduleConfigs<ScheduleSystem>);

/// TODO: Document.
pub struct Registrar {
    /// TODO: Document.
    components: Vec<ComponentDescriptor>,

    /// TODO: Document.
    systems: Vec<SystemPair>,
}

impl Registrar {
    /// TODO: Document.
    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.systems
            .push((schedule.intern(), systems.into_configs()));
        self
    }

    /// TODO: Document.
    pub fn drain_components(&mut self) -> Drain<'_, ComponentDescriptor> {
        self.components.drain(..)
    }

    /// TODO: Document.
    pub fn drain_systems(&mut self) -> Drain<'_, SystemPair> {
        self.systems.drain(..)
    }

    /// TODO: Document.
    pub fn new() -> Self {
        Registrar {
            components: Vec::new(),
            systems: Vec::new(),
        }
    }

    /// TODO: Document.
    pub fn register_component<C: Component>(&mut self) {
        self.components.push(ComponentDescriptor::new::<C>());
    }
}

impl Default for Registrar {
    fn default() -> Self {
        Registrar::new()
    }
}
