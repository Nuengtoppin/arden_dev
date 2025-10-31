
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod engine { pub mod config; }
mod world { pub mod sectors; pub mod grid; }
mod gameplay { pub mod camera; }
mod debug { pub mod ui; }

use engine::config::*;
use world::sectors::*;
use world::grid::*;
use gameplay::camera::*;
use debug::ui::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.06, 0.07)))
        .insert_resource(PlayerCameraSettings::default())
        .insert_resource(GridSettings::default())
        .insert_resource(SectorGridSettings::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup_env, setup_camera, setup_sectors))
        .add_systems(Update, (
            toggle_cursor,
            player_movement,
            player_look_when_locked,
            toggle_sector_grid,
            cycle_grid_mode,
            draw_sector_grid,
            draw_active_sector_grids,
            debug_overlay,
        ))
        .run();
}

// ---------- env (ground, light) ----------
fn setup_env(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Grey ground plane under the lower layer
    let plane_handle = meshes.add(Mesh::from(shape::Plane { size: 8000.0, ..default() }));

    commands.spawn(PbrBundle {
        mesh: plane_handle,
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.12, 0.15, 0.18),
            perceptual_roughness: 1.0,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, -5.0, 0.0),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 18.0, 8.0),
        ..default()
    });
}
