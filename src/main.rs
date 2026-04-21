use bevy::prelude::*;
use bevy::window::WindowResolution;
use std::marker::PhantomData;
use uuid::Uuid;

mod camera;
mod npc;
mod world;

use camera::CameraPlugin;
use npc::NpcPlugin;
use world::WorldPlugin;

#[derive(Component)]
pub struct GameEntity<T> {
    pub id: Uuid,
    _marker: PhantomData<T>, // tells Rust that T is used
}

impl<T> GameEntity<T> {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(get_window_config()))
        .add_plugins((CameraPlugin, NpcPlugin, WorldPlugin))
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
