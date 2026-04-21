use bevy::prelude::*;
use uuid::Uuid;
use rand::prelude::*;

use crate::GameEntity;

// ── WorldEntity ──────────────────────────────────────────────────

pub struct WorldEntity {
    pub profile: GameEntity<WorldEntity>,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub color: Color,
    pub size: Cuboid,
}

impl Default for WorldEntity {
    fn default() -> Self {
        let mut rng = rand::rng();
        Self {
            profile: GameEntity::new(Uuid::new_v4()),
            pos_x: rng.random_range(-5.0..=5.0),
            pos_y: rng.random_range(0.5..=3.0),
            pos_z: rng.random_range(-5.0..=5.0),
            color: Color::srgb(
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0)
            ),
            size: Cuboid::new(
                rng.random_range(0.5..=3.0),
                rng.random_range(0.5..=3.0),
                rng.random_range(0.5..=3.0)
            ),
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_wall));
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn spawn_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    for _ in 0..3 {
        let wall = WorldEntity::default();

        commands.spawn((
            Mesh3d(meshes.add(wall.size)),
            MeshMaterial3d(materials.add(wall.color)),
            Transform::from_xyz(wall.pos_x, wall.pos_y, wall.pos_z),
        ));
    }
}
