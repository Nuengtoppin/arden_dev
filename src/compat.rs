//! Arden Engine Compatibility Layer
//! Универсальный слой для стабилизации API между версиями Bevy.
//! Используется для того, чтобы не править код по всему проекту при обновлении движка.

pub use bevy::prelude::*;
pub use bevy::input::mouse::{MouseMotion, MouseWheel};
pub use bevy_math::primitives::Plane3d;
pub use bevy::pbr::NotShadowCaster;

/// Псевдоним для стандартного цвета/материала, чтобы не зависеть от будущих изменений.
pub type MatHandle = Handle<StandardMaterial>;
pub type MeshHandle = Handle<Mesh>;

/// Универсальная обёртка над спавном камеры (на случай изменений API)
pub fn spawn_basic_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(12.0, 10.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Универсальная обёртка для стандартного источника света
pub fn spawn_basic_light(commands: &mut Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Утилита для простого квадрата-пола (вместо устаревшего shape::Plane)
pub fn spawn_basic_floor(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let quad_handle = meshes.add(Mesh::from(bevy::render::mesh::shape::Quad::new(Vec2::new(16.0, 16.0))));
    let mat_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.15, 0.15, 0.18),
        perceptual_roughness: 0.9,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: quad_handle,
            material: mat_handle,
            transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::X, -std::f32::consts::FRAC_PI_2)),
            ..default()
        },
        NotShadowCaster,
    ));
}
