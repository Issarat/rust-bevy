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
            title: "Bevy Game Ui".into(),
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
