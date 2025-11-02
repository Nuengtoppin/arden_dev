use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use crate::gameplay::input::DevMenuState;
use crate::world::grid::GridSettings;

pub struct DevMenuPlugin;
impl Plugin for DevMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, devmenu_ui);
    }
}

fn devmenu_ui(mut egui_ctx: EguiContexts, mut dev: ResMut<DevMenuState>, mut grid: ResMut<GridSettings>) {
    if !dev.open { return; }
    egui::Window::new("Dev Tools").show(egui_ctx.ctx_mut(), |ui| {
        ui.collapsing("Tools", |ui| {
            if ui.button("Toggle Inspector (commented)").clicked() {
                // TODO: hook bevy-inspector-egui here later
            }
        });
        ui.collapsing("Vector", |_| {});
        ui.collapsing("Voxel", |_| {});
        ui.collapsing("Optimization (DTO/LOD/SVO)", |_| {});
        ui.collapsing("PFO Filter", |_| {});
        ui.separator();
        ui.label("Grid modes:");
        ui.horizontal(|ui| {
            ui.checkbox(&mut grid.show_chunks, "Chunks");
            ui.checkbox(&mut grid.show_octos, "Octo");
            ui.checkbox(&mut grid.show_subs, "Sub");
            ui.checkbox(&mut grid.show_sector_map, "Sector map");
        });
        if ui.button("Close (Tab)").clicked() { dev.open = false; }
    });
}