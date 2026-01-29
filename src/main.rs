use bevy::prelude::*;
use hachiya::SharedObject;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_asset::<SharedObject>()
        .init_asset_loader::<SharedObject>()
        .add_systems(Startup, |asset_server: Res<AssetServer>| {
            let _foo: Handle<SharedObject> = asset_server.load("libmod_a.so");
        })
        .run();
}
