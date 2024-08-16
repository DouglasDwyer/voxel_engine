pub use voxel_engine_macros::include_assets;
pub use voxel_engine_types::{asset, input, timing, Client, Server};

/// Allows for drawing user interfaces with `egui`.
#[cfg(feature = "egui")]
pub mod egui {
    pub use egui_wings::egui::*;
    pub use egui_wings::Egui;
}

/// Holds shim functions that allow derived WASM modules to print to the console
/// and access other OS-level functionality.
mod wasi;