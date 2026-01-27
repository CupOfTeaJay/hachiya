//! TODO: Document.

use bevy::ecs::{intern::Interned, schedule::ScheduleLabel};
use bevy::prelude::*;

use crate::repository::Repository;

/// Helper system for calling [`Repository::update`].
fn poll(mut repository: ResMut<Repository>) {
    repository.update();
}

/// Initializes Hachiya during Bevy's `Startup` schedule according to the
/// [`HachiyaPlugin`] configuration.
///
/// At the moment, initialization simply entails the construction and insertion
/// of a [`Repository`] resource into the main application.
fn initialize(commands: &mut Commands, plugin: &HachiyaPlugin) {
    match Repository::new(plugin) {
        Ok(repository) => {
            commands.insert_resource(repository);
            info!("initialization successful");
        }
        Err(err) => error!("initialization unsuccessful; {}", err),
    }
}

/// Hachiya's configuration. This should be added as a plugin to the main Bevy
/// application in order for everything to work.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use hachiya::HachiyaPlugin;
///
/// fn main() {
///     App::new().add_plugins(HachiyaPlugin::default());
/// }
/// ```
#[derive(Clone)]
pub struct HachiyaPlugin {
    /// The schedule in which the [`Repository`]'s [`crate::BuildState`] is
    /// polled and updated.
    ///
    /// Polling is enabled and performed in Bevy's `Update` schedule by default.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::ecs::{intern::Interned, schedule::ScheduleLabel};
    /// use bevy::prelude::*;
    /// use hachiya::HachiyaPlugin;
    ///
    /// // Continuously poll the mod repository in the `PostUpdate` schedule.
    /// let plugin: HachiyaPlugin = HachiyaPlugin {
    ///     poll_schedule: PostUpdate.intern(),
    ///     ..default()
    /// };
    /// ```
    pub poll_schedule: Interned<dyn ScheduleLabel>,

    /// The path to the root directory of the mod [`Repository`] manage.
    ///
    /// By default, the repository is expected to be under a `mods/` directory.
    /// If you are in a development context (the `CARGO_MANIFEST_DIR`
    /// environment variable is set), then it is assumed that this directory
    /// will be next to your project's `Cargo.toml`. Otherwise, a deployment
    /// context is inferred, and the directory is assumed to be next to your
    /// application's executable.
    ///
    /// If you want to specify a custom path, then a custom one may be provided.
    /// This path will be validated during the Startup schedule.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use hachiya::HachiyaPlugin;
    ///
    /// // Interperet the Cargo workspace located under `path/to/some/mods/` as
    /// // a mod repository.
    /// let plugin: HachiyaPlugin = HachiyaPlugin {
    ///     repository_path: Some("path/to/some/mods/".to_string()),
    ///     ..default()
    /// };
    /// ```
    pub repository_path: Option<String>,

    /// The path to the root directory containing a modding Software Development
    /// Kit (SDK).
    ///
    /// The SDK should contain:
    ///   * TODO:
    ///
    /// By default, the SDK is expected to be in an `sdk/` directory under the
    /// [`HachiyaPlugin::repository_path`]. You may specify something different
    /// if you do not want to follow this structure. A custom SDK path will be
    /// validated during the Startup schedule.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use hachiya::HachiyaPlugin;
    ///
    /// // Look for the modding SDK under the `path/to/the/sdk` directory.
    /// let plugin: HachiyaPlugin = HachiyaPlugin {
    ///     sdk_path: Some("path/to/the/sdk/".to_string()),
    ///     ..default()
    /// };
    /// ```
    pub sdk_path: Option<String>,
}

impl Default for HachiyaPlugin {
    /// Standard configuration for the [`HachiyaPlugin`].
    ///   * Use an inferred path for the mod [`Repository`]'s root directory.
    ///   * Look for the SDK under `<repository_path>/sdk/`
    ///   * Continuously poll the repository in Bevy's `Update` schedule.
    fn default() -> Self {
        HachiyaPlugin {
            poll_schedule: Update.intern(),
            repository_path: None,
            sdk_path: None,
        }
    }
}

impl Plugin for HachiyaPlugin {
    fn build(&self, app: &mut App) {
        let plugin: HachiyaPlugin = self.clone();
        app.add_systems(plugin.poll_schedule, poll);
        app.add_systems(Startup, move |mut commands: Commands| {
            initialize(&mut commands, &plugin);
        });
    }
}
