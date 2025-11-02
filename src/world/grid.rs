use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GridSettings {
    pub show_chunks: bool,
    pub show_octos: bool,
    pub show_subs: bool,
    pub show_sector_map: bool, // Shift+F2
}

pub struct GridOverlayPlugin;
impl Plugin for GridOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridSettings>()
           .add_systems(Update, draw_grid_stub);
    }
}

fn draw_grid_stub(settings: Res<GridSettings>) {
    // placeholder: hook for gizmo-based overlay
    // when implemented, use Bevy gizmos to draw thin lines with alpha & radial fade
    let _ = settings;
}