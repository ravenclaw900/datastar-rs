use std::time::Duration;

use crate::generator::{DEFAULT_RETRY_DURATION, DEFAULT_SETTLE_DURATION};

/// Defines various strategies for merging fragments.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FragmentMergeMode {
    /// Merge the fragment using Idiomorph (default).
    #[default]
    Morph,
    /// Replace target's innerHTML with the fragment.
    Inner,
    /// Replace target's outerHTML with the fragment.
    Outer,
    /// Prepend the fragment to the target's children.
    Prepend,
    /// Append the fragment to the target's children.
    Append,
    /// Insert the fragment before the target.
    Before,
    /// Insert the fragment after the target.
    After,
    /// Update attributes on the target to match the fragment.
    UpsertAttributes,
}

impl FragmentMergeMode {
    pub(crate) fn as_datastar_name(self) -> &'static str {
        match self {
            Self::Morph => "morph",
            Self::Inner => "inner",
            Self::Outer => "outer",
            Self::Prepend => "prepend",
            Self::Append => "append",
            Self::Before => "before",
            Self::After => "after",
            Self::UpsertAttributes => "upsertAttributes",
        }
    }
}

/// Configuration for how to place a fragment on the page.
#[derive(Debug, Clone)]
pub struct MergeFragmentsConfig {
    pub(crate) merge_mode: FragmentMergeMode,
    pub(crate) selector: Option<String>,
    pub(crate) settle_duration: u32,
    pub(crate) use_view_transition: bool,
    pub(crate) event_id: Option<String>,
    pub(crate) retry_duration: u32,
}

impl Default for MergeFragmentsConfig {
    fn default() -> Self {
        Self {
            merge_mode: FragmentMergeMode::Morph,
            selector: None,
            settle_duration: DEFAULT_SETTLE_DURATION,
            use_view_transition: false,
            event_id: None,
            retry_duration: DEFAULT_RETRY_DURATION,
        }
    }
}

impl MergeFragmentsConfig {
    /// Create a new [`MergeFragmentsConfig`] with default options.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge_mode(mut self, merge_mode: FragmentMergeMode) -> Self {
        self.merge_mode = merge_mode;
        self
    }

    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }

    pub fn settle_duration(mut self, settle_duration: Duration) -> Self {
        self.settle_duration = settle_duration
            .as_millis()
            .try_into()
            .expect("settle duration should not be >u32::MAX");
        self
    }

    pub fn use_view_transition(mut self, use_view_transition: bool) -> Self {
        self.use_view_transition = use_view_transition;
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
pub struct RemoveFragmentsConfig {
    pub(crate) settle_duration: u32,
    pub(crate) use_view_transition: bool,
    pub(crate) event_id: Option<String>,
    pub(crate) retry_duration: u32,
}

impl Default for RemoveFragmentsConfig {
    fn default() -> Self {
        Self {
            settle_duration: DEFAULT_SETTLE_DURATION,
            use_view_transition: false,
            event_id: None,
            retry_duration: DEFAULT_RETRY_DURATION,
        }
    }
}

impl RemoveFragmentsConfig {
    /// Create a new [`RemoveFragmentsConfig`] with default options.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn settle_duration(mut self, settle_duration: Duration) -> Self {
        self.settle_duration = settle_duration
            .as_millis()
            .try_into()
            .expect("settle duration should not be >u32::MAX");
        self
    }

    pub fn use_view_transition(mut self, use_view_transition: bool) -> Self {
        self.use_view_transition = use_view_transition;
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
