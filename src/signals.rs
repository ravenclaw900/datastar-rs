use std::time::Duration;

use crate::generator::DEFAULT_RETRY_DURATION;

#[derive(Debug, Clone)]
pub struct MergeSignalsConfig {
    pub(crate) only_if_missing: bool,
    pub(crate) event_id: Option<String>,
    pub(crate) retry_duration: u32,
}

impl Default for MergeSignalsConfig {
    fn default() -> Self {
        Self {
            only_if_missing: false,
            event_id: None,
            retry_duration: DEFAULT_RETRY_DURATION,
        }
    }
}

impl MergeSignalsConfig {
    /// Create a new [`MergeSignalsConfig`] with default options.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn only_if_missing(mut self, only_if_missing: bool) -> Self {
        self.only_if_missing = only_if_missing;
        self
    }

    pub fn event_id(mut self, event_id: impl Into<String>) -> Self {
        self.event_id = Some(event_id.into());
        self
    }

    pub fn retry_duration(mut self, retry_duration: Duration) -> Self {
        self.retry_duration = retry_duration
            .as_millis()
            .try_into()
            .expect("retry duration should not be >u32::MAX");
        self
    }
}

#[derive(Debug, Clone)]
pub struct RemoveSignalsConfig {
    pub(crate) event_id: Option<String>,
    pub(crate) retry_duration: u32,
}

impl Default for RemoveSignalsConfig {
    fn default() -> Self {
        Self {
            event_id: None,
            retry_duration: DEFAULT_RETRY_DURATION,
        }
    }
}

impl RemoveSignalsConfig {
    /// Create a new [`RemoveSignalsConfig`] with default options.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event_id(mut self, event_id: impl Into<String>) -> Self {
        self.event_id = Some(event_id.into());
        self
    }

    pub fn retry_duration(mut self, retry_duration: Duration) -> Self {
        self.retry_duration = retry_duration
            .as_millis()
            .try_into()
            .expect("retry duration should not be >u32::MAX");
        self
    }
}
