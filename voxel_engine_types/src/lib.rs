use wings::*;

#[derive(Copy, Clone, Debug)]
#[export_type]
pub struct Client;

#[derive(Copy, Clone, Debug)]
#[export_type]
pub struct Server;

#[system_trait(host)]
pub trait Logger: 'static {
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