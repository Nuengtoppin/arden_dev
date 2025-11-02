//! Engine sizes & constants (X|Z|Y ordering)
use bevy::prelude::*;

pub const SUB_SIZE: UVec3   = UVec3::new(32,32,32);
pub const OCTO_DIM: UVec3   = UVec3::new(8,8,8);
pub const CHUNK_SIZE: UVec3 = UVec3::new(512,512,512);
// Sector: 8×8×2 chunks → 4096×4096×1024 voxels total
pub const SECTOR_CHUNKS: UVec3 = UVec3::new(8,2,8); // (x|y|z), y is vertical half