//! TODO: Document.

use bevy::{
    ecs::{
        intern::Interned,
        schedule::{ScheduleConfigs, ScheduleLabel},
        system::ScheduleSystem,
    },
    prelude::*,
};

pub struct DynamicApp {
    systems: Vec<SystemsRegistration>,
}

impl DynamicApp {
    pub fn add_systems<M, S: IntoScheduleConfigs<ScheduleSystem, M>>(
        &mut self,
        schedule: impl ScheduleLabel,
        wrapper: impl SystemSet,
        systems: S,
    ) {
        self.systems.push(SystemsRegistration::new(
            SystemsRegistrationContext::new(schedule.intern(), wrapper.intern()),
            systems,
        ));
    }

    pub fn new() -> Self {
        DynamicApp {
            systems: Vec::new(),
        }
    }

    pub fn systems(&mut self) -> impl Iterator<Item = SystemsRegistration> {
        self.systems.drain(..)
    }
}

pub struct SystemsRegistration {
    pub context: SystemsRegistrationContext,
    pub configs: ScheduleConfigs<ScheduleSystem>,
}

impl SystemsRegistration {
    pub fn new<M, S: IntoScheduleConfigs<ScheduleSystem, M>>(
        context: SystemsRegistrationContext,
        systems: S,
    ) -> Self {
        SystemsRegistration {
            configs: systems.in_set(context.set).into_configs(),
            context,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct SystemsRegistrationContext {
    pub schedule: Interned<dyn ScheduleLabel>,
    pub set: Interned<dyn SystemSet>,
}

impl SystemsRegistrationContext {
    pub fn new(schedule: Interned<dyn ScheduleLabel>, set: Interned<dyn SystemSet>) -> Self {
        SystemsRegistrationContext { schedule, set }
    }
}
