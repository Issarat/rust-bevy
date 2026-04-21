use bevy::prelude::*;
use bevy::window::WindowResolution;

mod entities;
mod camera;
mod npc;
mod world;
mod ui;

use entities::GameEntity;
use camera::CameraPlugin;
use npc::NpcPlugin;
use world::WorldPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(get_window_config()))
        .add_plugins((CameraPlugin, NpcPlugin, WorldPlugin, UiPlugin))
        .run();
}

fn get_window_config() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Game".into(),
            resolution: WindowResolution::new(1280, 720),
            resize_constraints: WindowResizeConstraints {
                min_width: 800.0,
                min_height: 600.0,
                ..default()
            },
            ..default()
        }),
        ..default()
    }
}
