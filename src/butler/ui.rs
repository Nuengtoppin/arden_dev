//! Butler UI — stub
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

#[derive(Resource, Default)]
pub struct ButlerUiState { pub open: bool }

pub struct ButlerUiPlugin;
impl Plugin for ButlerUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButlerUiState>()
           .add_systems(Update, ui_panel);
    }
}

fn ui_panel(mut egui_ctx: EguiContexts, mut state: ResMut<ButlerUiState>) {
    if !state.open { return; }
    egui::Window::new("Butler").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("Status panel — TODO");
    });
}