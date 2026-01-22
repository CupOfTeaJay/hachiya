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
    resources: Vec<ComponentDescriptor>,

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
    pub fn drain_resources(&mut self) -> Drain<'_, ComponentDescriptor> {
        self.resources.drain(..)
    }

    /// TODO: Document.
    pub fn drain_systems(&mut self) -> Drain<'_, SystemPair> {
        self.systems.drain(..)
    }

    /// TODO: Document.
    pub fn new() -> Self {
        Registrar {
            components: Vec::new(),
            resources: Vec::new(),
            systems: Vec::new(),
        }
    }

    /// Registers a new [`Component`](bevy::ecs::component::Component) type with
    /// the [`World`](bevy::ecs::prelude::World) owned by the main application.
    pub fn register_component<C: Component>(&mut self) -> &mut Self {
        self.components.push(ComponentDescriptor::new::<C>());
        self
    }

    /// Registers a new [`Resource`](bevy::ecs::prelude::Resource) type with the
    /// [`World`](bevy::ecs::prelude::World) owned by the main application.
    pub fn register_resource<R: Resource + Component>(&mut self) -> &mut Self {
        self.resources.push(ComponentDescriptor::new::<R>());
        self
    }
}

impl Default for Registrar {
    fn default() -> Self {
        Registrar::new()
    }
}
