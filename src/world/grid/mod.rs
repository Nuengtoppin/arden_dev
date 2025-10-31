
use bevy::prelude::*;
use crate::engine::config::*;
use crate::world::sectors::*;

#[derive(Resource, Default)]
pub struct GridSettings {
    // 0 Off, 1 Chunk, 2 Chunk+Octo, 3 Chunk+Octo+Sub
    pub mode: u8,
}

pub fn cycle_grid_mode(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<GridSettings>) {
    if keys.just_pressed(KeyCode::F2) && !keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        settings.mode = (settings.mode + 1) % 4;
        info!("Grid mode -> {}", settings.mode);
    }
}

// Draw only for the active sector under the camera
pub fn draw_active_sector_grids(
    settings: Res<GridSettings>,
    mut gizmos: Gizmos,
    camera_q: Query<&Transform, With<Camera>>,
) {
    if settings.mode == 0 { return; }

    let cam = if let Ok(t) = camera_q.get_single() { t } else { return; };
    let (sx, sz, layer) = world_to_sector(cam.translation);

    let sector_origin = sector_origin_ws(sx, sz, layer);

    // chunk grid in this sector
    if settings.mode >= 1 {
        let chunk_size = crate::engine::config::chunk_size_ws();
        let chunks_x = SECTOR_DIM_OCTO.x / CHUNK_DIM_OCTO.x;
        let chunks_y = SECTOR_DIM_OCTO.y / CHUNK_DIM_OCTO.y;
        let chunks_z = SECTOR_DIM_OCTO.z / CHUNK_DIM_OCTO.z;
        for cy in 0..chunks_y {
            for cz in 0..chunks_z {
                for cx in 0..chunks_x {
                    let min = sector_origin + Vec3::new(
                        cx as f32 * chunk_size.x,
                        cy as f32 * chunk_size.y,
                        cz as f32 * chunk_size.z,
                    );
                    let center = min + chunk_size * 0.5;
                    gizmos.cuboid(Transform::from_translation(center).with_scale(chunk_size), Color::rgb(1.0, 0.5, 0.1));
                }
            }
        }
    }

    // octochunk grid inside sector
    if settings.mode >= 2 {
        let octo_size = OCTO_SIZE_WS;
        for oy in 0..SECTOR_DIM_OCTO.y {
            for oz in 0..SECTOR_DIM_OCTO.z {
                for ox in 0..SECTOR_DIM_OCTO.x {
                    let min = sector_origin + Vec3::new(
                        ox as f32 * octo_size.x,
                        oy as f32 * octo_size.y,
                        oz as f32 * octo_size.z,
                    );
                    let center = min + octo_size * 0.5;
                    gizmos.cuboid(Transform::from_translation(center).with_scale(octo_size), Color::rgb(1.0, 0.2, 0.2));
                }
            }
        }
    }

    // subchunk grid inside octochunks
    if settings.mode >= 3 {
        let sub = Vec3::new(SUB_SIZE.x as f32, SUB_SIZE.y as f32, SUB_SIZE.z as f32);
        let sub_x = (OCTO_SIZE.x / SUB_SIZE.x) as i32;
        let sub_y = (OCTO_SIZE.y / SUB_SIZE.y) as i32;
        let sub_z = (OCTO_SIZE.z / SUB_SIZE.z) as i32;

        for oy in 0..SECTOR_DIM_OCTO.y {
            for oz in 0..SECTOR_DIM_OCTO.z {
                for ox in 0..SECTOR_DIM_OCTO.x {
                    let octo_min = sector_origin + Vec3::new(
                        ox as f32 * OCTO_SIZE_WS.x,
                        oy as f32 * OCTO_SIZE_WS.y,
                        oz as f32 * OCTO_SIZE_WS.z,
                    );
                    for sy in 0..sub_y {
                        for sz_ in 0..sub_z {
                            for sx_ in 0..sub_x {
                                let min = octo_min + Vec3::new(
                                    sx_ as f32 * sub.x,
                                    sy as f32 * sub.y,
                                    sz_ as f32 * sub.z,
                                );
                                let center = min + sub * 0.5;
                                gizmos.cuboid(Transform::from_translation(center).with_scale(sub), Color::rgb(0.2, 0.5, 1.0));
                            }
                        }
                    }
                }
            }
        }
    }
}
