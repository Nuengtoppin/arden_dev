//! Addressing notation X|Z|Y — canonical stubs
#[derive(Clone, Copy, Debug)]
pub struct SectorCoord { pub sx: i32, pub sz: i32, pub sy: i32 }

#[derive(Clone, Copy, Debug)]
pub struct ChunkIdx  { pub x:u16, pub z:u16, pub y:u16 }
#[derive(Clone, Copy, Debug)]
pub struct OctoIdx   { pub x:u16, pub z:u16, pub y:u16 }
#[derive(Clone, Copy, Debug)]
pub struct SubIdx    { pub x:u8,  pub z:u8,  pub y:u8 }
#[derive(Clone, Copy, Debug)]
pub struct VoxelIdx  { pub x:u8,  pub z:u8,  pub y:u8 }

// TODO: implement (world→sector→chunk→octo→sub→voxel) per spec