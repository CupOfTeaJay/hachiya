//! TODO: Document.

use bevy::ecs::{schedule::ScheduleLabel, system::SystemState};
use bevy::prelude::*;
use camino::Utf8PathBuf;

mod error;
mod repo;

use error::HachiyaError;
use repo::ModRepository;

/// TODO: Document.
fn load_mods(world: &mut World) {
    let mut state: SystemState<MessageMutator<LoadMods>> = SystemState::new(world);
    let mut messages: MessageMutator<LoadMods> = state.get_mut(world);
    if !messages.is_empty() {
        messages.clear();
        world.resource_scope(|world: &mut World, mut repository: Mut<ModRepository>| {
            repository.load(world)
        });
    }
}

/// TODO: Document.
fn resolve() -> Result<Utf8PathBuf, HachiyaError> {
    let mut root: Utf8PathBuf;
    if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
        debug!("resolving path to `ModRepository` via CARGO_MANIFEST_DIR");
        root = Utf8PathBuf::from(dir);
    } else {
        debug!("resolving path to `ModRepository` via std::env::current_exe()");
        match std::env::current_exe() {
            Ok(exe) => {
                if let Some(parent) = exe.parent() {
                    root = Utf8PathBuf::from(parent.to_string_lossy().into_owned());
                } else {
                    return Err(HachiyaError::InitializationError(format!(
                        "repository could not be resolved from executable: {} has no parent",
                        exe.display()
                    )));
                }
            }
            Err(err) => {
                return Err(HachiyaError::InitializationError(format!(
                    "repository path could not be resolved from executable: {}",
                    err
                )));
            }
        }
    }
    root.push("assets");
    root.push("mods");
    Ok(root)
}

/// Initializes Hachiya during the [`Startup`](bevy::app::Startup) schedule
/// according to the [`HachiyaPlugin`] configuration.
fn initialize(commands: &mut Commands, plugin: &HachiyaPlugin) -> Result<(), HachiyaError> {
    // Try to determine where the `ModRepository` should be located.
    let root: Utf8PathBuf;
    if let Some(repository) = &plugin.repository {
        debug!("using user-designated path for `ModRepository`");
        root = Utf8PathBuf::from(repository);
    } else {
        root = resolve()?;
    }

    // TODO:
    commands.insert_resource(ModRepository::new(root)?);

    // TODO:
    info!("initialization successful");
    Ok(())
}

/// Hachiya's configuration. This should be added as a plugin to the main Bevy
/// application.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use hachiya::HachiyaPlugin;
///
/// fn main() {
///     App::new().add_plugins(HachiyaPlugin::default()).run();
/// }
/// ```
#[derive(Clone)]
pub struct HachiyaPlugin {
    /// The path of the [ModRepository] to initialize and manage.
    pub repository: Option<String>,
}

impl Default for HachiyaPlugin {
    /// Standard configuration for the [`HachiyaPlugin`]. By default, Hachiya
    /// assumes the [`ModRepository`] is located alongside the rest of the
    /// game's resources under `./assets/mods`.
    fn default() -> Self {
        HachiyaPlugin { repository: None }
    }
}

impl Plugin for HachiyaPlugin {
    fn build(&self, app: &mut App) {
        let plugin: HachiyaPlugin = self.clone();
        app.add_systems(Startup, {
            move |mut commands: Commands| -> Result<(), BevyError> {
                Ok(initialize(&mut commands, &plugin)?)
            }
        })
        .add_systems(Update, load_mods)
        .add_message::<LoadMods>();
    }
}

/// TODO: Document.
#[derive(Message)]
pub struct LoadMods;

/// TODO: Document.
pub struct Registrar {
    /// TODO: Document.
    systems: Vec<(Box<dyn ScheduleLabel>, Box<dyn System<In = (), Out = ()>>)>,
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
        system: impl IntoSystem<(), (), M>,
    ) -> &mut Self {
        self.systems.push((
            Box::new(schedule),
            Box::new(IntoSystem::into_system(system)),
        ));
        self
    }
}

impl Default for Registrar {
    fn default() -> Self {
        Registrar::new()
    }
}
