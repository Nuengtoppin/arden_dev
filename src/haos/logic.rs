//! HAOS logic — stub
pub fn tick(state: &mut super::state::ActiveState, tick: u64) {
    state.last_tick = tick;
}