use bevy::prelude::*;

#[derive(Component)]
pub struct FlyCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,      // vox/s
    pub fast_speed: f32, // vox/s with Shift
    pub zoom: f32,
}

pub struct CameraControllerPlugin;
impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera_if_missing)
           .add_systems(Update, (mouse_look_alt, move_wasd, wheel_zoom));
    }
}

fn spawn_camera_if_missing(mut commands: Commands, q: Query<Entity, With<Camera3d>>) {
    if q.is_empty() {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(12.0, 10.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
            FlyCamera { yaw: 0.0, pitch: -0.3, speed: 60.0, fast_speed: 120.0, zoom: 1.0 },
        ));
    }
}

fn mouse_look_alt(
    mut q: Query<(&mut Transform, &mut FlyCamera)>,
    mut ev_motion: EventReader<MouseMotion>,
    kb: Res<ButtonInput<KeyCode>>,
) {
    let holding_alt = kb.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);
    if !holding_alt { ev_motion.clear(); return; }

    let sens = 0.002;
    let mut delta = Vec2::ZERO;
    for e in ev_motion.read() { delta += e.delta; }

    for (mut tf, mut cam) in q.iter_mut() {
        cam.yaw   -= delta.x * sens;
        cam.pitch -= delta.y * sens;
        cam.pitch = cam.pitch.clamp(-1.54, 1.54);
        let rot = Quat::from_axis_angle(Vec3::Y, cam.yaw) * Quat::from_axis_angle(Vec3::X, cam.pitch);
        tf.rotation = rot;
    }
}

fn move_wasd(
    time: Res<Time>,
    kb: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut Transform, &FlyCamera)>,
) {
    let dt = time.delta_seconds();
    for (mut tf, cam) in q.iter_mut() {
        let mut dir = Vec3::ZERO;
        if kb.pressed(KeyCode::KeyW) { dir.z -= 1.0; }
        if kb.pressed(KeyCode::KeyS) { dir.z += 1.0; }
        if kb.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
        if kb.pressed(KeyCode::KeyD) { dir.x += 1.0; }
        if kb.pressed(KeyCode::Space) { dir.y += 1.0; }
        if kb.pressed(KeyCode::ControlLeft) || kb.pressed(KeyCode::ControlRight) { dir.y -= 1.0; }

        if dir.length_squared() > 0.0 {
            dir = dir.normalize();
            let speed = if kb.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) { cam.fast_speed } else { cam.speed };
            // move in camera space (yaw only)
            let forward = tf.forward();
            let right = tf.right();
            let up = Vec3::Y;
            let world_move = (forward * dir.z + right * dir.x + up * dir.y) * speed * dt;
            tf.translation += world_move;
        }
    }
}

fn wheel_zoom(mut ev: EventReader<MouseWheel>, mut q: Query<&mut Transform, With<FlyCamera>>) {
    let mut scroll = 0.0f32;
    for e in ev.read() { scroll += e.y as f32; }
    if scroll.abs() < f32::EPSILON { return; }
    for mut tf in q.iter_mut() {
        // simple dolly along forward
        tf.translation += tf.forward() * scroll * 0.5;
    }
}