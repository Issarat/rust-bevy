use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score::default())
        .insert_resource(EnemySpawnTimer::default())
        .insert_resource(PlayerLife::default())
        .insert_resource(PlayerProtectionTime::default())
        .add_message::<GameOver>()
        .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemies, spawn_star))
        .add_systems(Update, (
            player_movement,
            confine_player_movement,
            enemies_movement,
            player_hit_star,
            tick_player_protect_timer,
            blink_player,
            update_enemy_direction,
            confine_enemy_movement,
            update_enemies_movement,
            enemy_hit_player,
            tick_enemies_spawn_timer,
            spawn_enemies_over_time,
            spawn_stars_over_time,
            update_score,
            event_game_over_trigger,
        ))
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

//================================ events ===============================================================

#[derive(Message)]
pub struct GameOver {
    pub final_score: u32,
}

pub fn event_game_over_trigger(
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
            println!("Game Over! Final score: {}", score.value);
            game_over_writer.write(GameOver { final_score: score.value });
        }
        for star in stars_query.iter() {
            commands.entity(star).despawn();
        }
        for enemy in enemies_query.iter() {
            commands.entity(enemy).despawn();
        }
    }
}

//================================ player ===============================================================

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerLife {
    pub count: u8,
}
impl Default for PlayerLife {
    fn default() -> Self {
        Self { count: player::LIFE_COUNT }
    }
}

#[derive(Resource)]
pub struct PlayerProtectionTime {
    pub timer: Timer,
}
impl Default for PlayerProtectionTime {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(player::PROTECT_TIME, TimerMode::Once);
        timer.finish(); // ← pre-finish so player can take damage immediately on game start
        Self { timer }
    }
}

#[derive(Component)]
pub struct BlinkTimer {
    pub timer: Timer,
}

pub mod player {
    pub const SPEED: f32 = 500.0;
    pub const SIZE: f32 = 64.0;
    pub const LIFE_COUNT: u8 = 3;
    pub const PROTECT_TIME: f32 = 1.0;
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ball_blue_large.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
        BlinkTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
    ));
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

        transform.translation += direction * player::SPEED * time.delta_secs();
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
        let (x_min, x_max, y_min, y_max) = get_bounds(window, player::SIZE);
        player_transform.translation.x = player_transform.translation.x.clamp(x_min, x_max);
        player_transform.translation.y = player_transform.translation.y.clamp(y_min, y_max);
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>
) {
    if let Ok(player_transform) = player_query.single() {
        for (stars_entity, stars_transform) in star_query.iter_mut() {
            let distance = player_transform.translation.distance(stars_transform.translation);
            let player_radius = player::SIZE / 2.0;
            let star_radius = stars::SIZE / 2.0;
            if distance < player_radius + star_radius {
                commands.spawn(AudioPlayer::new(asset_server.load("audio/laserLarge_000.ogg")));
                commands.entity(stars_entity).despawn();
                score.value += 1;
            }
        }
    }
}

pub fn tick_player_protect_timer(mut protect_timer: ResMut<PlayerProtectionTime>, time: Res<Time>) {
    protect_timer.timer.tick(time.delta());
}

pub fn blink_player(
    mut player_query: Query<(&mut Visibility, &mut BlinkTimer), With<Player>>,
    player_protect_timer: Res<PlayerProtectionTime>,
    time: Res<Time>
) {
    if let Ok((mut visibility, mut blink)) = player_query.single_mut() {
        if !player_protect_timer.timer.is_finished() {
            // protection active — toggle visibility on each blink tick
            blink.timer.tick(time.delta());
            if blink.timer.just_finished() {
                *visibility = match *visibility {
                    Visibility::Visible => Visibility::Hidden,
                    _ => Visibility::Visible,
                };
            }
        } else {
            // protection over — make sure player is visible and reset blink
            *visibility = Visibility::Visible;
            blink.timer.reset();
        }
    }
}
//================================ enemies ==============================================================

pub mod enemies {
    pub const COUNT: usize = 4;
    pub const SPEED: f32 = 200.0;
    pub const SIZE: f32 = 64.0;
    pub const MAX_COUNT: usize = 6;
    pub const TIME_SPAWNER: f32 = 10.0;
    pub const MAX_SPEED: f32 = 350.0;
}

