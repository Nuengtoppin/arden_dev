
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;
use crate::engine::config::PlayerCameraSettings;

pub fn setup_camera(mut commands: Commands) {
    // Start roughly at center of top layer, facing origin
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(200.0, 150.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn player_look_when_locked(
    mut mouse: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<Camera>>,
    settings: Res<PlayerCameraSettings>,
) {
    if !settings.cursor_locked { return; }
    let mut t = camera.single_mut();
    for ev in mouse.read() {
        let yaw = -ev.delta.x * 0.002;
        let pitch = -ev.delta.y * 0.002;
        let (mut y, mut p, _) = t.rotation.to_euler(EulerRot::YXZ);
        p = (p + pitch).clamp(-1.54, 1.54);
        y += yaw;
        t.rotation = Quat::from_euler(EulerRot::YXZ, y, p, 0.0);
    }
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    let mut t = camera.single_mut();
    let mut dir = Vec3::ZERO;
    let forward = *t.forward();
    let right = *t.right();
    let up = Vec3::Y;

    if keys.pressed(KeyCode::KeyW) { dir += forward; }
    if keys.pressed(KeyCode::KeyS) { dir -= forward; }
    if keys.pressed(KeyCode::KeyA) { dir -= right; }
    if keys.pressed(KeyCode::KeyD) { dir += right; }
    if keys.pressed(KeyCode::Space) { dir += up; }
    if keys.pressed(KeyCode::ControlLeft) { dir -= up; }

    let mut speed = 8.0;
    if keys.pressed(KeyCode::ShiftLeft) && !keys.pressed(KeyCode::KeyF) {
        speed *= 3.0;
    }

    if dir.length_squared() > 0.0 {
        t.translation += dir.normalize() * speed * time.delta_seconds();
    }
}

pub fn toggle_cursor(
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
    mut settings: ResMut<PlayerCameraSettings>,
) {
    if keys.pressed(KeyCode::ShiftLeft) && keys.just_pressed(KeyCode::KeyF) {
        let mut window = windows.single_mut();
        let locked = matches!(window.cursor.grab_mode, CursorGrabMode::Locked);
        window.cursor.grab_mode = if locked { CursorGrabMode::None } else { CursorGrabMode::Locked };
        window.cursor.visible = locked;
        settings.cursor_locked = !locked;
    }
}
