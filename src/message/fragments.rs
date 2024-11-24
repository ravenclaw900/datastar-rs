//! Helpers for creating datastar SSE messages for fragments

use core::time::Duration;

use super::DatastarMessage;

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
            Self::MorphElement => "morph",
            Self::InnerHtml => "inner",
            Self::OuterHtml => "outer",
            Self::PrependElement => "prepend",
            Self::AppendElement => "append",
            Self::BeforeElement => "before",
            Self::AfterElement => "after",
            Self::DeleteElement => "delete",
            Self::AttributesOnly => "upsertAttributes",
        }
    }
}

/// Configuration for how to place fragments on the page.
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
    /// Create a new [`MergeFragmentConfig`] with default options.
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
    pub fn with_settle_duration(mut self, settle_duration: Duration) -> Self {
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

/// Configuration for how to remove fragments on the page.
#[derive(Debug, Default, Clone)]
pub struct RemoveFragmentsConfig {
    event_id: Option<String>,
    retry_duration: Option<u32>,
    settle_duration: Option<u32>,
    use_view_transition: Option<bool>,
}

impl RemoveFragmentsConfig {
    /// Create a new [`RemoveFragmentConfig`] with default options.
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

    /// How long to settle the element for.
    ///
    /// If not specified, defaults to 300ms.
    pub fn with_settle_duration(mut self, settle_duration: Duration) -> Self {
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

pub trait FragmentsMessage {
    fn merge_fragments(fragments: &str, config: MergeFragmentsConfig) -> Self;
    fn remove_fragments(selector: &str, config: RemoveFragmentsConfig) -> Self;
}

impl FragmentsMessage for DatastarMessage {
    /// Create a new SSE message that sends a fragment to the page.
    fn merge_fragments(fragments: &str, config: MergeFragmentsConfig) -> Self {
        let mut inner = String::from(Self::EVENT_FRAGMENT_MERGE);

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

        Self::push_data(&mut inner, "fragments", fragments);

        inner.push('\n');

        Self(inner)
    }

    /// Create a new SSE message that deletes fragments from the page.
    fn remove_fragments(selector: &str, config: RemoveFragmentsConfig) -> Self {
        let mut inner = String::from(Self::EVENT_FRAGMENT_REMOVE);

        if let Some(event_id) = config.event_id {
            Self::push_data(&mut inner, "eventId", &event_id);
        }

        if let Some(retry_duration) = config.retry_duration {
            Self::push_data(&mut inner, "retryDuration", &retry_duration.to_string());
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

        Self::push_data(&mut inner, "selector", selector);

        inner.push('\n');
        Self(inner)
    }
}
