mod game;

use bevy::prelude::*;
use game::plugin::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(OrthographicCameraBundle::new_2d());
            commands.spawn_bundle(UiCameraBundle::default());
        })
        .add_plugin(GamePlugin)
        .run();
}
