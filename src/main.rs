use bevy::prelude::*;
use hachiya::Dylib;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_asset::<Dylib>()
        .init_asset_loader::<Dylib>()
        .add_systems(Startup, |asset_server: Res<AssetServer>| {
            let _foo: Handle<Dylib> = asset_server.load("libmod_a.so");
        })
        .run();
}
