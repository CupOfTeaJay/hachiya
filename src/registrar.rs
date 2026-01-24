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
struct Context {
    /// TODO: Document.
    conditions: Vec<BoxedCondition>,

    /// TODO: Document.
    members: Vec<String>,

    /// TODO: Document.
    metadata: <ScheduleSystem as Schedulable>::GroupMetadata,
}

impl Context {
    /// TODO: Document.
    pub fn new(
        conditions: Vec<BoxedCondition>,
        members: Vec<String>,
        metadata: <ScheduleSystem as Schedulable>::GroupMetadata,
    ) -> Self {
        Context {
            conditions,
            members,
            metadata,
        }
    }
}

/// TODO: Document.
pub struct Registrar {
    /// TODO: Document.
    components: Vec<ComponentDescriptor>,

    /// TODO: Document.
    contexts: Vec<Context>,

    /// TODO: Document.
    resources: Vec<ComponentDescriptor>,

    /// TODO: Document.
    systems: HashMap<String, SystemPair>,
}

impl Registrar {
    /// TODO: Document.
    fn recurse_system_set(
        &mut self,
        name: &SystemsName,
        schedule: Interned<dyn ScheduleLabel>,
        configs: ScheduleConfigs<ScheduleSystem>,
    ) {
        match configs {
            ScheduleConfigs::ScheduleConfig(configs) => {
                if let SystemsName::Monomorph(name) = name {
                    self.systems.insert(
                        name.clone(),
                        (schedule, ScheduleConfigs::ScheduleConfig(configs)),
                    );
                } else {
                    // This should never happen.
                }
            }
            ScheduleConfigs::Configs {
                configs,
                collective_conditions,
                metadata,
            } => {
                let members: Vec<String> = name.members();
                if let SystemsName::Polymorph(name) = name {
                    self.contexts
                        .push(Context::new(collective_conditions, members, metadata));
                    for (subname, config) in name.iter().zip(configs.into_iter()) {
                        self.recurse_system_set(subname, schedule, config);
                    }
                } else {
                    // This should never happen.
                }
            }
        }
    }

    /// TODO: Document.
    pub fn add_systems<M, S: IntoScheduleConfigs<ScheduleSystem, M>>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: S,
    ) -> &mut Self {
        self.recurse_system_set(
            &SystemsName::new(&std::any::type_name::<S>().replace(' ', "")),
            schedule.intern(),
            systems.into_configs(),
        );
        self
    }

    /// Gets a draining iterator over all of the
    /// [`Component`](bevy::ecs::component::Component) registrations that have
    /// been catalogued by this [`Registrar`].
    pub fn components(&mut self) -> impl Iterator<Item = ComponentDescriptor> {
        self.components.drain(..)
    }

    /// TODO: Document.
    pub fn new() -> Self {
        Registrar {
            components: Vec::new(),
            contexts: Vec::new(),
            resources: Vec::new(),
            systems: HashMap::new(),
        }
    }

    /// Gets an iterator over all of the system "names" that have been
    /// catalogued by this [`Registrar`].
    ///
    /// Names in this context are just strings resolved by crate, module, and
    /// function. For example, a system named `my_system` in the `systems`
    /// module under the `my_game` crate would have a name of
    /// `my_game::systems::my_system`.
    ///
    /// This can be useful for determining valid targets that can be passed into
    /// [`override_system`](Registrar::override_system).
    pub fn registered_systems(&self) -> impl Iterator<Item = &String> {
        self.systems.keys()
    }

    /// Gets a draining iterator over all of the
    /// [`Resource`](bevy::ecs::prelude::Resource) registrations that have been
    /// catalogued by this [`Registrar`].
    pub fn resources(&mut self) -> impl Iterator<Item = ComponentDescriptor> {
        self.resources.drain(..)
    }

    /// Gets a draining iterator over all of the system registrations that have
    /// been catalogued by this [`Registrar`].
    pub fn systems(&mut self) -> impl Iterator<Item = (String, SystemPair)> {
        self.systems.drain()
    }

    /// Substitutes a system registration that has been catalogued by this
    /// [`Registrar`] with another system.
    pub fn override_system(
        &mut self,
        _target: &str,
        _system: impl IntoScheduleConfigs<ScheduleSystem, ()>,
    ) -> &mut Self {
        todo!()
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

/// TODO: Document.
#[derive(Debug)]
enum SystemsName {
    Monomorph(String),
    Polymorph(Vec<SystemsName>),
}

impl SystemsName {
    /// Gets
    pub fn members(&self) -> Vec<String> {
        match self {
            SystemsName::Monomorph(name) => vec![name.clone()],
            SystemsName::Polymorph(name) => {
                let mut members = Vec::new();
                for subname in name {
                    members.extend(subname.members());
                }
                members
            }
        }
    }

    /// Generates names for a given system or set of systems. If a set of
    /// systems is passed, then a name will be derived for each one. The names
    /// are used to uniquely identify and reference system registrations on
    /// an individual basis.
    pub fn new(tokens: &str) -> Self {
        if tokens.starts_with('(') && tokens.ends_with(')') {
            let mut depth: usize = 0;
            let mut curr: String = String::new();
            let mut subs: Vec<SystemsName> = Vec::new();
            for char in tokens[1..(tokens.len() - 1)].chars() {
                match char {
                    '(' => {
                        depth += 1;
                        curr.push(char);
                    }
                    ')' => {
                        depth -= 1;
                        curr.push(char);
                    }
                    ',' => {
                        if depth == 0 {
                            subs.push(SystemsName::new(&curr));
                            curr.clear();
                        } else {
                            curr.push(char);
                        }
                    }
                    _ => curr.push(char),
                }
            }
            subs.push(SystemsName::new(&curr));
            SystemsName::Polymorph(subs)
        } else {
            SystemsName::Monomorph(tokens.to_string())
        }
    }
}
