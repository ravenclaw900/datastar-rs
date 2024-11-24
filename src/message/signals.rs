/// Configuration for how to merge signals
#[derive(Debug, Default, Clone)]
struct MergeSignalsConfig {
    only_if_missing: Option<bool>,
    event_id: Option<String>,
    retry_duration: Option<u32>,
}
