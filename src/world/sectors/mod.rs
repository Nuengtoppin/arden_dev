
use bevy::prelude::*;
use crate::engine::config::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Sector {
    pub sx: i32,
    pub sz: i32,
    pub layer: i32, // 0 lower, 1 upper
}

#[derive(Resource, Default)]
pub struct SectorGridSettings {
    pub show: bool, // Shift+F2
}

// spawn a 3x3 grid for each of two layers
pub fn setup_sectors(mut commands: Commands) {
    for layer in 0..LAYERS {
        for gz in 0..SECTORS_PER_AXIS.y {
            for gx in 0..SECTORS_PER_AXIS.x {
                let world_origin = sector_origin_ws(gx, gz, layer);
                commands.spawn((
                    Sector { sx: gx, sz: gz, layer },
                    Transform::from_translation(world_origin),
                    Visibility::Visible,
                    Name::new(format!("Sector ({},{},{})", gx, gz, layer)),
                ));
            }
        }
    }
}

// compute origin (min corner) of sector in world space
pub fn sector_origin_ws(sx: i32, sz: i32, layer: i32) -> Vec3 {
    let size = sector_size_ws();
    Vec3::new(
        sx as f32 * size.x,
        layer as f32 * size.y,
        sz as f32 * size.z,
    )
}

// Returns (sx, sz, layer) for a world-space position
pub fn world_to_sector(pos: Vec3) -> (i32, i32, i32) {
    let size = sector_size_ws();
    let sx = (pos.x / size.x).floor() as i32;
    let sz = (pos.z / size.z).floor() as i32;
    let layer = (pos.y / size.y).floor() as i32;
    (sx.clamp(0, SECTORS_PER_AXIS.x - 1),
     sz.clamp(0, SECTORS_PER_AXIS.y - 1),
     layer.clamp(0, LAYERS - 1))
}

// Toggle sector grid
pub fn toggle_sector_grid(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<SectorGridSettings>) {
    if keys.just_pressed(KeyCode::F2) && keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        settings.show = !settings.show;
        info!("Sector Grid: {}", settings.show);
    }
}

// Draw sector boxes
pub fn draw_sector_grid(
    settings: Res<SectorGridSettings>,
    mut gizmos: Gizmos,
) {
    if !settings.show { return; }
    let size = sector_size_ws();
    for layer in 0..LAYERS {
        for gz in 0..SECTORS_PER_AXIS.y {
            for gx in 0..SECTORS_PER_AXIS.x {
                let origin = sector_origin_ws(gx, gz, layer);
                let center = origin + size * 0.5;
                gizmos.cuboid(Transform::from_translation(center).with_scale(size), Color::rgb(1.0, 1.0, 0.2));
            }
        }
    }
}
