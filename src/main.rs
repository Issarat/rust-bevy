use bevy::{ prelude::*, window::{ WindowResolution, PresentMode } };
mod menus;
mod background;
mod games;

use menus::{ AppState };
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(get_window_config()))
        .init_state::<AppState>()
        .add_plugins((menus::menu_plugin, games::game_plugin))
        .add_systems(Startup, setup)
        .run();
}

//==================================set up=========================

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn get_window_config() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "My Isometric Sandbox".into(),
            resolution: WindowResolution::new(1280.0 as u32, 720.0 as u32), // f32, not u32
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