#[derive(Component)]
pub struct Enemy {
    dirction: Vec2,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(enemies::TIME_SPAWNER, TimerMode::Repeating) }
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
    spawn_sprites(&mut commands, window, enemies::COUNT, None, |x, y| {
        (
            Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                dirction: Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5).normalize(),
            },
        )
    });
}

pub fn enemies_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.dirction.x, enemy.dirction.y, 0.0);
        transform.translation += direction * enemies::SPEED * time.delta_secs();
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
    let (x_min, x_max, y_min, y_max) = get_bounds(window, enemies::SIZE);

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let pos = transform.translation;
        let mut direction_change = false;

        if pos.x <= x_min || pos.x >= x_max {
            enemy.dirction.x *= -1.0;
            direction_change = true;
        }
        if pos.y <= y_min || pos.y >= y_max {
            enemy.dirction.y *= -1.0;
            direction_change = true;
        }

        if direction_change {
            let sound = if random::<f32>() > 0.5 {
                asset_server.load("audio/pluck_001.ogg")
            } else {
                asset_server.load("audio/pluck_002.ogg")
            };
            commands.spawn(AudioPlayer::new(sound));
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
        let (x_min, x_max, y_min, y_max) = get_bounds(window, enemies::SIZE);
        transform.translation.x = transform.translation.x.clamp(x_min, x_max);
        transform.translation.y = transform.translation.y.clamp(y_min, y_max);
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut player_life: ResMut<PlayerLife>,
    mut player_protect_timer: ResMut<PlayerProtectionTime> // ← mut so we can reset
) {
    if let Ok((_, player_transform)) = player_query.single_mut() {
        for enemy in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy.translation);
            let player_radius = player::SIZE / 2.0;
            let enemy_radius = enemies::SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                if player_protect_timer.timer.is_finished() {
                    // ← is_finished() not finished()
                    commands.spawn(
                        AudioPlayer::new(asset_server.load("audio/explosionCrunch_000.ogg"))
                    );
                    player_life.count -= 1;
                    player_protect_timer.timer.reset(); // ← restart protection window
                    println!("Player hit! Lives remaining: {}", player_life.count);
                    break;
                }
            }
        }
    }
}

pub fn tick_enemies_spawn_timer(mut spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    enemies_spawn_timer: Res<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_life: Res<PlayerLife>
) {
    if enemies_spawn_timer.timer.just_finished() && player_life.count > 0 {
        let Ok(window) = window_query.single() else {
            return;
        };
        spawn_sprites(&mut commands, window, 1, None, |x, y| {
            (
                Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
                Transform::from_xyz(x, y, 0.0),
                Enemy {
                    dirction: Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5).normalize(),
                },
            )
        });
    }
}

pub fn update_enemies_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
    score: ResMut<Score>
) {
    if score.is_changed() {
        let speed_bonus = ((score.value / 10) * 10) as f32;
        let speed_move = (enemies::SPEED + speed_bonus).min(enemies::MAX_SPEED);

        for (mut transform, enemy) in enemy_query.iter_mut() {
            let direction = Vec3::new(enemy.dirction.x, enemy.dirction.y, 0.0);
            transform.translation += direction * speed_move * time.delta_secs();
        }
    }
}

//================================ stars ================================================================

pub mod stars {
    pub const COUNT: usize = 10;
    pub const SIZE: f32 = 30.0;
    pub const MAX_COUNT: usize = 10;
}

#[derive(Component)]
pub struct Star;

pub fn spawn_star(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    spawn_sprites(&mut commands, window, stars::COUNT, None, |x, y| {
        (
            Sprite::from_image(asset_server.load("sprites/star.png")),
            Transform::from_xyz(x, y, 0.0),
            Star,
        )
    });
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>,
    player_life: Res<PlayerLife>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let star_is_below_max = star_query.iter().count() < stars::MAX_COUNT;
    if star_is_below_max && player_life.count > 0 {
        let Ok(window) = window_query.single() else {
            return;
        };
        spawn_sprites(&mut commands, window, 1, None, |x, y| {
            (
                Sprite::from_image(asset_server.load("sprites/star.png")),
                Transform::from_xyz(x, y, 0.0),
                Star,
            )
        });
    }
}

//================================ score ================================================================

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}
impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

//================================ helpers ==============================================================

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
