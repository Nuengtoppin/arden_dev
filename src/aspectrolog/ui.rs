//! Aspectrolog UI (egui) — stub
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Resource, Default)]
pub struct AspectUiState { pub open: bool }

pub struct AspectUiPlugin;
impl Plugin for AspectUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AspectUiState>()
           .add_systems(Update, ui_panel);
    }
}

fn ui_panel(mut egui_ctx: EguiContexts, mut state: ResMut<AspectUiState>) {
    if !state.open { return; }
    egui::Window::new("Aspectrolog").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("DB stub");
        ui.separator();
        ui.label("Add / List aspects — TODO");
    });
}