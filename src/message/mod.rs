//! Helpers for creating datastar SSE messages.

use core::time::Duration;

/// Defines various strategies for merging fragments.
#[derive(Debug, Clone, Copy)]
pub enum MergeStrategy {
    /// Merge the fragment using Idiomorph (default).
    MorphElement,
    /// Replace target's innerHTML with the fragment.
    InnerHtml,
    /// Replace target's outerHTML with the fragment.
    OuterHtml,
    /// Prepend the fragment to the target's children.
    PrependElement,
    /// Append the fragment to the target's children.
    AppendElement,
    /// Insert the fragment before the target.
    BeforeElement,
    /// Insert the fragment after the target.
    AfterElement,
    /// Delete the target.
    DeleteElement,
    /// Update attributes on the target to match the fragment.
    AttributesOnly,
}

impl MergeStrategy {
    fn as_datastar_name(self) -> &'static str {
        match self {
            Self::MorphElement => "morph_element",
            Self::InnerHtml => "inner_html",
            Self::OuterHtml => "outer_html",
            Self::PrependElement => "prepend_element",
            Self::AppendElement => "append_element",
            Self::BeforeElement => "before_element",
            Self::AfterElement => "after_element",
            Self::DeleteElement => "delete_element",
            Self::AttributesOnly => "upsertAttributes",
        }
    }
}

/// Configuration for how to place a fragment on the page.
#[derive(Debug, Default, Clone)]
pub struct MergeFragmentsConfig {
    event_id: Option<String>,
    retry_duration: Option<u32>,
    selector: Option<String>,
    merge_mode: Option<MergeStrategy>,
    settle_duration: Option<u32>,
    use_view_transition: Option<bool>,
}

impl MergeFragmentsConfig {
    /// Create a new [`FragmentConfig`] with default options.
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

    /// Select the merge strategy that datastar will use to merge the fragment and the target.
    ///
    /// For more information, see [`MergeStrategy`]
    pub fn with_merge(mut self, merge_mode: MergeStrategy) -> Self {
        self.merge_mode = Some(merge_mode);
        self
    }

    /// Select the target element of the merge process.
    ///
    /// If not specified, will default to the element initiating the request.
    pub fn with_selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }

    /// How long to settle the element for.
    ///
    /// If not specified, defaults to 300ms.
    pub fn with_settle(mut self, settle_duration: Duration) -> Self {
        self.settle_duration = Some(
            settle_duration
                .as_millis()
                .try_into()
                .expect("settle time should not be >u32::MAX"),
        );
        self
    }

    /// Use view transitions?
    ///
    /// If not specified, defaults is false
    pub fn with_use_view_transition(mut self, use_view_transition: impl Into<bool>) -> Self {
        self.use_view_transition = Some(use_view_transition.into());
        self
    }
}

/// A datastar SSE message.
///
/// # Example
/// Build a new fragment message.
/// ```
/// # use datastar::message::{DatastarMessage, FragmentConfig};
///
/// DatastarMessage::new_fragment(
///     Some(r#"<div id="hello-world">Hello, world!</div>"#),
///     FragmentConfig::new().with_selector("#hello-world")
/// );
/// ```
#[derive(Debug, Clone)]
pub struct DatastarMessage(String);

impl DatastarMessage {
    const EVENT_FRAGMENT: &'static str = "event: datastar-merge-fragments\n";
    const EVENT_SIGNAL: &'static str = "event: datastar-merge-signals\n";
    const EVENT_EXECUTE_SCRIPT: &'static str = "event: datastar-execute-script\n";
    const EVENT_FRAGMENT_REMOVE: &'static str = "event: datastar-remove-fragments\n";
    const EVENT_SIGNAL_REMOVE: &'static str = "event: datastar-remove-signals\n";

    fn push_data(msg: &mut String, key: &str, val: &str) {
        msg.push_str("data: ");
        msg.push_str(key);
        msg.push(' ');
        msg.push_str(val);
        msg.push('\n');
    }

    /// Create a new SSE message that sends a fragment to the page.
    ///
    /// If the fragment is `None`, it will default to an empty `div`.
    /// This can be useful when deleting an element.
    pub fn merge_fragments(fragments: Option<Vec<String>>, config: MergeFragmentsConfig) -> Self {
        let mut inner = String::from(Self::EVENT_FRAGMENT);

        if let Some(event_id) = config.event_id {
            Self::push_data(&mut inner, "eventId", &event_id);
        }

        if let Some(retry_duration) = config.retry_duration {
            Self::push_data(&mut inner, "retryDuration", &retry_duration.to_string());
        }

        if let Some(merge) = config.merge_mode {
            Self::push_data(&mut inner, "merge", merge.as_datastar_name());
        }

        if let Some(selector) = config.selector {
            Self::push_data(&mut inner, "selector", &selector);
        }

        if let Some(settle_duration) = config.settle_duration {
            Self::push_data(&mut inner, "settleDuration", &settle_duration.to_string());
        }

        if let Some(use_view_transition) = config.use_view_transition {
            Self::push_data(
                &mut inner,
                "useViewTransition",
                &use_view_transition.to_string(),
            );
        }

        if let Some(fragments) = fragments {
            Self::push_data(&mut inner, "fragments", &fragments.join(" "));
        }

        inner.push('\n');

        Self(inner)
    }

    /// Create a new SSE message that deletes fragments from the page.
    pub fn remove_fragments(config: RemoveFragmentsConfig) {
        let mut inner = String::from(Self::EVENT_FRAGMENT_REMOVE);
        todo!()
    }

    /// Create a new SSE message that deletes signals from the store.
    pub fn remove_signals() {
        todo!()
    }

    /// Create a new SSE message that sends a fragment to the page.
    pub fn execute_script() {
        todo!()
    }

    /// Create a new SSE message that updates the client-side store.
    ///
    /// Will serialize the provided object into JSON, and returns an error if that fails.
    pub fn merge_signals<T: serde::Serialize>(obj: &T) -> Result<Self, serde_json::Error> {
        let mut inner = String::from(Self::EVENT_SIGNAL);

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
