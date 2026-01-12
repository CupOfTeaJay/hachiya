use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use dylib::DynamicLibrary;
use glob::glob;

#[derive(Default)]
pub struct LoadedMods {
    libraries: Vec<DynamicLibrary>,
}

// SAFETY: DynamicLibrary handles are safe to send between threads.
// The underlying library is loaded into process memory which is accessible from all threads.
unsafe impl Send for LoadedMods {}
unsafe impl Sync for LoadedMods {}

impl Resource for LoadedMods {}

#[derive(Event, Message)]
pub struct LoadMods {
    pub workspace: String,
}

fn collect(mods: &Path) -> Vec<PathBuf> {
    println!("collecting mods");
    let mut dylibs: Vec<PathBuf> = Vec::new();
    let binding = mods.join("target/debug/*.so");
    let pattern: &str = binding.to_str().unwrap();
    for item in glob(pattern).expect("glob failed") {
        match item {
            Ok(dylib) => dylibs.push(dylib),
            Err(e) => println!("{:?}", e),
        }
    }
    dylibs
}

fn compile(mods: &Path) {
    println!("compiling mods");
    let output = Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            mods.join("Cargo.toml").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let _ = io::stdout().write_all(&output.stdout);
    let _ = io::stderr().write_all(&output.stderr);
}

fn populate(workspace: &Path, world: &mut World) {
    compile(workspace);

    if !world.contains_resource::<LoadedMods>() {
        world.insert_resource(LoadedMods::default());
    }

    let dylibs = collect(workspace);

    world.resource_scope(|world: &mut World, mut loaded_mods: Mut<LoadedMods>| {
        for dylib in dylibs {
            let library = DynamicLibrary::open(Some(&dylib)).unwrap();
            let init: fn(*mut World) =
                unsafe { std::mem::transmute(library.symbol::<usize>("init").unwrap()) };
            init(world as *mut World);
            loaded_mods.libraries.push(library);
        }
    });
}

fn load_mods_system(world: &mut World) {
    let mut system_state: SystemState<(MessageReader<LoadMods>,)> = SystemState::new(world);

    let workspaces: Vec<String> = {
        let mut messages = system_state.get_mut(world);
        messages.0.read().map(|msg| msg.workspace.clone()).collect()
    };

    for workspace in workspaces {
        populate(Path::new(&workspace), world);
    }
}

/// TODO: Document.
pub struct FuyuPlugin;

impl Plugin for FuyuPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LoadMods>();
        app.add_systems(Update, load_mods_system);
    }
}

