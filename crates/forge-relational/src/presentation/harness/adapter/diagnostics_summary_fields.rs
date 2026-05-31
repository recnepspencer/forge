use crate::logic::planning::RelationalExecutionModel;
use crate::performance::data::RuntimeComplexityCounters;
use crate::publication::data::PublicationDiagnosticsSnapshot;

use super::external_harness_summary_projection::{
    external_harness_summary_projection_array,
    external_harness_summary_projection_diagnostic_fields,
    external_harness_summary_projection_object, external_harness_summary_projection_string,
    external_harness_summary_projection_usize, ExternalHarnessSummaryProjection,
};
use super::run_summary_fields::publication_diagnostic_observation_fields;

pub(super) fn diagnostics_summary(
    execution_mode: forge_harness::facade::ExecutionMode,
    runtime_execution_model: RelationalExecutionModel,
    performance_counters: RuntimeComplexityCounters,
    publication_diagnostics: PublicationDiagnosticsSnapshot,
) -> ExternalHarnessSummaryProjection {
    DiagnosticsSummary::new(
        execution_mode,
        runtime_execution_model,
        performance_counters,
        publication_diagnostics,
    )
    .into_external_harness_summary_projection()
}

struct DiagnosticsSummary {
    execution_mode: forge_harness::facade::ExecutionMode,
    runtime_execution_model: RelationalExecutionModel,
    performance_counters: PerformanceCounterSummary,
    publication_diagnostics: PublicationDiagnosticSummary,
}

impl DiagnosticsSummary {
    fn new(
        execution_mode: forge_harness::facade::ExecutionMode,
        runtime_execution_model: RelationalExecutionModel,
        performance_counters: RuntimeComplexityCounters,
        publication_diagnostics: PublicationDiagnosticsSnapshot,
    ) -> Self {
        Self {
            execution_mode,
            runtime_execution_model,
            performance_counters: PerformanceCounterSummary::from_counters(performance_counters),
            publication_diagnostics: PublicationDiagnosticSummary::from_snapshot(
                publication_diagnostics,
            ),
        }
    }

    fn into_external_harness_summary_projection(self) -> ExternalHarnessSummaryProjection {
        external_harness_summary_projection_object([
            (
                "execution_mode",
                external_harness_summary_projection_string(format!("{:?}", self.execution_mode)),
            ),
            (
                "runtime_execution_model",
                external_harness_summary_projection_string(format!(
                    "{:?}",
                    self.runtime_execution_model
                )),
            ),
            (
                "performance_counters",
                self.performance_counters
                    .into_external_harness_summary_projection(),
            ),
            (
                "publication_diagnostics",
                self.publication_diagnostics
                    .into_external_harness_summary_projection(),
            ),
        ])
    }
}

struct PerformanceCounterSummary {
    query_packet_count: usize,
    query_packet_item_count: usize,
    preparation_packet_count: usize,
    preparation_packet_item_count: usize,
    preparation_packet_peak_width_total: usize,
    preparation_scope_unit_count: usize,
    preparation_serial_strategy_count: usize,
    preparation_staged_parallel_strategy_count: usize,
    post_commit_consumer_packet_count: usize,
    post_commit_consumer_peak_width_total: usize,
    post_commit_serial_strategy_count: usize,
    post_commit_parallel_strategy_count: usize,
    replay_digest_parity_checks: usize,
    replay_summary_parity_checks: usize,
}

impl PerformanceCounterSummary {
    fn from_counters(counters: RuntimeComplexityCounters) -> Self {
        Self {
            query_packet_count: counters.query_packet_count,
            query_packet_item_count: counters.query_packet_item_count,
            preparation_packet_count: counters.preparation_packet_count,
            preparation_packet_item_count: counters.preparation_packet_item_count,
            preparation_packet_peak_width_total: counters.preparation_packet_peak_width_total,
            preparation_scope_unit_count: counters.preparation_scope_unit_count,
            preparation_serial_strategy_count: counters.preparation_serial_strategy_count,
            preparation_staged_parallel_strategy_count: counters
                .preparation_staged_parallel_strategy_count,
            post_commit_consumer_packet_count: counters.post_commit_consumer_packet_count,
            post_commit_consumer_peak_width_total: counters.post_commit_consumer_peak_width_total,
            post_commit_serial_strategy_count: counters.post_commit_serial_strategy_count,
            post_commit_parallel_strategy_count: counters.post_commit_parallel_strategy_count,
            replay_digest_parity_checks: counters.replay_digest_parity_checks,
            replay_summary_parity_checks: counters.replay_summary_parity_checks,
        }
    }

