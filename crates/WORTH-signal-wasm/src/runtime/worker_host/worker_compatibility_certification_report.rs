use serde::Serialize;

use super::{
    WorkerBranchLifecycleParityProbeReport, WorkerCompatibilityTruthReport,
    WorkerGraphPublicationSummary,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCompatibilityCertificationReport {
    pub committed_truth_report: WorkerCompatibilityTruthReport,
    pub async_lifecycle_report: WorkerRuntimeAsyncLifecycleTruthReport,
    pub branch_lifecycle_report: WorkerBranchLifecycleParityProbeReport,
    pub observation_report: WorkerRuntimeObservationTruthReport,
    pub diagnostics_report: WorkerRuntimeDiagnosticsTruthReport,
    pub isolation_report: WorkerRuntimeNonHostIsolationReport,
    pub worker_publication_summary: WorkerGraphPublicationSummary,
    pub compatibility_publication_summary: WorkerGraphPublicationSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerRuntimeObservationTruthReport {
    pub worker_first_observation_digest: String,
    pub compatibility_mode_observation_digest: String,
    pub observation_truth_matches: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerRuntimeDiagnosticsTruthReport {
    pub worker_first_diagnostics_digest: String,
    pub compatibility_mode_diagnostics_digest: String,
    pub diagnostics_truth_matches: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerRuntimeAsyncLifecycleTruthReport {
    pub worker_first_async_lifecycle_digest: String,
    pub compatibility_mode_async_lifecycle_digest: String,
    pub async_lifecycle_truth_matches: bool,
    pub request_admitted: bool,
    pub completion_committed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerRuntimeNonHostIsolationReport {
    pub declared_independent_region_count: u64,
    pub declared_independent_region_recipe_ids: Vec<String>,
    pub worker_admitted_source_count: u64,
    pub worker_admitted_recipe_count: u64,
    pub transaction_op_count: u64,
    pub worker_touched_node_count: u32,
    pub worker_evaluated_node_count: u32,
    pub worker_recomputed_node_count: u32,
    pub all_regions_remain_worker_owned: bool,
    pub broad_placement_collapse_detected: bool,
    pub placement_frontier_digest: String,
    pub worker_breadth_digest: String,
    pub main_thread_hosted_digest: String,
    pub broadening_denial_artifact: String,
}
