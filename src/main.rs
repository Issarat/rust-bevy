use bevy::{ prelude::*, window::WindowResolution, input::mouse::{ MouseMotion, MouseWheel } };

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(get_window_config()))
        .add_systems(Startup, (setup, light_setup, wall_setup, player_setup))
        .add_systems(Update, camera_move)
        .run();
}
#[derive(Component)]
struct Player;

#[derive(Component)]
struct MyCamera;

#[derive(Component)]
struct OrbitCamera {
    pub zoom_level: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_speed: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            zoom_level: 15.0,
            min_zoom: 5.0,
            max_zoom: 25.0,
            zoom_speed: 20.0,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        MyCamera,
        OrbitCamera::default(),
    ));
}

fn light_setup(mut commands: Commands) {
    commands.spawn((
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn wall_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.7, 0.2))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}

fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.4, 0.9))),
        Transform::from_xyz(0.0, 0.5, 3.0),
        Player,
    ));
}

fn get_window_config() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Game Ui".into(),
            resolution: WindowResolution::new(1280, 720),
            resize_constraints: WindowResizeConstraints {
                min_width: 800.0,
                min_height: 600.0,
                ..default()
            },
            ..default()
        }),
        ..default()
    }
}

fn camera_move(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut mouse_motion: MessageReader<MouseMotion>,
    player_query: Query<&Transform, (With<Player>, Without<MyCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut OrbitCamera), With<MyCamera>>,
    time: Res<Time>
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok((mut camera_transform, mut orbit)) = camera_query.single_mut() else {
        return;
    };

    let scroll: f32 = mouse_wheel
        .read()
        .map(|e| e.y)
        .sum();

    let relative_pos = camera_transform.translation - player_transform.translation;

    // zoom
    orbit.zoom_level -= scroll * orbit.zoom_speed * time.delta_secs();
    orbit.zoom_level = orbit.zoom_level.clamp(orbit.min_zoom, orbit.max_zoom);

    // collect mouse delta
    let mouse_delta: Vec2 = if
        keyboard.pressed(KeyCode::ControlLeft) &&
        mouse_button.pressed(MouseButton::Right)
    {
        mouse_motion
            .read()
            .map(|e| e.delta)
            .sum()
    } else {
        mouse_motion.clear();
        Vec2::ZERO
    };

    // Q/E orbit on Y axis
    let speed = 2.0;
    let mut yaw_delta = mouse_delta.x * 0.003; // free look horizontal
    if keyboard.pressed(KeyCode::KeyQ) {
        yaw_delta += speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyE) {
        yaw_delta -= speed * time.delta_secs();
    }

    let pitch_delta = mouse_delta.y * 0.003; // free look vertical

    // rotate the offset around the player
    let yaw_rotation = Quat::from_rotation_y(-yaw_delta);
    let right = camera_transform.right().as_vec3();
    let pitch_rotation = Quat::from_axis_angle(right, -pitch_delta);

    let mut new_offset = yaw_rotation * pitch_rotation * relative_pos;

    // clamp vertical angle
    let min_pitch: f32 = (5_f32).to_radians();
    let max_pitch: f32 = (80_f32).to_radians();

    let horizontal = Vec3::new(new_offset.x, 0.0, new_offset.z).length();
    let current_pitch = new_offset.y.atan2(horizontal);
    let clamped_pitch = current_pitch.clamp(min_pitch, max_pitch);

    let horizontal_dir = Vec3::new(new_offset.x, 0.0, new_offset.z).normalize();
    new_offset = horizontal_dir * clamped_pitch.cos() + Vec3::Y * clamped_pitch.sin();

    camera_transform.translation =
        player_transform.translation + new_offset.normalize() * orbit.zoom_level;
    camera_transform.look_at(player_transform.translation, Vec3::Y);
}
