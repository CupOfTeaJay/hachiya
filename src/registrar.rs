//! TODO: Document.

use std::collections::HashMap;

use bevy::ecs::{
    component::ComponentDescriptor,
    intern::Interned,
    schedule::{BoxedCondition, Schedulable, ScheduleConfigs, ScheduleLabel},
    system::ScheduleSystem,
};
use bevy::prelude::*;

// TODO: Document.
type SystemPair = (Interned<dyn ScheduleLabel>, ScheduleConfigs<ScheduleSystem>);

/// TODO: Document.
enum SystemsName {
    Monomorph(String),
    Polymorph(Vec<SystemsName>),
}

impl SystemsName {
    /// Generates names for a given system or set of systems. If a set of
    /// systems is passed, then a name will be derived for each one. The names
    /// are used to uniquely identify and reference individual registrations.
    pub fn new(tokens: &str) -> Self {
        if tokens.starts_with('(') {
            let mut curr: String = String::new();
            let mut subs: Vec<SystemsName> = Vec::new();
            for char in tokens.chars().skip(1) {
                match char {
                    ')' => {
                        curr.push(char);
                        subs.push(SystemsName::new(&curr));
                        curr.clear();
                    }
                    _ => curr.push(char),
                }
            }
            SystemsName::Polymorph(subs)
        } else {
            SystemsName::Monomorph(tokens.to_string())
        }
    }
}

/// TODO: Document.
pub struct Registrar {
    /// TODO: Document.
    components: Vec<ComponentDescriptor>,

    /// TODO: Document.
    resources: Vec<ComponentDescriptor>,

    /// TODO: Document.
    systems: HashMap<String, SystemPair>,
}

impl Registrar {
    /// TODO: Document.
    fn recurse_system_set<T: Schedulable>(
        &self,
        configs: &Vec<ScheduleConfigs<T>>,
        collective_conditions: &Vec<BoxedCondition>,
        metadata: &T::GroupMetadata,
    ) {
    }

    /// TODO: Document.
    pub fn add_systems<M, S: IntoScheduleConfigs<ScheduleSystem, M>>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: S,
    ) -> &mut Self {
        let name: SystemsName =
            SystemsName::new(&std::any::type_name::<S>().replace(' ', ""));
        match systems.into_configs() {
            ScheduleConfigs::ScheduleConfig(config) => {
                if let SystemsName::Monomorph(name) = name {
                    self.systems.insert(
                        name.clone(),
                        (schedule.intern(), ScheduleConfigs::ScheduleConfig(config)),
                    );
                }
            }
            ScheduleConfigs::Configs {
                configs,
                collective_conditions,
                metadata,
            } => self.recurse_system_set(&configs, &collective_conditions, &metadata),
        }
        self
    }

    /// TODO: Document.
    pub fn drain_components(&mut self) -> std::vec::Drain<'_, ComponentDescriptor> {
        self.components.drain(..)
    }

    /// TODO: Document.
    pub fn drain_resources(&mut self) -> std::vec::Drain<'_, ComponentDescriptor> {
        self.resources.drain(..)
    }

    /// TODO: Document.
    pub fn drain_systems(&mut self) -> std::collections::hash_map::Drain<'_, String, SystemPair> {
        self.systems.drain()
    }

    /// TODO: Document.
    pub fn new() -> Self {
        Registrar {
            components: Vec::new(),
            resources: Vec::new(),
            systems: HashMap::new(),
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
