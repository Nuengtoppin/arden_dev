use bevy::prelude::*;
use crate::world::grid::GridSettings;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridSettings>()
           .insert_resource(DevMenuState::default())
           .add_systems(Update, (toggle_grid_modes, toggle_devmenu));
    }
}

#[derive(Resource, Default)]
pub struct DevMenuState { pub open: bool }

fn toggle_grid_modes(mut settings: ResMut<GridSettings>, kb: Res<ButtonInput<KeyCode>>) {
    if kb.just_pressed(KeyCode::F2) && !kb.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        // cycle: off -> chunk -> chunk+octo -> chunk+octo+sub
        let s = &mut *settings;
        if !s.show_chunks { s.show_chunks = true; s.show_octos = false; s.show_subs = false; }
        else if s.show_chunks && !s.show_octos { s.show_octos = true; }
        else if s.show_chunks && s.show_octos && !s.show_subs { s.show_subs = true; }
        else { s.show_chunks = false; s.show_octos = false; s.show_subs = false; }
    }
    // Shift+F2 => sector map
    let shift = kb.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if shift && kb.just_pressed(KeyCode::F2) {
        settings.show_sector_map = !settings.show_sector_map;
    }
}

fn toggle_devmenu(mut state: ResMut<DevMenuState>, kb: Res<ButtonInput<KeyCode>>) {
    if kb.just_pressed(KeyCode::Tab) {
        state.open = !state.open;
    }
}