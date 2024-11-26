//! Helpers for creating datastar SSE messages.
pub mod fragments;
pub mod script;
pub mod signals;

/// A datastar SSE message.
///
/// # Example
/// Build a new fragment message.
/// ```
/// use datastar::message::DatastarMessage;
/// use datastar::message::fragments::{MergeFragmentsConfig, FragmentsMessage};
///
/// DatastarMessage::merge_fragments(
///     r#"<div id="hello-world">Hello, world!</div>"#,
///     MergeFragmentsConfig::new().with_selector("#hello-world")
/// );
/// ```
#[derive(Debug, Clone)]
pub struct DatastarMessage(String);

impl DatastarMessage {
    const EVENT_FRAGMENT_MERGE: &'static str = "event: datastar-merge-fragments\n";
    const EVENT_SIGNAL_MERGE: &'static str = "event: datastar-merge-signals\n";
    const EVENT_FRAGMENT_REMOVE: &'static str = "event: datastar-remove-fragments\n";
    const EVENT_SIGNAL_REMOVE: &'static str = "event: datastar-remove-signals\n";
    const EVENT_EXECUTE_SCRIPT: &'static str = "event: datastar-execute-script\n";

    fn push_data(msg: &mut String, key: &str, val: &str) {
        msg.push_str("data: ");
        msg.push_str(key);
        msg.push(' ');
        msg.push_str(val);
        msg.push('\n');
    }

    /// Create a new SSE message that updates the client-side store.
    ///
    /// Will serialize the provided object into JSON, and returns an error if that fails.
    pub fn merge_signals<T: serde::Serialize>(obj: &T) -> Result<Self, serde_json::Error> {
        let mut inner = String::from(Self::EVENT_SIGNAL_MERGE);

        let serialized_obj = serde_json::to_string(obj)?;

        inner.push_str("data: ");
        inner.push_str(&serialized_obj);
        inner.push_str("\n\n");

        Ok(Self(inner))
    }

    /// Get the message as a [`String`].
    pub fn into_string(self) -> String {
        self.0
    }
}
