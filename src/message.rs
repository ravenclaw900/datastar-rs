//! Helpers for creating datastar SSE messages.

use std::time::Duration;

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
            Self::AttributesOnly => "upsert_attributes",
        }
    }
}

/// Configuration for how to place a fragment on the page.
#[derive(Debug, Default, Clone)]
pub struct FragmentConfig {
    merge: Option<MergeStrategy>,
    selector: Option<String>,
    settle: Option<u32>,
}

impl FragmentConfig {
    /// Create a new [`FragmentConfig`] with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Select the merge strategy that datastar will use to merge the fragment and the target.
    ///
    /// For more information, see [`MergeStrategy`]
    pub fn with_merge(mut self, merge: MergeStrategy) -> Self {
        self.merge = Some(merge);
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
    /// If not specified, defaults to 500ms.
    pub fn with_settle(mut self, settle: Duration) -> Self {
        self.settle = Some(
            settle
                .as_millis()
                .try_into()
                .expect("settle time should not be >u32::MAX"),
        );
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
    const EVENT_FRAGMENT: &'static str = "event: datastar-fragment\n";
    const EVENT_SIGNAL: &'static str = "event: datastar-signal\n";

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
    pub fn new_fragment(fragment: Option<&str>, config: FragmentConfig) -> Self {
        let mut inner = String::from(Self::EVENT_FRAGMENT);

        if let Some(merge) = config.merge {
            Self::push_data(&mut inner, "merge", merge.as_datastar_name());
        }

        if let Some(selector) = config.selector {
            Self::push_data(&mut inner, "selector", &selector);
        }

        if let Some(settle) = config.settle {
            Self::push_data(&mut inner, "settle", &settle.to_string());
        }

        if let Some(fragment) = fragment {
            Self::push_data(&mut inner, "fragment", fragment);
        }

        inner.push('\n');

        Self(inner)
    }

    /// Create a new SSE message that causes a client-side redirect.
    pub fn new_redirect(path: &str) -> Self {
        let mut inner = String::from(Self::EVENT_FRAGMENT);

        Self::push_data(&mut inner, "redirect", path);
        inner.push('\n');

        Self(inner)
    }

    /// Create a new SSE message that throws an error on the client.
    pub fn new_error(msg: &str) -> Self {
        let mut inner = String::from(Self::EVENT_FRAGMENT);

        Self::push_data(&mut inner, "error", msg);
        inner.push('\n');

        Self(inner)
    }

    /// Create a new SSE message that updates the client-side store.
    ///
    /// Will serialize the provided object into JSON, and returns an error if that fails.
    pub fn new_signal<T: serde::Serialize>(obj: &T) -> Result<Self, serde_json::Error> {
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
