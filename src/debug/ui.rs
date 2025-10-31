
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::engine::config::*;
use crate::world::sectors::*;
use crate::world::grid::*;

pub fn debug_overlay(
    mut contexts: EguiContexts,
    time: Res<Time>,
    camera: Query<&Transform, With<Camera>>,
    player: Res<PlayerCameraSettings>,
    grid: Res<GridSettings>,
) {
    let ctx = contexts.ctx_mut();
    let t = if let Ok(t) = camera.get_single() { t } else { return; };
    let pos = t.translation;
    let (sx, sz, layer) = world_to_sector(pos);

    egui::Window::new("Arden Debug")
        .resizable(false)
        .default_width(360.0)
        .show(ctx, |ui| {
            ui.label(format!("FPS: {:.0}", 1.0 / time.delta_seconds().max(1e-6)));
            ui.separator();
            ui.label(format!("Pos:  x={:.2}  y={:.2}  z={:.2}", pos.x, pos.y, pos.z));
            ui.label(format!("Sector: ({},{},{})", sx, sz, layer));
            let mode_str = match grid.mode {
                0 => "OFF",
                1 => "CHUNK",
                2 => "CHUNK + OCTO",
                3 => "CHUNK + OCTO + SUB",
                _ => "?",
            };
            ui.label(format!("Grid Mode: {}", mode_str));
            ui.label(format!("Cursor Locked: {}", player.cursor_locked));
            ui.separator();
            ui.label("Controls:");
            ui.label("  WASD move, Space/Ctrl up/down, Shift = sprint");
            ui.label("  Shift+F = toggle cursor lock");
            ui.label("  F2 = cycle grids | Shift+F2 = sector grid");
        });
}
