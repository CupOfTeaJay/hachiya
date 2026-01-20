//! TODO: Document.

use bevy::ecs::{
    schedule::{ScheduleConfigs, ScheduleLabel},
    system::ScheduleSystem,
};
use bevy::prelude::*;

/// TODO: Document.
pub struct Registrar {
    /// TODO: Document.
    systems: Vec<(Box<dyn ScheduleLabel>, ScheduleConfigs<ScheduleSystem>)>,
}

impl Registrar {
    /// TODO: Document.
    pub fn new() -> Self {
        Registrar {
            systems: Vec::new(),
        }
    }

    /// TODO: Document.
    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.systems
            .push((Box::new(schedule), systems.into_configs()));
        self
    }
}

impl Default for Registrar {
    fn default() -> Self {
        Registrar::new()
    }
}
