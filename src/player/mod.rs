use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::shared::get_bounds;
use crate::star::{ Star, stars };
use crate::score::Score;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerLife::default())
            .insert_resource(PlayerProtectionTime::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_movement,
                confine_player_movement,
                player_hit_star,
                tick_player_protect_timer,
                blink_player,
                enemy_hit_player,
            ));
    }
}

// ── Constants ───────────────────────────────────────────────────

pub mod consts {
    pub const SPEED: f32 = 500.0;
    pub const SIZE: f32 = 64.0;
    pub const LIFE_COUNT: u8 = 3;
    pub const PROTECT_TIME: f32 = 1.0;
    pub const BLINK_RATE: f32 = 0.1;
}

// ── Components & Resources ──────────────────────────────────────

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct BlinkTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct PlayerLife {
    pub count: u8,
}
impl Default for PlayerLife {
    fn default() -> Self {
        Self { count: consts::LIFE_COUNT }
    }
}

#[derive(Resource)]
pub struct PlayerProtectionTime {
    pub timer: Timer,
}
impl Default for PlayerProtectionTime {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(consts::PROTECT_TIME, TimerMode::Once);
        timer.finish(); // pre-finish so player can take damage immediately
        Self { timer }
    }
}

// ── Systems ─────────────────────────────────────────────────────

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ball_blue_large.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
        BlinkTimer {
            timer: Timer::from_seconds(consts::BLINK_RATE, TimerMode::Repeating),
        },
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        transform.translation += direction * consts::SPEED * time.delta_secs();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    if let Ok(mut t) = player_query.single_mut() {
        let (x_min, x_max, y_min, y_max) = get_bounds(window, consts::SIZE);
        t.translation.x = t.translation.x.clamp(x_min, x_max);
        t.translation.y = t.translation.y.clamp(y_min, y_max);
    }
}

fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>
) {
    let Ok(player_t) = player_query.single() else {
        return;
    };
    let player_radius = consts::SIZE / 2.0;
    let star_radius = stars::SIZE / 2.0;

    for (star_entity, star_t) in star_query.iter_mut() {
        if player_t.translation.distance(star_t.translation) < player_radius + star_radius {
            commands.spawn(AudioPlayer::new(asset_server.load("audio/laserLarge_000.ogg")));
            commands.entity(star_entity).despawn();
            score.value += 1;
        }
    }
}

fn tick_player_protect_timer(mut protect_timer: ResMut<PlayerProtectionTime>, time: Res<Time>) {
    protect_timer.timer.tick(time.delta());
}

fn blink_player(
    mut player_query: Query<(&mut Visibility, &mut BlinkTimer), With<Player>>,
    player_protect_timer: Res<PlayerProtectionTime>,
    time: Res<Time>
) {
    let Ok((mut visibility, mut blink)) = player_query.single_mut() else {
        return;
    };

    if !player_protect_timer.timer.is_finished() {
        blink.timer.tick(time.delta());
        if blink.timer.just_finished() {
            *visibility = match *visibility {
                Visibility::Visible => Visibility::Hidden,
                _ => Visibility::Visible,
            };
        }
    } else {
        *visibility = Visibility::Visible;
        blink.timer.reset();
    }
}

fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<crate::enemy::Enemy>>,
    asset_server: Res<AssetServer>,
    mut player_life: ResMut<PlayerLife>,
    mut protect_timer: ResMut<PlayerProtectionTime>
) {
    let Ok(player_t) = player_query.single_mut() else {
        return;
    };
    let player_radius = consts::SIZE / 2.0;
    let enemy_radius = crate::enemy::consts::SIZE / 2.0;

    for enemy_t in enemy_query.iter() {
        if player_t.translation.distance(enemy_t.translation) < player_radius + enemy_radius {
            if protect_timer.timer.is_finished() {
                commands.spawn(
                    AudioPlayer::new(asset_server.load("audio/explosionCrunch_000.ogg"))
                );
                player_life.count -= 1;
                protect_timer.timer.reset();
                println!("Player hit! Lives remaining: {}", player_life.count);
                break;
            }
        }
    }
}
