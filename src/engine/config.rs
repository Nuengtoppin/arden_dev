
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerCameraSettings {
    pub cursor_locked: bool,
}

// World sizes (treat as meters)
pub const OCTO_SIZE: IVec3 = IVec3::new(64, 64, 64);      // octochunk = 64^3 vox
pub const SUB_SIZE:  IVec3 = IVec3::new(32, 32, 32);      // subchunk = 32^3 vox

// Sector dimensions in octochunks: 64x16x64 (X,Z,Y with Y vertical)
pub const SECTOR_DIM_OCTO: IVec3 = IVec3::new(64, 16, 64);

// Chunk layer: 8x8x8 octo
pub const CHUNK_DIM_OCTO: IVec3 = IVec3::new(8, 8, 8);

// Layout: 3x3 sectors per layer, 2 layers (0 lower, 1 upper)
pub const SECTORS_PER_AXIS: IVec2 = IVec2::new(3, 3);
pub const LAYERS: i32 = 2;

// Derived: world-space size of one octochunk cube
pub const OCTO_SIZE_WS: Vec3 = Vec3::new(64.0, 64.0, 64.0);

// World-space size of sector
pub fn sector_size_ws() -> Vec3 {
    Vec3::new(
        OCTO_SIZE_WS.x * SECTOR_DIM_OCTO.x as f32,
        OCTO_SIZE_WS.y * SECTOR_DIM_OCTO.y as f32,
        OCTO_SIZE_WS.z * SECTOR_DIM_OCTO.z as f32,
    )
}

// World-space size of chunk (8x8x8 octo)
pub fn chunk_size_ws() -> Vec3 {
    Vec3::new(
        OCTO_SIZE_WS.x * CHUNK_DIM_OCTO.x as f32,
        OCTO_SIZE_WS.y * CHUNK_DIM_OCTO.y as f32,
        OCTO_SIZE_WS.z * CHUNK_DIM_OCTO.z as f32,
    )
}
