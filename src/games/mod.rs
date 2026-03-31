use bevy::prelude::*;

use crate::menus::AppState;

mod main_game;
use main_game::main_game_setup;
#[derive(Component)]
struct OnGameScreen;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), main_game_setup);
}
