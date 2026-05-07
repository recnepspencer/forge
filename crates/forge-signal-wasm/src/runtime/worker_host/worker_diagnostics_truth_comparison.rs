use forge_signal::facade::diagnostics::GraphSummary;
use serde::Serialize;

use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::core::RuntimeCore;

use super::{
    canonical_worker_certification_digest, WorkerRuntimeDiagnosticsTruthReport, WorkerRuntimeShell,
};

pub(crate) fn compare_worker_diagnostics_truth(
    worker_shell: &WorkerRuntimeShell,
    compatibility_runtime: &RuntimeCore,
) -> Result<WorkerRuntimeDiagnosticsTruthReport, ForgeSignalJsError> {
    let worker_first_diagnostics_digest = canonical_worker_certification_digest(
        &StableDiagnosticsTruthProjection::from(worker_shell.diagnostics_summary_now()?),
    )?;
    let compatibility_mode_diagnostics_digest = canonical_worker_certification_digest(
        &StableDiagnosticsTruthProjection::from(compatibility_runtime.diagnostics_summary_now()?),
    )?;

    Ok(WorkerRuntimeDiagnosticsTruthReport {
        diagnostics_truth_matches: worker_first_diagnostics_digest
            == compatibility_mode_diagnostics_digest,
        worker_first_diagnostics_digest,
        compatibility_mode_diagnostics_digest,
    })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StableDiagnosticsTruthProjection {
    active_node_count: u32,
    clean_node_count: u32,
    maybe_stale_node_count: u32,
    dirty_node_count: u32,
    dependency_edge_count: u32,
    subscriber_edge_count: u32,
    nodes_with_execution_record: u32,
    nodes_with_causality: u32,
}

impl From<GraphSummary> for StableDiagnosticsTruthProjection {
    fn from(summary: GraphSummary) -> Self {
        Self {
            active_node_count: summary.active_node_count,
            clean_node_count: summary.clean_node_count,
            maybe_stale_node_count: summary.maybe_stale_node_count,
            dirty_node_count: summary.dirty_node_count,
            dependency_edge_count: summary.dependency_edge_count,
            subscriber_edge_count: summary.subscriber_edge_count,
            nodes_with_execution_record: summary.nodes_with_execution_record,
            nodes_with_causality: summary.nodes_with_causality,
        }
    }
}
