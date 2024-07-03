use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum MergeStrategy {
    MorphElement,
    InnerHtml,
    OuterHtml,
    PrependElement,
    AppendElement,
    BeforeElement,
    AfterElement,
    DeleteElement,
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

#[derive(Debug, Default, Clone)]
pub struct FragmentConfig {
    merge: Option<MergeStrategy>,
    selector: Option<String>,
    settle: Option<u32>,
}

impl FragmentConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_merge(mut self, merge_strategy: MergeStrategy) -> Self {
        self.merge = Some(merge_strategy);
        self
    }

    pub fn with_selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }

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

    pub fn new_redirect(path: &str) -> Self {
        let mut inner = String::from(Self::EVENT_FRAGMENT);

        Self::push_data(&mut inner, "redirect", path);
        inner.push('\n');

        Self(inner)
    }

    pub fn new_error(msg: &str) -> Self {
        let mut inner = String::from(Self::EVENT_FRAGMENT);

        Self::push_data(&mut inner, "error", msg);
        inner.push('\n');

        Self(inner)
    }

    pub fn new_signal<T: serde::Serialize>(obj: &T) -> Result<Self, serde_json::Error> {
        let mut inner = String::from(Self::EVENT_SIGNAL);

        let serialized_obj = serde_json::to_string(obj)?;

        inner.push_str("data: ");
        inner.push_str(&serialized_obj);
        inner.push_str("\n\n");

        Ok(Self(inner))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}
