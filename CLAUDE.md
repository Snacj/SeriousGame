# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

**BodyQuest** — an educational 2D browser game (ages 10–14) about exploring the human body and fighting diseases. Built in Rust on a **custom 2D engine** (no game framework) using `wgpu` (WebGPU/WebGL2) + `winit`. Ships as both a native binary (for dev) and a WebAssembly bundle (the real target).

## Commands

```bash
# Native dev build/run (fastest iteration)
cargo run                    # runs the `game` bin (src/main.rs -> lib::run())

cargo check                  # type-check
cargo build --release

# Browser build (the actual deployment target — needs wasm-pack)
wasm-pack build --target web # outputs to pkg/ (git-ignored), imported by index.html
npx serve .                  # then open the served URL; needs a WebGL2 browser
```

There are no tests in this repo. `cargo test` does nothing meaningful.

## Critical: assets are compiled in, not loaded at runtime

Every asset is embedded into the binary via `include_bytes!` / `include_str!` at compile time. There is **no runtime filesystem access** (required for the wasm target). Consequence:

- New PNGs → register in `load_sprites()` in `src/engine/texture.rs` with an `include_bytes!` path and a string key. Sprites are referenced everywhere by that string key (e.g. `renderer.draw_sprite("virus", ...)`).
- Maps → `src/game/game.rs` embeds `maps/testMap.json` via `include_bytes!`.
- Shaders → `src/engine/renderer.rs` embeds `shader/shader.wgsl` and `shader/shader_pulse.wgsl` via `include_str!`.
- Editing an asset file requires a **recompile** to take effect.

## Architecture

Two layers, cleanly separated: `engine/` knows nothing about the game; `game/` drives it.

```
main.rs / lib.rs      App + winit event loop, fixed 1/60s timestep (accumulator), wasm entry
  └─ engine::Engine   wgpu device/queue/surface (native + wasm init paths differ)
  └─ engine::Renderer sprite batching (one draw call per texture), camera, shaders
  └─ game::GameContext top-level game wrapper: owns GameState + global health bar
```

**Game loop** (`lib.rs`): fixed-timestep update at 1/60s via an accumulator; `state.update()` runs zero-or-more times per frame, `state.render()` once. The wasm path initializes the engine async and delivers it back through a `winit` user event (`Initialized`).

**State machine** (`src/game/mod.rs`): `GameState` is the core enum — `MainMenu | Playing | Paused | Dialogue | Minigame | Results | Won`. Variants own their data (e.g. `Playing(MyGame)`). `update()` returns `Option<(GameState, f32)>`: a `Some` transitions to the next state and applies the `f32` health delta. State transitions move `MyGame` out via `std::mem::replace(game, MyGame::new(0.0, 0.0))`, so the overworld survives across dialogue/minigame/results. Global health (0.0–1.0) lives in `GameContext`; reaching 1.0 triggers `Won`. Debug keys: `F12` instantly wins, `F9` debug overlay.

**Overworld** (`src/game/game.rs` = `MyGame`): fixed 64×64 tile map (16px tiles), Y-sorted sprite rendering (depth by feet position), AABB collision via `CollisionWorld`. Interactable `Object`s (organs/stations) carry an optional `Interaction` that can launch dialogue and a minigame. Completing a minigame sets `object.completed` and calls `rebuild_collision_map()`.

**Minigames** (`src/game/minigame*.rs`): implement the `Minigame` trait (`update -> MinigameResult { Running | Won{score} | Lost }`, `render`, `on_resize`). They render full-screen, suspending the overworld.

### Adding a minigame (end-to-end)

1. New `src/game/minigame_<name>.rs` implementing `Minigame`; add `pub mod` in `src/game/mod.rs`.
2. Add a `MinigameTrigger` variant in `src/game/dialogue.rs`.
3. Wire the trigger → minigame + outcome title/fact in `make_minigame()` in `src/game/mod.rs`.
4. Register any new textures in `load_sprites()` (`src/engine/texture.rs`).
5. Attach the trigger to an `Object`'s interaction in `MyGame::new()` (`src/game/game.rs`).

## Rendering notes

- Logical resolution is height-based (180px tall, configured in `lib.rs`/`Camera`); the camera scales to the window aspect ratio. Pixel-art uses `Nearest` filtering.
- Three batch layers per frame: world, UI (`draw_sprite_ui`, fixed to camera), and a "pulse" animated layer (separate pipeline + time uniform).
- Solid-color "textures" are generated at runtime via `create_solid_texture` (e.g. `health_green`, `transparent_gray`) — used for UI rectangles and overlays.
- Tiled maps: `src/engine/map.rs` parses Tiled JSON layers; layer custom properties (e.g. solid) drive collision.
