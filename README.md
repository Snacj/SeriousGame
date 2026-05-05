# 🧬 BodyQuest — Your Body. Your Adventure.

> An educational browser game for children to explore the human body, fight diseases, and learn about organs, health, diet, and vaccines — built with a custom engine in Rust and WebGPU.

---

## 📖 About

**BodyQuest** is a serious game developed as a university project. The player shrinks down and travels through the human body, freeing organs and organ systems from dangerous diseases. Along the way they discover how organs work, where they are located, and how illnesses affect them.

The game targets children in late primary school and early secondary school (ages 10–14) and runs entirely in the browser via WebAssembly — no installation required.

### Learning Goals

- Understand the location and function of major organs and organ systems
- Learn how diseases affect the body and how the immune system responds
- Discover the role of diet, vaccines, and healthy habits in keeping the body well
- Build curiosity about biology through exploration and play

### Game Concept

The protagonist miniaturises and sets off through the human body. Each region — the digestive tract, the circulatory system, the lungs, and more — is overrun by pathogens and disease. The player explores the region, interacts with cells and organs to learn about them, and then completes a minigame to defeat the illness and restore that part of the body to health.

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Language | [Rust](https://www.rust-lang.org/) |
| GPU Rendering | [wgpu](https://wgpu.rs/) (WebGPU / WebGL2) |
| Windowing | [winit](https://github.com/rust-windowing/winit) |
| Web Target | [WebAssembly](https://webassembly.org/) via [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) |
| Map Editing | [Tiled](https://www.mapeditor.org/) |
| Math | [cgmath](https://github.com/rustgfx/cgmath) |

The game runs on a **custom 2D engine** written from scratch — no third-party game framework. The engine provides:

- Orthographic camera with logical resolution scaling
- Sprite batching (single draw call per texture)
- Texture atlas animation system
- Fixed-timestep game loop
- Tiled JSON map loading
- Y-sorted sprite rendering (depth by feet position)
- AABB collision detection and resolution
- Keyboard input tracking
- Native + WebAssembly dual targets

---

## 🗺️ Roadmap

![Roadmap](docs/roadmap.svg)

---

## 🏗️ Project Structure

```
body_quest/
├── src/
│   ├── main.rs                  # Entry point
│   ├── lib.rs                   # App, event loop, wasm entry
│   ├── engine/
│   │   ├── engine.rs            # wgpu context: device, queue, surface
│   │   ├── renderer.rs          # Sprite batching, pipeline, draw calls
│   │   ├── camera.rs            # Orthographic camera + uniforms
│   │   ├── input.rs             # Key state tracking
│   │   ├── animation.rs         # Sprite sheet animation system
│   │   ├── collision.rs         # AABB collision world
│   │   ├── font.rs              # Bitmap font renderer
│   │   └── texture.rs           # Texture loading
│   ├── game/
│   │   ├── mod.rs               # GameState enum (MainMenu, Playing, Minigame…)
│   │   ├── game.rs              # Overworld: map, player, objects, Y-sort
│   │   ├── main_menu.rs         # Title screen
│   │   ├── player.rs            # Movement, animation, interaction
│   │   ├── object.rs            # Interactable objects (organs, cells, stations)
│   │   ├── tile.rs              # Tile types and Tiled map loading
│   │   └── map.rs               # Tiled JSON parser
│   └── shader.wgsl              # Vertex + fragment shader
├── assets/
│   ├── maps/                    # Tiled JSON map exports
│   ├── player.png               # Player sprite atlas
│   └── tiles/                   # Tile and object textures
├── docs/
│   └── roadmap.svg              # Project roadmap diagram
├── pkg/                         # wasm-pack output (git-ignored)
├── Cargo.toml
└── index.html                   # Browser entry point
```

---

## 🚀 Running the Game

### In the browser

Requires [wasm-pack](https://rustwasm.github.io/wasm-pack/).

```bash
wasm-pack build --target web
npx serve .
```

Open `http://localhost:3000` in a browser with WebGL2 support (Chrome, Firefox, Edge).

### Natively (for development)

```bash
cargo run
```

---

## 🎮 Controls

| Key | Action |
|---|---|
| `W` / `↑` | Move up |
| `S` / `↓` | Move down |
| `A` / `←` | Move left |
| `D` / `→` | Move right |
| `Shift` | Sprint |
| `Space` | Interact / confirm |
| `Escape` | Quit (native) |
| `F9` | Toggle debug overlay |

---

## 🧠 Game Structure

```
MainMenu
   │
   └─▶ Overworld (explore the body region)
            │
            └─▶ Interact with organ / cell / station
                        │
                        ├─▶ Dialogue (learn what it does)
                        │
                        └─▶ Minigame (fight the illness)
                                    │
                                    ├─▶ Win  → body health restored, fact logged
                                    └─▶ Lose → retry minigame
```

Each body region is a separate Tiled map. Completing all minigames in a region unlocks the next one. A persistent health bar tracks the overall state of the body.

---

## 🖼️ Engine Architecture

```
┌─────────────────────────────────┐
│           Game code             │  ← game.rs, player.rs, minigames…
│  update()  render()  on_resize()│
└────────────────┬────────────────┘
                 │ GameState enum
┌────────────────▼────────────────┐
│            Renderer             │  ← draw_sprite(), draw_sprite_frame()
│    Camera   Input   Font        │
└────────────────┬────────────────┘
                 │
┌────────────────▼────────────────┐
│             Engine              │  ← wgpu device, queue, surface
│     (native + wasm targets)     │
└─────────────────────────────────┘
```

---

## 👥 Team

| Name | Role |
|---|---|
| _Your Name_ | Engine, Rendering |
| _Team Member_ | Game Design, Art |
| _Team Member_ | Level Design, Educational Content |

---

## 📄 License

This project was developed for academic purposes at _[TU DARMSTADT]_.
