use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Enemy {
    dirction: Vec2,
}

#[derive(Component)]
pub struct Star;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMIES_SPEED: f32 = 200.0;
pub const ENEMIES_SIZE: f32 = 64.0;
pub const NUMBER_OF_STAR: usize = 10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemies, spawn_star))
        .add_systems(Update, (
            player_movement,
            confine_player_movement,
            enemies_movement,
            update_enemy_direction,
            confine_enemy_movement,
            enemy_hit_player,
        ))
        .run();
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ball_blue_large.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn player_movement(
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

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    if let Ok(mut player_transform) = player_query.single_mut() {
        let (x_min, x_max, y_min, y_max) = get_bounds(window, PLAYER_SIZE);

        player_transform.translation.x = player_transform.translation.x.clamp(x_min, x_max);
        player_transform.translation.y = player_transform.translation.y.clamp(y_min, y_max);
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    spawn_sprites(&mut commands, window, NUMBER_OF_ENEMIES, None, |x, y| (
        Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
        Transform::from_xyz(x, y, 0.0),
        Enemy {
            dirction: Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5).normalize(),
        },
    ));
}

pub fn spawn_star(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    spawn_sprites(&mut commands, window, NUMBER_OF_STAR, None, |x, y| (
        Sprite::from_image(asset_server.load("sprites/star.png")),
        Transform::from_xyz(x, y, 0.0),
        Star,
    ));
}

pub fn enemies_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.dirction.x, enemy.dirction.y, 0.0);
        transform.translation += direction * ENEMIES_SPEED * time.delta_secs();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let (x_min, x_max, y_min, y_max) = get_bounds(window, ENEMIES_SIZE);
    let mut direction_change = false;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let pos = transform.translation;

        if pos.x <= x_min || pos.x >= x_max {
            enemy.dirction.x *= -1.0;
            direction_change = true;
        }
        if pos.y <= y_min || pos.y >= y_max {
            enemy.dirction.y *= -1.0;
            direction_change = true;
        }

        if direction_change {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 { sound_effect_1 } else { sound_effect_2 };
            commands.spawn(AudioPlayer::new(sound_effect));
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    for (mut transform, _enemy) in enemy_query.iter_mut() {
        let (x_min, x_max, y_min, y_max) = get_bounds(window, ENEMIES_SIZE);

        transform.translation.x = transform.translation.x.clamp(x_min, x_max);
        transform.translation.y = transform.translation.y.clamp(y_min, y_max);
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        for enemy in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMIES_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(AudioPlayer::new(sound_effect));
                commands.entity(player_entity).despawn();
            }
        }
    }
}

//private service
fn get_bounds(window: &Window, sprite_size: f32) -> (f32, f32, f32, f32) {
    let half = sprite_size / 2.0;
    (
        -window.width() / 2.0 + half,
        window.width() / 2.0 - half,
        -window.height() / 2.0 + half,
        window.height() / 2.0 - half,
    )
}

fn random_position(window: &Window) -> (f32, f32) {
    ((random::<f32>() - 0.5) * window.width(), (random::<f32>() - 0.5) * window.height())
}

fn spawn_sprites<F, B>(
    commands: &mut Commands,
    window: &Window,
    count: usize,
    position: Option<(f32, f32)>,
    mut bundle_fn: F
)
    where B: Bundle, F: FnMut(f32, f32) -> B
{
    for _ in 0..count {
        let (x, y) = position.unwrap_or_else(|| random_position(window));
        commands.spawn(bundle_fn(x, y));
    }
}
