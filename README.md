<p align="center">
  <img src="https://raw.githubusercontent.com/GoCoder7/trioxus/main/assets/icon.svg" alt="trioxus logo" width="160" height="160" />
</p>

<h1 align="center">trioxus</h1>

<p align="center">
  <strong>Declarative Three.js for Dioxus — React Three Fiber for Rust across Web and Desktop.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/trioxus"><img src="https://img.shields.io/crates/v/trioxus.svg" alt="Crates.io" /></a>
  <a href="https://docs.rs/trioxus"><img src="https://docs.rs/trioxus/badge.svg" alt="docs.rs" /></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg" alt="License" /></a>
</p>

---

**Trioxus** is a reactive, declarative 3D scene graph engine and component library that brings the power and ecosystem of [Three.js](https://threejs.org/) to [Dioxus](https://dioxuslabs.com/) across Web (`dioxus-web`) and Desktop (`dioxus-desktop`).

Just as **React Three Fiber (R3F)** revolutionized 3D on the React web, Trioxus enables Rust developers to build rich, interactive 3D visualizations, model viewers, and spatial graphics using idiomatic Dioxus RSX syntax and Signals.

---

## 🌟 Why Trioxus?

- **Zero-Friction 3D in Rust**: Pure-Rust 3D engines (like Bevy or raw wgpu on WASM) often come with steep binary sizes (tens of megabytes), slow compile iterations, and cannot easily tap into the vast universe of web shaders, post-processing filters, and 3D format loaders.
- **Battle-Tested Ecosystem**: Direct access to 10+ years of Three.js loaders (GLTF, Draco, KTX2), PBR materials, lights, shadows, and WebGL/WebGPU renderers.
- **Fine-Grained Reactivity**: Leverages Dioxus 0.5/0.6 Signals to update 3D properties (position, color, intensity) directly without wasteful full-VDOM re-renders.

---

## 🏗️ Unified Cross-Platform Architecture

Trioxus is designed from the ground up to support both **Browser Web (`dioxus-web`)** and **Native Desktop (`dioxus-desktop`)** using a unified command-driven driver model:

```text
[ Dioxus Component Tree (RSX) ]
           │
           ▼ (Declarative Scene Graph Diff / Command Stream)
   [ trioxus-core (Rust) ]
           │
     ┌─────┴─────────────────────────┐
     ▼                               ▼
[ dioxus-web Driver ]      [ dioxus-desktop Driver ]
(Wasm direct call)          (use_eval IPC channel)
     │                               │
     └─────┬─────────────────────────┘
           ▼ (Command Packets / JSON)
[ trioxus-runtime.js (In-WebView Runtime) ]
   - Three.js Scene Graph cache & lifecycle management
   - 60 / 120 FPS requestAnimationFrame render loop
   - Raycasting interaction & pointer event detection
           │
           ▼
     [ Three.js ] ──► [ WebGL / WebGPU Canvas ]
```

### ⚡ Solving the 60 FPS Jitter Problem
High-frequency loops (animations, physics ticks, OrbitControls) run smoothly **inside the browser/webview event loop**. Rust does not poll or send IPC packets on every frame; instead, Rust sends **state diffs only when Signals change**, guaranteeing silky-smooth 120 FPS even across native desktop IPC boundaries.

---

## 🔮 Declarative RSX Preview (Target Syntax)

```rust
use dioxus::prelude::*;
use trioxus::prelude::*;

fn App() -> Element {
    let mut rotation_y = use_signal(|| 0.0f32);
    let mut color = use_signal(|| "royalblue");

    rsx! {
        Canvas {
            PerspectiveCamera { make_default: true, position: (0.0, 2.0, 5.0) }
            AmbientLight { intensity: 0.5 }
            DirectionalLight { position: (5.0, 5.0, 5.0), intensity: 1.0 }

            Mesh {
                position: [0.0, 0.0, 0.0],
                rotation: [0.0, rotation_y(), 0.0],
                on_click: move |_| {
                    color.set(if color() == "royalblue" { "crimson" } else { "royalblue" });
                },

                BoxGeometry { args: (1.0, 1.0, 1.0) }
                StandardMaterial { color: color(), roughness: 0.3 }
            }

            OrbitControls {}
        }
    }
}
```

---

## 🗺️ Implementation Roadmap

- [x] **Phase 0: Project Scaffolding**
  - Package metadata, dual licensing (MIT / Apache-2.0).
  - CI/CD release pipeline with `release-plz`.
- [ ] **Phase 1: Minimal PoC (Core Runtime & Bridge)**
  - `<Canvas>` container initializing WebGLRenderer, Scene, and PerspectiveCamera.
  - Bundled `trioxus-runtime.js` bridge.
  - Basic rotating cube demonstration.
- [ ] **Phase 2: Scene Graph & Attachment Hierarchy**
  - `Mesh`, primitive geometries (`BoxGeometry`, `SphereGeometry`, `PlaneGeometry`).
  - Standard materials (`MeshStandardMaterial`, `MeshBasicMaterial`).
  - Context-driven node attachment (`use_context` hierarchy) and automatic GPU cleanup (`dispose()`).
- [ ] **Phase 3: Events & High-Performance Reactivity**
  - Three.js Raycaster integration for 3D pointer events (`on_click`, `on_pointer_enter`, `on_pointer_leave`).
  - `use_frame` hook for custom frame callbacks.
- [ ] **Phase 4: Ecosystem Helpers (trioxus-drei)**
  - `OrbitControls` helper.
  - `use_gltf` asset loader hook for 3D models.
  - HTML overlay projection inside 3D space (`Html` component).

---

## 📦 Status

Early scaffold release (`v0.1.0`). Package metadata, repository scaffolding, and automated release pipelines are configured. The core engine and bridge runtime are under active development.

---

## 📄 License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
