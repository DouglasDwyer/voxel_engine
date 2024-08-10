#[cfg(feature = "egui")]
pub use egui_wings as egui;
pub use voxel_engine_macros::include_assets;
pub use voxel_engine_types::{asset, timing, Client, Server};

/// Holds shim functions that allow derived WASM modules to print to the console
/// and access other OS-level functionality.
mod wasi;