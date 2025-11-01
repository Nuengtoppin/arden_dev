use bevy::prelude::*;

#[derive(Component)]
pub struct Crosshair;

// CAMERA
pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0))
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        })
        .insert(Crosshair)
        .insert(Visibility::Hidden);
}

// CAMERA
pub fn draw_crosshair(
    mut gizmos: Gizmos,
    camera_query: Query<(&Camera, &GlobalTransform, &Visibility), With<Crosshair>>,
) {
    for (camera, camera_transform, visibility) in camera_query.iter() {
        if matches!(visibility, Visibility::Hidden) {
            continue;
        }

        let viewport_size = match camera.logical_viewport_size() {
            Some(size) => size,
            None => continue,
        };

        let viewport_center = viewport_size / 2.0;

        if let Some(ray) = camera.viewport_to_world(camera_transform, viewport_center) {
            let position = ray.get_point(10.0);
            gizmos.sphere(position, Quat::IDENTITY, 0.3, Color::rgba(1.0, 1.0, 1.0, 0.7));
        }
    }
}
