Arden — Agent Guide

Scope
- Applies to the entire repository.

Tech Stack
- Rust 2021 (stable toolchain).
- Bevy 0.13.2, bevy_egui 0.26.0.

Run/Build/Test
- Run: `cargo run`
- Release build: `cargo build --release`
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Tests: `cargo test` (none yet; prefer tests for pure functions when added).

Project Layout
- `src/main.rs` — App setup: plugins, resources, schedules.
- `src/engine/config.rs` — Constants, sizes, shared config/resources.
- `src/gameplay/camera.rs` — Camera spawn, input (WASD, look, cursor lock).
- `src/world/sectors/` — Sector data, spawning, world/sector conversions, gizmos.
- `src/world/grid/` — Grid modes and gizmo drawers (chunk/octo/sub).
- `src/debug/ui.rs` — Egui debug overlay (FPS, pos, sector, controls).

Controls (runtime)
- Move: `W/A/S/D`  Elevation: `Space` / `LeftCtrl`
- Sprint: `LeftShift` (unless held with `F`)
- Toggle cursor lock: `LeftShift` + `F`
- Cycle grids: `F2`  |  Toggle sector grid: `LeftShift` + `F2`

Coding Conventions
- Keep changes minimal and focused; do not introduce unrelated refactors.
- Use `rustfmt` defaults; run `cargo fmt` before submissions.
- Prefer descriptive names over one-letter identifiers.
- Use Bevy logging macros (`info!`, `warn!`, `error!`) rather than `println!`.
- Keep systems small and deterministic; avoid per-frame allocations where feasible.
- Access shared data via `Res`/`ResMut`; prefer `Query::get_single()` only when exactly one is expected.
- When adding features that affect UX, update `src/debug/ui.rs` controls/help text.

Bevy Scheduling
- Add setup to `Startup`, frame logic to `Update`.
- Maintain stable ordering when introducing new systems that depend on existing ones.

World/Sector Invariants
- Sectors per axis: `3 x 3`; Layers: `2` (0 lower, 1 upper).
- Sector dimensions (octochunks): `SECTOR_DIM_OCTO = 64 x 16 x 64`.
- Chunk dimensions (octochunks): `CHUNK_DIM_OCTO = 8 x 8 x 8`.
- `sector_origin_ws(sx, sz, layer)` returns the min-corner in world space.
- `world_to_sector(Vec3)` floors then clamps to valid indices.

Module/File Organization
- Place domain code under `src/<domain>/module.rs` (snake_case).
- Keep public APIs narrow; re-export from module `mod.rs` where appropriate.

Performance Notes
- Use `Gizmos` sparingly; large grid modes are for debug/visualization only.
- Avoid heavy work in tight `Update` loops; cache derived values when possible.

Licensing/Headers
- Do not add license or copyright headers unless explicitly requested.

