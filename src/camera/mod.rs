use bevy::prelude::*;
use bevy::input::mouse::{ MouseMotion, MouseWheel };
use crate::npc::Npc;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(Update, camera_move);
    }
}

// ── Components ───────────────────────────────────────────────────

#[derive(Component)]
pub struct MyCamera;

#[derive(Component)]
pub struct OrbitCamera {
    pub zoom_level: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_speed: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub orbit_speed: f32,
    pub mouse_sensitivity: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            zoom_level: 15.0,
            min_zoom: 5.0,
            max_zoom: 25.0,
            zoom_speed: 20.0,
            min_pitch: (5_f32).to_radians(),
            max_pitch: (80_f32).to_radians(),
            orbit_speed: 2.0,
            mouse_sensitivity: 0.003,
        }
    }
}

// ── Systems ──────────────────────────────────────────────────────

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        MyCamera,
        OrbitCamera::default(),
    ));
}

fn camera_move(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut mouse_motion: MessageReader<MouseMotion>,
    player_query: Query<&Transform, (With<Npc>, Without<MyCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut OrbitCamera), With<MyCamera>>,
    time: Res<Time>
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok((mut camera_transform, mut orbit)) = camera_query.single_mut() else {
        return;
    };

    // ── Zoom ─────────────────────────────────────────────────────
    let scroll: f32 = mouse_wheel
        .read()
        .map(|e| e.y)
        .sum();
    orbit.zoom_level -= scroll * orbit.zoom_speed * time.delta_secs();
    orbit.zoom_level = orbit.zoom_level.clamp(orbit.min_zoom, orbit.max_zoom);

    // ── Input deltas ─────────────────────────────────────────────
    let mouse_delta: Vec2 = if mouse_button.pressed(MouseButton::Right) {
        mouse_motion
            .read()
            .map(|e| e.delta)
            .sum()
    } else {
        mouse_motion.clear();
        Vec2::ZERO
    };

    let mut yaw_delta = mouse_delta.x * orbit.mouse_sensitivity;
    if keyboard.pressed(KeyCode::KeyQ) {
        yaw_delta += orbit.orbit_speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyE) {
        yaw_delta -= orbit.orbit_speed * time.delta_secs();
    }

    let pitch_delta = mouse_delta.y * orbit.mouse_sensitivity;

    // ── Rotate offset around player ───────────────────────────────
    let relative_pos = camera_transform.translation - player_transform.translation;
    let yaw_rotation = Quat::from_rotation_y(-yaw_delta);
    let right = camera_transform.right().as_vec3();
    let pitch_rotation = Quat::from_axis_angle(right, -pitch_delta);
    let mut new_offset = yaw_rotation * pitch_rotation * relative_pos;

    // ── Clamp vertical angle ──────────────────────────────────────
    let horizontal = Vec3::new(new_offset.x, 0.0, new_offset.z).length();
    let current_pitch = new_offset.y.atan2(horizontal);
    let clamped_pitch = current_pitch.clamp(orbit.min_pitch, orbit.max_pitch);

    let horizontal_dir = Vec3::new(new_offset.x, 0.0, new_offset.z).normalize();
    new_offset = horizontal_dir * clamped_pitch.cos() + Vec3::Y * clamped_pitch.sin();

    // ── Apply ─────────────────────────────────────────────────────
    camera_transform.translation =
        player_transform.translation + new_offset.normalize() * orbit.zoom_level;
    camera_transform.look_at(player_transform.translation, Vec3::Y);
}
