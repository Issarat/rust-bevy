use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score::default())
            .add_systems(Update, update_score);
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}
