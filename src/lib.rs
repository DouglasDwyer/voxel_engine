//! This repository contains the public modding API for the [Octo voxel game engine](https://github.com/DouglasDwyer/octo-release). The modding API can be used to develop new games for the engine, in the form of WASM plugins. The following functionality is available:
//! 
//! - Adjusting the camera position
//! - Drawing in-game GUIs using [`egui`](https://github.com/emilk/egui)
//! - Reading user input, including mouse movements, key presses, and controller inputs
//! 
//! *This crate is early in development, and breaking changes may occur at any time. This crate is not guaranteed to follow semantic versioning.*
//! 
//! ## Getting started
//! 
//! The following is an abridged tutorial demonstrating how to compile/load a WASM plugin into the voxel engine. A complete example plugin can be found in the [`example_mod` directory](/example_mod/).
//! 
//! First, initialize a Rust project:
//! 
//! ```
//! cargo new my_mod --lib
//! ```
//! 
//! Next, add the following lines to the `Cargo.toml`:
//! 
//! ```toml
//! [lib]
//! crate-type = [ "cdylib" ]  # Indicate that cargo should generate a .wasm file
//! 
//! [dependencies]
//! voxel_engine = { version = "0.1" }  # Provides access to engine APIs
//! wings = { version = "0.1" }  # Allows for creating WASM plugins
//! ```
//! 
//! Then, create an example `WingsSystem` and export it. This will cause the engine to load the system:
//! 
//! ```ignore
//! use voxel_engine::*;
//! use voxel_engine::timing::*;
//! use wings::*;
//! 
//! // The game client will load all systems listed in brackets.
//! instantiate_systems!(Client, [HelloClient]);
//! 
//! /// A system that will print out the frame
//! /// time each frame.
//! struct HelloClient {
//!     /// The context handle.
//!     ctx: WingsContextHandle<Self>
//! }
//! 
//! impl HelloClient {
//!     /// Event that executes once per frame
//!     fn print_frame_time(&mut self, _: &voxel_engine::timing::on::Frame) {
//!         println!("Frame time: {}", self.ctx.get::<dyn FrameTiming>().frame_duration().as_secs_f32());
//!     }
//! }
//! 
//! impl WingsSystem for HelloClient {
//!     const DEPENDENCIES: Dependencies = dependencies()
//!         .with::<dyn FrameTiming>();
//! 
//!     const EVENT_HANDLERS: EventHandlers<Self> = event_handlers()
//!         .with(Self::print_frame_time);
//! 
//!     fn new(ctx: WingsContextHandle<Self>) -> Self {
//!         println!("Hello client!");
//!         Self
//!     }
//! }
//! 
//! ```
//! 
//! Next, build the WASM mod with Cargo:
//! 
//! ```
//! cargo build --target wasm32-wasip1
//! ```
//! 
//! The resultant WASM binary will be located under `target/wasm32-wasip1/release`. This file can be selected and loaded into the voxel engine.

pub use voxel_engine_macros::include_assets;
pub use voxel_engine_types::{asset, input, math, physics, player, timing, Client, Server};

/// Allows for drawing user interfaces with `egui`.
#[cfg(feature = "egui")]
pub mod egui {
    pub use egui_wings::egui::*;
    pub use egui_wings::Egui;
}

/// Holds shim functions that allow derived WASM modules to print to the console
/// and access other OS-level functionality.
mod wasi;