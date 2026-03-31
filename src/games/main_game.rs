use bevy::prelude::*;
use super::OnGameScreen;
use crate::menus::AppState;
use crate::background::get_bg_setup;

pub fn main_game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bg_entity = get_bg_setup(&mut commands, &asset_server);
    commands.entity(bg_entity);

    commands.spawn((DespawnOnExit(AppState::Game), OnGameScreen));
}
