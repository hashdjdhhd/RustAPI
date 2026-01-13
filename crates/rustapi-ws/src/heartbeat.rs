//! WebSocket heartbeat configuration
//!
//! This module provides configuration for WebSocket heartbeats (ping/pong).

use std::time::Duration;

/// Configuration for WebSocket heartbeats
#[derive(Debug, Clone, Copy)]
pub struct WsHeartbeatConfig {
    /// Interval between ping messages
    pub interval: Duration,
    /// Timeout for waiting for a pong response
    pub timeout: Duration,
}

impl Default for WsHeartbeatConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(10),
        }
    }
}

impl WsHeartbeatConfig {
    /// Create a new heartbeat config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ping interval
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Set the pong timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        // **Property 12: Heartbeat interval accuracy (Configuration)**
        // **Validates: Requirements 6.1, 6.4**
        //
        // Ensures that the configuration builder correctly sets the interval and timeout,
        // and that they remain positive and valid durations.
        #[test]
        fn test_heartbeat_config_roundtrip(
            interval_secs in 1u64..3600,
            timeout_secs in 1u64..3600,
        ) {
            let config = WsHeartbeatConfig::new()
                .interval(Duration::from_secs(interval_secs))
                .timeout(Duration::from_secs(timeout_secs));

            prop_assert_eq!(config.interval, Duration::from_secs(interval_secs));
            prop_assert_eq!(config.timeout, Duration::from_secs(timeout_secs));
        }
    }
}
