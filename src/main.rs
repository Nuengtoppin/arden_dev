//! Arden Bootstrap — v0.1
//! Layers: CORE → VECTOR → ASPECTROLOG → BUTLER → HAOS → INTERFACE

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use engine::compat::*; // <-- используем слой совместимости

mod engine {
    pub mod config;
    pub mod compat;
}
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
    app.add_plugins((
        gameplay::camera::CameraControllerPlugin,
        gameplay::input::InputPlugin,
    ));

    // Simple DevMenu pane
    app.add_plugins(ui::devmenu::DevMenuPlugin);

    // Debug UI (FPS/coords)
    app.add_plugins(debug::ui::DebugUiPlugin);

    // Grid overlay stub
    app.add_plugins(world::grid::GridOverlayPlugin);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Используем compat-слой — вместо устаревших shape::Plane и ручного света/камеры
    spawn_basic_camera(&mut commands);
    spawn_basic_light(&mut commands);
    spawn_basic_floor(&mut commands, &mut meshes, &mut materials);
}
