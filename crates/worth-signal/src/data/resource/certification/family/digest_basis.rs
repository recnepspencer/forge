use super::contract::ResourceCertificationRecord;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceBranchRestoreReport;
use crate::data::resource::ResourceCompletionRollbackSubject;
use crate::data::resource::ResourceLifecycleTransition;
use crate::data::resource::ResourceObservationBatchReport;
use crate::data::resource::ResourceRequestHandle;
use crate::data::resource::ResourceRuntimeSummary;
use crate::data::resource::ResourceSupersessionRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct ResourceCertificationBundleDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) records: &'a [ResourceCertificationRecord],
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceLifecycleParityEvidenceBasis<'a> {
    pub(super) descriptor_digest: &'a str,
    pub(super) lifecycle_digest: &'a str,
    pub(super) output_continuity_digest: &'a str,
    pub(super) denied_completion_digest: &'a str,
    pub(super) retry_lineage_digest: &'a str,
    pub(super) in_flight_digest: &'a str,
    pub(super) replay_digest: &'a str,
    pub(super) retained_history_unavailable_count: u32,
    pub(super) denied_completion_unavailable_count: u32,
    pub(super) retry_lineage_unavailable_count: u32,
    pub(super) diagnostics_provenance_digest: &'a str,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceSupersessionEvidenceBasis {
    pub(super) supersession: ResourceSupersessionRecord,
    pub(super) superseded_request: Option<ResourceRequestHandle>,
    pub(super) superseded_transition: Option<ResourceLifecycleTransition>,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceRollbackObservationEvidenceBasis<'a> {
    pub(super) subject: ResourceCompletionRollbackSubject,
    pub(super) observation: ResourceObservationBatchReport,
    pub(super) control_observation: ResourceObservationBatchReport,
    pub(super) pre_rollback_descriptor_digest: &'a str,
    pub(super) pre_rollback_lifecycle_digest: &'a str,
    pub(super) pre_rollback_output_continuity_digest: &'a str,
    pub(super) pre_rollback_denied_completion_digest: &'a str,
    pub(super) pre_rollback_retry_lineage_digest: &'a str,
    pub(super) pre_rollback_in_flight_digest: &'a str,
    pub(super) pre_rollback_replay_digest: &'a str,
    pub(super) post_rollback_descriptor_digest: &'a str,
    pub(super) post_rollback_lifecycle_digest: &'a str,
    pub(super) post_rollback_output_continuity_digest: &'a str,
    pub(super) post_rollback_denied_completion_digest: &'a str,
    pub(super) post_rollback_retry_lineage_digest: &'a str,
    pub(super) post_rollback_in_flight_digest: &'a str,
    pub(super) post_rollback_replay_digest: &'a str,
    pub(super) diagnostics_provenance_digest: &'a str,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceBranchRestoreReplayEvidenceBasis<'a> {
    pub(super) restore: ResourceBranchRestoreReport,
    pub(super) descriptor_digest: &'a str,
    pub(super) lifecycle_digest: &'a str,
    pub(super) denied_completion_digest: &'a str,
    pub(super) in_flight_digest: &'a str,
    pub(super) replay_digest: &'a str,
    pub(super) replay_performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceInflightBoundednessEvidenceBasis {
    pub(super) summary: ResourceRuntimeSummary,
    pub(super) replay_in_flight_width: u32,
    pub(super) replay_digest: String,
    pub(super) retry_admission_count: u64,
    pub(super) retry_duplicate_denial_count: u64,
    pub(super) branch_restore_count: u64,
    pub(super) branch_restore_broad_rebuild_denial_count: u64,
    pub(super) superseded_completion_denial_count: u64,
    pub(super) duplicate_completion_denial_count: u64,
    pub(super) contradictory_completion_denial_count: u64,
    pub(super) unknown_request_completion_denial_count: u64,
    pub(super) broad_scan_denial_count: u64,
    pub(super) hot_in_flight_lookup_count: u64,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}
