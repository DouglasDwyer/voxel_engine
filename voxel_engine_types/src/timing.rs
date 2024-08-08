use std::time::*;
use wings::*;

/// Provides data about frame timings. Only available on the [`Client`](crate::Client).
#[system_trait(host)]
pub trait FrameTiming: 'static {
    /// Returns the number of frames since this timer was started.
    fn frame_count(&self) -> u64;

    /// Returns the time that the last frame took.
    fn frame_duration(&self) -> Duration;

    /// Returns the time that the previous frame ended, relative to when
    /// this timer was started.
    fn last_frame(&self) -> Duration;
}

/// Provides data about tick-based timings. Only available on the [`Server`](crate::Server).
#[system_trait(host)]
pub trait TickTiming: 'static {
    /// Provides the interval at which this timer ticks.
    fn interval(&self) -> Duration;

    /// Returns the last time that this timer emitted a tick event, relative to when
    /// this timer was started.
    fn last_tick(&self) -> Duration;

    /// Provides the next time that this timer will tick, relative to when
    /// this timer was started.
    fn next_tick(&self) -> Duration;

    /// Retrieves the number of ticks that have occurred since this
    /// tick timer was started.
    fn tick_count(&self) -> u64;
}

/// The set of events that this module raises.
pub mod on {
    use super::*;

    /// Raised whenever a new frame occurs.
    #[derive(Clone, Debug, Default)]
    #[export_type]
    pub struct Frame;

    /// Raised whenever a new tick occurs.
    #[derive(Clone, Debug, Default)]
    #[export_type]
    pub struct Tick;
}