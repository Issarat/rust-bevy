use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;
use crate::shared::{ get_bounds, spawn_sprites };
use crate::player::PlayerLife;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer::default())
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, (
                enemies_movement,
                update_enemy_direction,
                confine_enemy_movement,
                update_enemies_movement,
                tick_enemies_spawn_timer,
                spawn_enemies_over_time,
            ));
    }
}

// ── Constants ───────────────────────────────────────────────────

pub mod consts {
    pub const COUNT: usize = 4;
    pub const SPEED: f32 = 200.0;
    pub const SIZE: f32 = 64.0;
    pub const TIME_SPAWNER: f32 = 10.0;
    pub const MAX_SPEED: f32 = 350.0;
}

// ── Components & Resources ──────────────────────────────────────

#[derive(Component)]
pub struct Enemy {
    pub dirction: Vec2,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(consts::TIME_SPAWNER, TimerMode::Repeating) }
    }
}

// ── Helpers ─────────────────────────────────────────────────────

fn enemy_bundle(asset_server: &AssetServer, x: f32, y: f32) -> impl Bundle {
    (
        Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
        Transform::from_xyz(x, y, 0.0),
        Enemy {
            dirction: Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5).normalize(),
        },
    )
}

// ── Systems ─────────────────────────────────────────────────────

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    spawn_sprites(&mut commands, window, consts::COUNT, None, |x, y| {
        enemy_bundle(&asset_server, x, y)
    });
}

fn enemies_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.dirction.x, enemy.dirction.y, 0.0);
        transform.translation += direction * consts::SPEED * time.delta_secs();
    }
}

fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let (x_min, x_max, y_min, y_max) = get_bounds(window, consts::SIZE);

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let pos = transform.translation;
        let mut changed = false;

        if pos.x <= x_min || pos.x >= x_max {
            enemy.dirction.x *= -1.0;
            changed = true;
        }
        if pos.y <= y_min || pos.y >= y_max {
            enemy.dirction.y *= -1.0;
            changed = true;
        }

        if changed {
            let sound = if random::<f32>() > 0.5 {
                asset_server.load("audio/pluck_001.ogg")
            } else {
                asset_server.load("audio/pluck_002.ogg")
            };
            commands.spawn(AudioPlayer::new(sound));
        }
    }
}

fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let (x_min, x_max, y_min, y_max) = get_bounds(window, consts::SIZE);

    for mut transform in enemy_query.iter_mut() {
        transform.translation.x = transform.translation.x.clamp(x_min, x_max);
        transform.translation.y = transform.translation.y.clamp(y_min, y_max);
    }
}

fn tick_enemies_spawn_timer(mut spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    spawn_timer.timer.tick(time.delta());
}

fn spawn_enemies_over_time(
    mut commands: Commands,
    spawn_timer: Res<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_life: Res<PlayerLife>
) {
    if spawn_timer.timer.just_finished() && player_life.count > 0 {
        let Ok(window) = window_query.single() else {
            return;
        };
        spawn_sprites(&mut commands, window, 1, None, |x, y| { enemy_bundle(&asset_server, x, y) });
    }
}

fn update_enemies_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
    score: Res<crate::score::Score>
) {
    if score.is_changed() {
        let speed_bonus = ((score.value / 10) * 10) as f32;
        let speed = (consts::SPEED + speed_bonus).min(consts::MAX_SPEED);

        for (mut transform, enemy) in enemy_query.iter_mut() {
            let direction = Vec3::new(enemy.dirction.x, enemy.dirction.y, 0.0);
            transform.translation += direction * speed * time.delta_secs();
        }
    }
}