    fn into_external_harness_summary_projection(self) -> ExternalHarnessSummaryProjection {
        external_harness_summary_projection_object([
            (
                "query_packet_count",
                external_harness_summary_projection_usize(self.query_packet_count),
            ),
            (
                "query_packet_item_count",
                external_harness_summary_projection_usize(self.query_packet_item_count),
            ),
            (
                "preparation_packet_count",
                external_harness_summary_projection_usize(self.preparation_packet_count),
            ),
            (
                "preparation_packet_item_count",
                external_harness_summary_projection_usize(self.preparation_packet_item_count),
            ),
            (
                "preparation_packet_peak_width_total",
                external_harness_summary_projection_usize(self.preparation_packet_peak_width_total),
            ),
            (
                "preparation_scope_unit_count",
                external_harness_summary_projection_usize(self.preparation_scope_unit_count),
            ),
            (
                "preparation_serial_strategy_count",
                external_harness_summary_projection_usize(self.preparation_serial_strategy_count),
            ),
            (
                "preparation_staged_parallel_strategy_count",
                external_harness_summary_projection_usize(
                    self.preparation_staged_parallel_strategy_count,
                ),
            ),
            (
                "post_commit_consumer_packet_count",
                external_harness_summary_projection_usize(self.post_commit_consumer_packet_count),
            ),
            (
                "post_commit_consumer_peak_width_total",
                external_harness_summary_projection_usize(
                    self.post_commit_consumer_peak_width_total,
                ),
            ),
            (
                "post_commit_serial_strategy_count",
                external_harness_summary_projection_usize(self.post_commit_serial_strategy_count),
            ),
            (
                "post_commit_parallel_strategy_count",
                external_harness_summary_projection_usize(self.post_commit_parallel_strategy_count),
            ),
            (
                "replay_digest_parity_checks",
                external_harness_summary_projection_usize(self.replay_digest_parity_checks),
            ),
            (
                "replay_summary_parity_checks",
                external_harness_summary_projection_usize(self.replay_summary_parity_checks),
            ),
        ])
    }
}

struct PublicationDiagnosticSummary {
    observation: crate::publication::data::PublicationObservationSnapshot,
    diagnostic_artifacts: Vec<DiagnosticArtifactSummary>,
    diagnostic_artifact_count: usize,
    diagnostic_entry_count: usize,
}

impl PublicationDiagnosticSummary {
    fn from_snapshot(diagnostics: PublicationDiagnosticsSnapshot) -> Self {
        let observation = diagnostics.observation;
        let diagnostic_artifacts = diagnostics
            .diagnostics
            .into_iter()
            .map(DiagnosticArtifactSummary::from_artifact)
            .collect::<Vec<_>>();
        let diagnostic_artifact_count = diagnostic_artifacts.len();
        let diagnostic_entry_count = diagnostic_artifacts
            .iter()
            .map(|artifact| artifact.entry_count())
            .sum();
        Self {
            observation,
            diagnostic_artifacts,
            diagnostic_artifact_count,
            diagnostic_entry_count,
        }
    }

    fn into_external_harness_summary_projection(self) -> ExternalHarnessSummaryProjection {
        external_harness_summary_projection_object([
            (
                "observation",
                publication_diagnostic_observation_fields(&self.observation),
            ),
            (
                "diagnostics",
                external_harness_summary_projection_array(
                    self.diagnostic_artifacts
                        .into_iter()
                        .map(DiagnosticArtifactSummary::into_external_harness_summary_projection),
                ),
            ),
            (
                "diagnostic_artifact_count",
                external_harness_summary_projection_usize(self.diagnostic_artifact_count),
            ),
            (
                "diagnostic_entry_count",
                external_harness_summary_projection_usize(self.diagnostic_entry_count),
            ),
        ])
    }
}

struct DiagnosticArtifactSummary {
    scope: String,
    kind: String,
    determinism: String,
    entries: Vec<DiagnosticEntrySummary>,
}

impl DiagnosticArtifactSummary {
    fn from_artifact(artifact: crate::diagnostics::data::RelationalDiagnosticArtifact) -> Self {
        Self {
            scope: format!("{:?}", artifact.scope),
            kind: format!("{:?}", artifact.kind),
            determinism: format!("{:?}", artifact.determinism),
            entries: artifact
                .entries
                .into_iter()
                .map(DiagnosticEntrySummary::from_entry)
                .collect(),
        }
    }

    fn entry_count(&self) -> usize {
        self.entries.len()
    }

    fn into_external_harness_summary_projection(self) -> ExternalHarnessSummaryProjection {
        external_harness_summary_projection_object([
            (
                "scope",
                external_harness_summary_projection_string(self.scope),
            ),
            (
                "kind",
                external_harness_summary_projection_string(self.kind),
            ),
            (
                "determinism",
                external_harness_summary_projection_string(self.determinism),
            ),
            (
                "entries",
                external_harness_summary_projection_array(
                    self.entries
                        .into_iter()
                        .map(DiagnosticEntrySummary::into_external_harness_summary_projection),
                ),
            ),
        ])
    }
}

struct DiagnosticEntrySummary {
    code: String,
    message: String,
    fields: crate::diagnostics::data::RelationalDiagnosticValue,
}

impl DiagnosticEntrySummary {
    fn from_entry(entry: crate::diagnostics::data::RelationalDiagnosticsEntry) -> Self {
        Self {
            code: format!("{:?}", entry.code),
            message: entry.message,
            fields: entry.fields.root().clone(),
        }
    }

    fn into_external_harness_summary_projection(self) -> ExternalHarnessSummaryProjection {
        external_harness_summary_projection_object([
            (
                "code",
                external_harness_summary_projection_string(self.code),
            ),
            (
                "message",
                external_harness_summary_projection_string(self.message),
            ),
            (
                "fields",
                external_harness_summary_projection_diagnostic_fields(self.fields),
            ),
        ])
    }
}
