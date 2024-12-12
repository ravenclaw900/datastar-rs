use std::time::Duration;

use crate::generator::DEFAULT_RETRY_DURATION;

#[derive(Debug, Clone)]
pub struct ExecuteScriptConfig {
    pub(crate) auto_remove: bool,
    pub(crate) attributes: Vec<String>,
    pub(crate) event_id: Option<String>,
    pub(crate) retry_duration: u32,
}

impl Default for ExecuteScriptConfig {
    fn default() -> Self {
        Self {
            auto_remove: false,
            attributes: Vec::new(),
            event_id: None,
            retry_duration: DEFAULT_RETRY_DURATION,
        }
    }
}

impl ExecuteScriptConfig {
    /// Create a new [`ExecuteScriptConfig`] with default options.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn auto_remove(mut self, auto_remove: bool) -> Self {
        self.auto_remove = auto_remove;
        self
    }

    pub fn attribute(mut self, attribute: impl Into<String>) -> Self {
        self.attributes.push(attribute.into());
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
