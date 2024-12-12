use asynk_strim::Yielder;

use crate::{
    fragments::{FragmentMergeMode, MergeFragmentsConfig, RemoveFragmentsConfig},
    scripts::ExecuteScriptConfig,
    signals::{MergeSignalsConfig, RemoveSignalsConfig},
};

pub(crate) const DEFAULT_RETRY_DURATION: u32 = 1000;
pub(crate) const DEFAULT_SETTLE_DURATION: u32 = 300;

pub struct ServerSentEventGenerator {
    pub(crate) yielder: Yielder<String>,
}

impl ServerSentEventGenerator {
    const MERGE_FRAGMENTS: &'static str = "datastar-merge-fragments";
    const MERGE_SIGNALS: &'static str = "datastar-merge-signals";
    const REMOVE_FRAGMENTS: &'static str = "datastar-remove-fragments";
    const REMOVE_SIGNALS: &'static str = "datastar-remove-signals";
    const EXECUTE_SCRIPT: &'static str = "datastar-execute-script";

    async fn send(
        &mut self,
        event_type: &str,
        data_pairs: &[(&str, &str)],
        event_id: Option<String>,
        retry_duration: u32,
    ) {
        let mut event = format!("event: {event_type}\n");

        if let Some(event_id) = event_id {
            event.push_str(&format!("id: {event_id}\n"));
        }

        if retry_duration != DEFAULT_RETRY_DURATION {
            event.push_str(&format!("retryDuration: {retry_duration}\n"));
        }

        event.extend(data_pairs.iter().map(|(k, v)| format!("data: {k} {v}\n")));

        event.push('\n');

        self.yielder.yield_item(event).await;
    }

    pub async fn merge_fragments(
        &mut self,
        fragments: &str,
        MergeFragmentsConfig {
            merge_mode,
            selector,
            settle_duration,
            use_view_transition,
            event_id,
            retry_duration,
        }: MergeFragmentsConfig,
    ) {
        let mut data_pairs = Vec::new();

        if merge_mode != FragmentMergeMode::Morph {
            data_pairs.push(("mergeMode", merge_mode.as_datastar_name()));
        }

        if let Some(selector) = &selector {
            data_pairs.push(("selector", selector));
        }

        let settle_duration_str;
        if settle_duration != DEFAULT_SETTLE_DURATION {
            settle_duration_str = settle_duration.to_string();
            data_pairs.push(("settleDuration", &settle_duration_str));
        }

        if use_view_transition {
            data_pairs.push(("useViewTransition", "true"));
        }

        data_pairs.extend(fragments.lines().map(|line| ("fragments", line)));

        self.send(Self::MERGE_FRAGMENTS, &data_pairs, event_id, retry_duration)
            .await;
    }

    pub async fn remove_fragments(
        &mut self,
        selector: &str,
        RemoveFragmentsConfig {
            settle_duration,
            use_view_transition,
            event_id,
            retry_duration,
        }: RemoveFragmentsConfig,
    ) {
        let mut data_pairs = Vec::new();

        data_pairs.push(("selector", selector));

        let settle_duration_str;
        if settle_duration != DEFAULT_SETTLE_DURATION {
            settle_duration_str = settle_duration.to_string();
            data_pairs.push(("settleDuration", &settle_duration_str));
        }

        if use_view_transition {
            data_pairs.push(("useViewTransition", "true"));
        }

        self.send(
            Self::REMOVE_FRAGMENTS,
            &data_pairs,
            event_id,
            retry_duration,
        )
        .await;
    }

    pub async fn merge_signals(
        &mut self,
        signals: &str,
        MergeSignalsConfig {
            only_if_missing,
            event_id,
            retry_duration,
        }: MergeSignalsConfig,
    ) {
        let mut data_pairs = Vec::new();

        if only_if_missing {
            data_pairs.push(("onlyIfMissing", "true"));
        }

        data_pairs.extend(signals.lines().map(|line| ("signals", line)));

        self.send(Self::MERGE_SIGNALS, &data_pairs, event_id, retry_duration)
            .await;
    }

    pub async fn remove_signals(
        &mut self,
        paths: &[&str],
        RemoveSignalsConfig {
            event_id,
            retry_duration,
        }: RemoveSignalsConfig,
    ) {
        let mut data_pairs = Vec::new();

        data_pairs.extend(paths.iter().map(|&path| ("paths", path)));

        self.send(Self::REMOVE_SIGNALS, &data_pairs, event_id, retry_duration)
            .await;
    }

    pub async fn execute_script(
        &mut self,
        script: &str,
        ExecuteScriptConfig {
            auto_remove,
            attributes,
            event_id,
            retry_duration,
        }: ExecuteScriptConfig,
    ) {
        let mut data_pairs = Vec::new();

        if !auto_remove {
            data_pairs.push(("autoRemove", "false"));
        }

        data_pairs.extend(
            attributes
                .iter()
                .map(|attribute| ("attributes", attribute.as_str())),
        );

        data_pairs.extend(script.lines().map(|line| ("script", line)));

        self.send(Self::EXECUTE_SCRIPT, &data_pairs, event_id, retry_duration)
            .await;
    }
}
