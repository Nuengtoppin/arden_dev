# Arden

Bevy-based 3D sandbox demonstrating a sector/chunk world layout with debug grid overlays and an in-game debug UI.

## Requirements
- Rust (stable toolchain) via rustup
- A working graphics backend (wgpu) on your platform

## Quick Start
- Run: `cargo run`
- Release build: `cargo build --release`

## Controls
- Move: `W/A/S/D`  Elevation: `Space` / `LeftCtrl`
- Sprint: `LeftShift` (unless held with `F`)
- Toggle cursor lock: `LeftShift` + `F`
- Cycle grids: `F2`  |  Toggle sector grid: `LeftShift` + `F2`

## Project Structure
- `src/main.rs` — App setup: plugins, resources, systems.
- `src/engine/config.rs` — Constants, sizes, and resource types.
- `src/gameplay/camera.rs` — Camera spawn, movement, and look.
- `src/world/sectors/mod.rs` — Sector entities, world/sector math, sector gizmos.
- `src/world/grid/mod.rs` — Grid modes and gizmo drawers (chunk / octo / subchunk).
- `src/debug/ui.rs` — Egui debug overlay (FPS, position, sector, controls).

## Development
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Tests: `cargo test` (none yet; add for pure functions like `world_to_sector`)

## Notes
- The camera starts near the upper layer, looking toward the origin.
