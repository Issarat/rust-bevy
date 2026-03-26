use bevy::prelude::*;
use crate::player::{ Player, PlayerLife };
use crate::enemy::Enemy;
use crate::star::Star;
use crate::score::Score;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GameOver>().add_systems(Update, event_game_over_trigger);
    }
}

#[derive(Message)]
pub struct GameOver {
    pub final_score: u32,
}

fn event_game_over_trigger(
    mut commands: Commands,
    player_life: Res<PlayerLife>,
    score: Res<Score>,
    player_query: Query<Entity, With<Player>>,
    enemies_query: Query<Entity, With<Enemy>>,
    stars_query: Query<Entity, With<Star>>,
    mut game_over_writer: MessageWriter<GameOver>
) {
    if player_life.is_changed() && player_life.count == 0 {
        if let Ok(player_entity) = player_query.single() {
            commands.entity(player_entity).despawn();
            game_over_writer.write(GameOver { final_score: score.value });
            println!("Game Over! Final score: {}", score.value);
        }
        for entity in stars_query.iter().chain(enemies_query.iter()) {
            commands.entity(entity).despawn();
        }
    }
}
