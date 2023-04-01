// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
mod environment;
mod player;

mod prelude {
    pub use crate::environment::platforms::*;
    pub use crate::player::player::*;
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(40.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlatformsPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
