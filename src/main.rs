use bevy::prelude::*;

mod menus;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(menus::menu_plugin)
        .add_systems(Startup, setup)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

//==================================main menu=========================
