use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy_egui::{EguiContexts, egui};

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_debug);
    }
}

fn show_debug(mut egui_ctx: EguiContexts, diag: Res<DiagnosticsStore>) {
    let fps = diag.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.smoothed()).unwrap_or(0.0);
    egui::TopBottomPanel::top("debug_bar").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label(format!("FPS: {fps:.1}"));
            ui.separator();
            ui.label("Coords: X|Z|Y — TODO");
        });
    });
}