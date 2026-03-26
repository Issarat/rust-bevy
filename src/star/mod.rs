use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::shared::spawn_sprites;
use crate::player::PlayerLife;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_star)
            .add_systems(Update, spawn_stars_over_time);
    }
}

// ── Constants ───────────────────────────────────────────────────

pub mod stars {
    pub const COUNT: usize = 10;
    pub const SIZE: f32 = 30.0;
    pub const MAX_COUNT: usize = 10;
}

// ── Components ──────────────────────────────────────────────────

#[derive(Component)]
pub struct Star;

// ── Helpers ─────────────────────────────────────────────────────

fn star_bundle(asset_server: &AssetServer, x: f32, y: f32) -> impl Bundle {
    (
        Sprite::from_image(asset_server.load("sprites/star.png")),
        Transform::from_xyz(x, y, 0.0),
        Star,
    )
}

// ── Systems ─────────────────────────────────────────────────────

fn spawn_star(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(window) = window_query.single() else { return };
    spawn_sprites(&mut commands, window, stars::COUNT, None, |x, y| {
        star_bundle(&asset_server, x, y)
    });
}

fn spawn_stars_over_time(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>,
    player_life: Res<PlayerLife>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if star_query.iter().count() < stars::MAX_COUNT && player_life.count > 0 {
        let Ok(window) = window_query.single() else { return };
        spawn_sprites(&mut commands, window, 1, None, |x, y| {
            star_bundle(&asset_server, x, y)
        });
    }
}
