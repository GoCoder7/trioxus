//! # Trioxus
//!
//! **Declarative Three.js for Dioxus — React Three Fiber for Rust across Web and Desktop.**
//!
//! Trioxus brings the power and ecosystem of [Three.js](https://threejs.org/) to the
//! [Dioxus](https://dioxuslabs.com/) reactive component tree.
//!
//! ## Core Architecture
//!
//! - **Unified Driver Model**: Seamlessly drives 3D scenes on both `dioxus-web` (browser WASM)
//!   and `dioxus-desktop` (Wry/WebView via lightweight IPC eval channels).
//! - **In-WebView 60/120 FPS Runtime**: High-frequency rendering, orbit controls, physics,
//!   and animations run locally in the browser/webview event loop to eliminate FFI/IPC jitter.
//! - **Fine-Grained Reactive Diffing**: Dioxus Signals communicate discrete scene graph updates
//!   (mesh creation, material property changes, camera positions) without full canvas re-renders.

#![warn(missing_docs)]

/// Early scaffold version of Trioxus.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
