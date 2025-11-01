use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode, WindowPlugin};

mod gameplay;

use gameplay::camera::GameplayCameraPlugin;
use gameplay::player::PlayerPlugin;

fn main() {
    App::new()
        // WINDOW
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Arden_Mvp v1.0".to_string(),
                resolution: (1280.0, 720.0).into(),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((PlayerPlugin, GameplayCameraPlugin))
        .run();
}
