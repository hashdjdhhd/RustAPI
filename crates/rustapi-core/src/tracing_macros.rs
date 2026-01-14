//! Conditional tracing macros
//!
//! These macros wrap tracing calls to allow compilation without the `tracing` feature,
//! reducing overhead for production deployments that don't need detailed logging.

/// Log at error level, only when tracing feature is enabled
#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! trace_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}

/// Log at error level, no-op when tracing feature is disabled
#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! trace_error {
    ($($arg:tt)*) => {};
}

/// Log at warn level, only when tracing feature is enabled
#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! trace_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*)
    };
}

/// Log at warn level, no-op when tracing feature is disabled
#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! trace_warn {
    ($($arg:tt)*) => {};
}

/// Log at info level, only when tracing feature is enabled
#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! trace_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

/// Log at info level, no-op when tracing feature is disabled
#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! trace_info {
    ($($arg:tt)*) => {};
}

/// Log at debug level, only when tracing feature is enabled
#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! trace_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    };
}

/// Log at debug level, no-op when tracing feature is disabled
#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! trace_debug {
    ($($arg:tt)*) => {};
}

/// Log at trace level, only when tracing feature is enabled
#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! trace_trace {
    ($($arg:tt)*) => {
        tracing::trace!($($arg)*)
    };
}

/// Log at trace level, no-op when tracing feature is disabled
#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! trace_trace {
    ($($arg:tt)*) => {};
}
