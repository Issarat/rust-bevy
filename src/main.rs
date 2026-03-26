mod shared;
mod events;
mod player;
mod enemy;
mod star;
mod score;

use bevy::prelude::*;
use events::EventsPlugin;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use star::StarPlugin;
use score::ScorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ScorePlugin, PlayerPlugin, EnemyPlugin, StarPlugin, EventsPlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
