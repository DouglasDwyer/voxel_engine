//! Implements types for the [`voxel_engine`](https://crates.io/crates/voxel_engine) crate.

use serde::*;
use wings::*;

/// Manages access to `wasset`-embedded data.
pub mod asset;

/// Allows for accessing user input.
pub mod input;

/// Provides abstractions/functions for working with vectors and other math.
pub mod math;

/// Provides access to raycasting and physics functionality.
pub mod physics;

/// Allows for manipulating the camera and player.
pub mod player;

/// Facilitates access to frame and tick timing data.
pub mod timing;

/// Marks systems that will be instantiated on the game client.
#[derive(Copy, Clone, Debug)]
#[export_type]
pub struct Client;

/// Marks systems that will be instantiated on the game server.
#[derive(Copy, Clone, Debug)]
#[export_type]
pub struct Server;

/// Indicates an error that occurred in the engine.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EngineError(String);

impl<T: Into<Box<dyn std::error::Error>>> From<T> for EngineError {
    fn from(value: T) -> Self {
        Self(format!("{:?}", value.into()))
    }
}

/// Allows for writing log messages to the game's console output.
#[system_trait(host)]
pub trait Logger: 'static {
    /// Prints a log message with the specified level.
    #[global(global_log)]
    fn log(&self, level: LogLevel, message: &str);
}

/// Determines the severity of a log message.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[export_type]
pub enum LogLevel {
    /// Describes messages about the values of variables and the flow of
    /// control within a program.
    Trace,

    /// Describes messages likely to be of interest to someone debugging a
    /// program.
    Debug,

    /// Describes messages likely to be of interest to someone monitoring a
    /// program.
    Info,

    /// Describes messages indicating hazardous situations.
    Warn,

    /// Describes messages indicating serious errors.
    Error
}