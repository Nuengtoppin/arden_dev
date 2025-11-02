//! Arden Bootstrap — v0.1
//! Layers: CORE → VECTOR → ASPECTROLOG → BUTLER → HAOS → INTERFACE

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod engine { pub mod config; }
mod world { pub mod coords; pub mod grid; }
mod vector { pub mod map; pub mod field; }
mod aspectrolog { pub mod db; pub mod ui; }
mod butler { pub mod core; pub mod ui; }
mod haos { pub mod state; pub mod logic; }
mod gameplay { pub mod camera; pub mod input; }
mod ui { pub mod devmenu; }
mod debug { pub mod ui; }

fn main() {
    // Logger
    tracing_subscriber::fmt()
        .with_env_filter("info,wgpu_core=warn,bevy_render=warn")
        .init();

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        EguiPlugin,
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin,
    ));

    // Core scene
    app.add_systems(Startup, setup);
    // Camera controller & input
    app.add_plugins((gameplay::camera::CameraControllerPlugin, gameplay::input::InputPlugin));
    // Simple DevMenu pane
    app.add_plugins(ui::devmenu::DevMenuPlugin);
    // Debug UI (FPS/coords) stub
    app.add_plugins(debug::ui::DebugUiPlugin);
    // Grid overlay stub (hook points only)
    app.add_plugins(world::grid::GridOverlayPlugin);

    app.run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(12.0, 10.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { shadows_enabled: false, ..default() },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ground plane (visual cue)
    commands.spawn(PbrBundle {
        mesh: Mesh::from(shape::Plane { size: 8.0 }).into(),
        material: StandardMaterial { base_color: Color::rgb(0.15, 0.15, 0.18), perceptual_roughness: 0.9, ..default() },
        ..default()
    });
}