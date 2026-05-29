use serde_json::{Map, Value};

use crate::logic::planning::RelationalExecutionModel;
use crate::performance::data::RuntimeComplexityCounters;
use crate::publication::data::PublicationDiagnosticsSnapshot;

use super::run_summary_fields::publication_observation_fields;

pub(super) fn diagnostics_summary(
    execution_mode: forge_harness::facade::ExecutionMode,
    runtime_execution_model: RelationalExecutionModel,
    performance_counters: RuntimeComplexityCounters,
    publication_diagnostics: PublicationDiagnosticsSnapshot,
) -> Value {
    Value::Object(Map::from_iter([
        (
            "execution_mode".to_string(),
            Value::String(format!("{execution_mode:?}")),
        ),
        (
            "runtime_execution_model".to_string(),
            Value::String(format!("{runtime_execution_model:?}")),
        ),
        (
            "performance_counters".to_string(),
            performance_counter_summary(performance_counters),
        ),
        (
            "publication_diagnostics".to_string(),
            publication_diagnostic_summary(publication_diagnostics),
        ),
    ]))
}

fn performance_counter_summary(counters: RuntimeComplexityCounters) -> Value {
    Value::Object(Map::from_iter([
        (
            "query_packet_count".to_string(),
            Value::from(counters.query_packet_count as u64),
        ),
        (
            "query_packet_item_count".to_string(),
            Value::from(counters.query_packet_item_count as u64),
        ),
        (
            "preparation_packet_count".to_string(),
            Value::from(counters.preparation_packet_count as u64),
        ),
        (
            "post_commit_consumer_packet_count".to_string(),
            Value::from(counters.post_commit_consumer_packet_count as u64),
        ),
        (
            "replay_digest_parity_checks".to_string(),
            Value::from(counters.replay_digest_parity_checks as u64),
        ),
        (
            "replay_summary_parity_checks".to_string(),
            Value::from(counters.replay_summary_parity_checks as u64),
        ),
    ]))
}

fn publication_diagnostic_summary(diagnostics: PublicationDiagnosticsSnapshot) -> Value {
    Value::Object(Map::from_iter([
        (
            "observation".to_string(),
            publication_observation_fields(&diagnostics.observation),
        ),
        (
            "diagnostic_artifact_count".to_string(),
            Value::from(diagnostics.diagnostics.len() as u64),
        ),
        (
            "diagnostic_entry_count".to_string(),
            Value::from(
                diagnostics
                    .diagnostics
                    .iter()
                    .map(|artifact| artifact.entries.len())
                    .sum::<usize>() as u64,
            ),
        ),
    ]))
}
