use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player;

const WALK_SPEED: f32 = 60.0;
const SPRINT_SPEED: f32 = 120.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement_system);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 1.5, 5.0)),
    ));
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };

    // INPUT
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight)
    {
        direction.y -= 1.0;
    }

    if direction.length_squared() > 1.0 {
        direction = direction.normalize();
    }

    let is_sprinting =
        keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight);
    let speed = if is_sprinting {
        SPRINT_SPEED
    } else {
        WALK_SPEED
    };

    transform.translation += direction * speed * time.delta_seconds();
}
