//! HAOS state — stub
#[derive(Clone, Copy, Debug)]
pub struct ActiveState {
    pub sector_id: i32,
    pub awake: bool,
    pub last_tick: u64,
}