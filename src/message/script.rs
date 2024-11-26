use core::time::Duration;

use http::Uri;

use super::DatastarMessage;

/// Configuration for how to run scripts
#[derive(Debug, Default, Clone)]
pub struct ExecuteScriptConfig {
    event_id: Option<String>,
    retry_duration: Option<u32>,
    auto_remove: Option<bool>,
    attributes: Option<String>,
}

impl ExecuteScriptConfig {
    /// Create a new [`ExecuteScriptConfig`] with default options.
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

    /// Whether to remove the script after execution
    /// Defaults to true
    pub fn with_auto_remove(mut self, auto_remove: bool) -> Self {
        self.auto_remove = Some(auto_remove);
        self
    }

    /// Optionally, provide an ID for an event.
    pub fn with_attributes(mut self, attributes: impl Into<String>) -> Self {
        self.attributes = Some(attributes.into());
        self
    }
}

pub trait ExecuteScriptMessage {
    fn execute_script(script: &str, config: ExecuteScriptConfig) -> Self;
    fn redirect(url: &Uri) -> Self;
}

impl ExecuteScriptMessage for DatastarMessage {
    /// Create an SSE message for executing a js script sent to the frontend
    fn execute_script(script: &str, config: ExecuteScriptConfig) -> Self {
        let mut inner = String::from(Self::EVENT_EXECUTE_SCRIPT);

        if let Some(event_id) = config.event_id {
            Self::push_data(&mut inner, "eventId", &event_id);
        }

        if let Some(retry_duration) = config.retry_duration {
            Self::push_data(&mut inner, "retryDuration", &retry_duration.to_string());
        }

        if let Some(auto_remove) = config.auto_remove {
            Self::push_data(&mut inner, "autoRemove", &auto_remove.to_string());
        }

        if let Some(attributes) = config.attributes {
            Self::push_data(&mut inner, "attributes", &attributes.to_string());
        }

        Self::push_data(&mut inner, "script", script);

        inner.push('\n');
        Self(inner)
    }

    /// Create an SSE message for client side redirect
    fn redirect(uri: &Uri) -> Self {
        Self::execute_script(
            &format!("window.location = {}", uri),
            ExecuteScriptConfig::new(),
        )
    }
}
