use core::time::Duration;

use super::DatastarMessage;

/// Configuration for how to merge signals
#[derive(Debug, Default, Clone)]
pub struct MergeSignalsConfig {
    only_if_missing: Option<bool>,
    event_id: Option<String>,
    retry_duration: Option<u32>,
}

impl MergeSignalsConfig {
    /// Create a new [`MergeSignalsConfig`] with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Optionally, provide an ID for an event.
    pub fn with_event_id(mut self, event_id: impl Into<String>) -> Self {
        self.event_id = Some(event_id.into());
        self
    }

    /// Override the default Retry duration
    pub fn with_retry_duration(mut self, retry_duration: Duration) -> Self {
        self.retry_duration = Some(
            retry_duration
                .as_millis()
                .try_into()
                .expect("retry duration should not be >u32::MAX"),
        );
        self
    }

    /// Do you want to merge signals only if it's missing.
    ///
    /// Defaults to false
    pub fn with_only_if_missing(mut self, only_if_missing: bool) -> Self {
        self.only_if_missing = Some(only_if_missing);
        self
    }
}

/// Configuration for how to merge signals
#[derive(Debug, Default, Clone)]
pub struct RemoveSignalsConfig {
    event_id: Option<String>,
    retry_duration: Option<u32>,
}

/// Configuration for how to remove signals from the store
impl RemoveSignalsConfig {
    /// Create a new [`RemoveSignalsConfig`] with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Optionally, provide an ID for an event.
    pub fn with_event_id(mut self, event_id: impl Into<String>) -> Self {
        self.event_id = Some(event_id.into());
        self
    }

    /// Override the default Retry duration
    pub fn with_retry_duration(mut self, retry_duration: Duration) -> Self {
        self.retry_duration = Some(
            retry_duration
                .as_millis()
                .try_into()
                .expect("retry duration should not be >u32::MAX"),
        );
        self
    }
}

pub trait SignalsMessage {
    fn merge_signals(signals: &str, config: MergeSignalsConfig) -> Self;
    fn remove_signals(paths: &[String], config: RemoveSignalsConfig) -> Self;
}

impl SignalsMessage for DatastarMessage {
    /// Create a new SSE message that sends signals to merge in the frontend store
    fn merge_signals(data: &str, config: MergeSignalsConfig) -> Self {
        let mut inner = String::from(Self::EVENT_SIGNAL_MERGE);

        if let Some(event_id) = config.event_id {
            Self::push_data(&mut inner, "eventId", &event_id);
        }

        if let Some(retry_duration) = config.retry_duration {
            Self::push_data(&mut inner, "retryDuration", &retry_duration.to_string());
        }

        if let Some(only_if_missing) = config.only_if_missing {
            Self::push_data(&mut inner, "onlyIfMissing", &only_if_missing.to_string());
        }

        Self::push_data(&mut inner, "data", data);

        inner.push('\n');
        Self(inner)
    }

    /// Create a new SSE message that sends signals to remove from the frontend store
    fn remove_signals(paths: &[String], config: RemoveSignalsConfig) -> Self {
        let mut inner = String::from(Self::EVENT_SIGNAL_REMOVE);

        if let Some(event_id) = config.event_id {
            Self::push_data(&mut inner, "eventId", &event_id);
        }

        if let Some(retry_duration) = config.retry_duration {
            Self::push_data(&mut inner, "retryDuration", &retry_duration.to_string());
        }

        Self::push_data(&mut inner, "paths", &paths.join(" "));

        inner.push('\n');
        Self(inner)
    }
}
