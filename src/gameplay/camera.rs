use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, Window, WindowMode};

use super::player::Player;

pub struct GameplayCameraPlugin;

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct Crosshair;

#[derive(Component)]
struct CameraRig {
    yaw: f32,
    pitch: f32,
    distance: f32,
    focus_height: f32,
}

impl Default for CameraRig {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: -0.35,
            distance: 10.0,
            focus_height: 1.5,
        }
    }
}

impl CameraRig {
    fn local_offset(&self) -> Vec3 {
        let rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        rotation * Vec3::new(0.0, 0.0, self.distance)
    }

    fn look_target(&self) -> Vec3 {
        Vec3::new(0.0, self.focus_height, 0.0)
    }
}

#[derive(Resource, Default)]
struct CursorState {
    hidden: bool,
}

const ROTATION_SENSITIVITY: f32 = 0.005;
const ZOOM_SPEED: f32 = 1.0;
const MIN_ZOOM: f32 = 2.0;
const MAX_ZOOM: f32 = 40.0;

impl Plugin for GameplayCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorState>();
        app.add_systems(Startup, (setup_camera, setup_ui));
        app.add_systems(
            Update,
            (
                update_camera_controls,
                toggle_cursor_and_crosshair,
                toggle_fullscreen,
            ),
        );
    }
}

fn setup_camera(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    let rig = CameraRig::default();
    let offset = rig.local_offset();
    let look_target = rig.look_target();

    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            Camera3dBundle {
                transform: Transform::from_translation(offset).looking_at(look_target, Vec3::Y),
                ..default()
            },
            PlayerCamera,
            rig,
        ));
    });
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
        Name::new("UI Camera"),
    ));

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(6.0),
                height: Val::Px(6.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                margin: UiRect {
                    left: Val::Px(-3.0),
                    top: Val::Px(-3.0),
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 0.3)),
            border_radius: BorderRadius::all(Val::Px(3.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        Crosshair,
    ));
}

fn update_camera_controls(
    mut motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    cursor_state: Res<CursorState>,
    mut query: Query<(&mut Transform, &mut CameraRig), With<PlayerCamera>>,
) {
    let Ok((mut transform, mut rig)) = query.get_single_mut() else {
        motion_events.clear();
        scroll_events.clear();
        return;
    };

    let mut rotation_delta = Vec2::ZERO;
    for event in motion_events.read() {
        rotation_delta += event.delta;
    }

    if rotation_delta != Vec2::ZERO
        && (mouse_buttons.pressed(MouseButton::Middle) || cursor_state.hidden)
    {
        // CAMERA
        rig.yaw -= rotation_delta.x * ROTATION_SENSITIVITY;
        rig.pitch = (rig.pitch - rotation_delta.y * ROTATION_SENSITIVITY).clamp(-1.54, 1.2);
    }

    let mut zoom_delta = 0.0;
    for event in scroll_events.read() {
        let amount = match event.unit {
            MouseScrollUnit::Line => event.y * ZOOM_SPEED,
            MouseScrollUnit::Pixel => event.y * 0.1 * ZOOM_SPEED,
        };
        zoom_delta += amount;
    }

    if zoom_delta.abs() > f32::EPSILON {
        rig.distance = (rig.distance - zoom_delta).clamp(MIN_ZOOM, MAX_ZOOM);
    }

    transform.translation = rig.local_offset();
    transform.look_at(rig.look_target(), Vec3::Y);
}

fn toggle_cursor_and_crosshair(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor_state: ResMut<CursorState>,
    mut crosshair_query: Query<&mut Visibility, With<Crosshair>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyF)
        && (keyboard_input.pressed(KeyCode::ShiftLeft)
            || keyboard_input.pressed(KeyCode::ShiftRight))
    {
        cursor_state.hidden = !cursor_state.hidden;

        if let Ok(mut window) = windows.get_single_mut() {
            window.cursor.visible = !cursor_state.hidden;
            window.cursor.grab_mode = if cursor_state.hidden {
                CursorGrabMode::Confined
            } else {
                CursorGrabMode::None
            };
        }

        if let Ok(mut visibility) = crosshair_query.get_single_mut() {
            *visibility = if cursor_state.hidden {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn toggle_fullscreen(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen,
                WindowMode::BorderlessFullscreen | WindowMode::Fullscreen => WindowMode::Windowed,
                other => other,
            };
        }
    }
}
