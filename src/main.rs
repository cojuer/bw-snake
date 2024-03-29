mod game;

use bevy::prelude::*;
use game::plugin::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_plugin(GamePlugin)
        .run();
}
