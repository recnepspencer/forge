use super::catalog::ResourceMilestoneCPolicyCertificationFamily;
use super::catalog::ResourceMilestoneCPolicyPerformanceClaimId;
use super::catalog::ResourceMilestoneCPolicyScenarioId;
use super::family::ResourceMilestoneCPolicyCertificationRecord;
use super::performance::{
    ResourceMilestoneCPolicyPerformanceCloseoutRow,
    ResourceMilestoneCPolicyPerformanceCloseoutSummary,
};
use super::run::ResourceMilestoneCCertificationRunSummary;
use super::scenario::{
    ResourceMilestoneCPolicyScenarioMatrixSummary, ResourceMilestoneCPolicyScenarioRow,
};
use crate::data::resource::AdmittedResourceRevalidation;
use crate::data::resource::CancelledResourceRequest;
use crate::data::resource::DeniedResourceCancellation;
use crate::data::resource::DeniedResourceRetry;
use crate::data::resource::DeniedResourceRevalidation;
use crate::data::resource::DeniedResourceTimeout;
use crate::data::resource::DeniedResourceTimeoutHeartbeatExtension;
use crate::data::resource::ExtendedResourceTimeoutHeartbeat;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;
use crate::data::resource::ResourceDependentCancellationPropagation;
use crate::data::resource::ResourceDiagnosticsDecisionClass;
use crate::data::resource::ResourceDiagnosticsExpansionDenialClass;
use crate::data::resource::ResourceIntentEquivalenceCoalescing;
use crate::data::resource::ResourceLifecycleRetentionCompactionReport;
use crate::data::resource::ResourceLifecycleSummary;
use crate::data::resource::ResourceLifecycleTransition;
use crate::data::resource::ResourceObservationEvent;
use crate::data::resource::ResourceOverlappingGenerationAdmission;
use crate::data::resource::ResourcePolicyKind;
use crate::data::resource::ResourcePolicyRestoreCompatibilityDenialClass;
use crate::data::resource::ResourceReplayAvailabilityClass;
use crate::data::resource::ResourceReplayAvailabilityDenialClass;
use crate::data::resource::ResourceRetryBudgetScope;
use crate::data::resource::ResourceRetryDenialClass;
use crate::data::resource::ResourceTimeoutHeartbeatExtensionDenialClass;
use crate::data::resource::ScheduledResourceRetry;
use crate::data::resource::TimedOutResourceRequest;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCPolicyCertificationBundleDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) records: &'a [ResourceMilestoneCPolicyCertificationRecord],
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCPolicyScenarioMatrixDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) required_scenarios: &'a [ResourceMilestoneCPolicyScenarioId],
    pub(super) bundle_digest: &'a str,
    pub(super) summary: &'a ResourceMilestoneCPolicyScenarioMatrixSummary,
    pub(super) rows: &'a [ResourceMilestoneCPolicyScenarioRow],
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCPolicyFamilyEvidenceBasis<'a> {
    pub(super) descriptor_count: usize,
    pub(super) id_index_width: usize,
    pub(super) kind_name_index_width: usize,
    pub(super) registry_digest: &'a str,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCRetryPolicyEvidenceBasis<'a> {
    pub(super) scheduled_retry: Option<&'a ScheduledResourceRetry>,
    pub(super) denied_retry: Option<DeniedResourceRetry>,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCTimeoutPolicyEvidenceBasis<'a> {
    pub(super) timed_out_request: Option<&'a TimedOutResourceRequest>,
    pub(super) denied_timeout: Option<DeniedResourceTimeout>,
    pub(super) heartbeat_extension: Option<&'a ExtendedResourceTimeoutHeartbeat>,
    pub(super) denied_heartbeat_extension: Option<DeniedResourceTimeoutHeartbeatExtension>,
    pub(super) timeout_performance: ResourceBoundaryPerformanceEnvelope,
    pub(super) heartbeat_performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCCancellationSupersessionEvidenceBasis<'a> {
    pub(super) cancelled_request: Option<CancelledResourceRequest>,
    pub(super) denied_cancellation: Option<DeniedResourceCancellation>,
    pub(super) dependent_propagation: Option<ResourceDependentCancellationPropagation>,
    pub(super) overlap_admission: &'a ResourceOverlappingGenerationAdmission,
    pub(super) intent_coalescing: &'a ResourceIntentEquivalenceCoalescing,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCRevalidationEvidenceBasis {
    pub(super) admitted_revalidation: Option<AdmittedResourceRevalidation>,
    pub(super) denied_revalidation: Option<DeniedResourceRevalidation>,
    pub(super) lifecycle: Option<ResourceLifecycleSummary>,
    pub(super) transition: Option<ResourceLifecycleTransition>,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCObservationEvidenceBasis<'a> {
    pub(super) events: &'a [ResourceObservationEvent],
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCRetentionReplayEvidenceBasis<'a> {
    pub(super) retention_report: &'a ResourceLifecycleRetentionCompactionReport,
    pub(super) replay_class: ResourceReplayAvailabilityClass,
    pub(super) replay_denial_class: Option<ResourceReplayAvailabilityDenialClass>,
    pub(super) retained_history_unavailable_count: u32,
    pub(super) denied_completion_unavailable_count: u32,
    pub(super) retry_lineage_unavailable_count: u32,
    pub(super) availability_digest: &'a str,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCPolicyRegistryFreezeEvidenceBasis<'a> {
    pub(super) descriptor_count: usize,
    pub(super) id_index_width: usize,
    pub(super) kind_name_index_width: usize,
    pub(super) registry_digest: &'a str,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCRetryDenialEvidenceBasis {
    pub(super) class: ResourceRetryDenialClass,
    pub(super) retry_budget_scope: Option<ResourceRetryBudgetScope>,
    pub(super) retry_budget_limit: Option<u32>,
    pub(super) retry_budget_usage: Option<u32>,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCTimeoutHeartbeatDenialEvidenceBasis {
    pub(super) class: ResourceTimeoutHeartbeatExtensionDenialClass,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCRetentionCompactionEvidenceBasis {
    pub(super) retained_history_pruned_count: u32,
    pub(super) retained_history_unavailable_count: u32,
    pub(super) retained_denied_completion_pruned_count: u32,
    pub(super) retained_retry_lineage_pruned_count: u32,
    pub(super) compacted_terminal_summary_count: u32,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCDiagnosticsDenialEvidenceBasis<'a> {
    pub(super) class: ResourceDiagnosticsExpansionDenialClass,
    pub(super) policy_decision_class: ResourceDiagnosticsDecisionClass,
    pub(super) replay_reconstruction_width: u32,
    pub(super) forensic_reconstruction_width: u32,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
    pub(super) policy_decision_digest: &'a str,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCRestoreProofEvidenceBasis<'a> {
    pub(super) compatibility_digest: &'a str,
    pub(super) replay_decision_digest: &'a str,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCRestoreDenialEvidenceBasis<'a> {
    pub(super) class: ResourcePolicyRestoreCompatibilityDenialClass,
    pub(super) primary_incompatible_kind: Option<ResourcePolicyKind>,
    pub(super) compatibility_digest: &'a str,
    pub(super) replay_decision_digest: &'a str,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCPolicyPerformanceCloseoutDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) required_claims: &'a [ResourceMilestoneCPolicyPerformanceClaimId],
    pub(super) scenario_matrix_digest: &'a str,
    pub(super) summary: &'a ResourceMilestoneCPolicyPerformanceCloseoutSummary,
    pub(super) rows: &'a [ResourceMilestoneCPolicyPerformanceCloseoutRow],
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCPolicyPerformanceScenarioEvidenceBasis<'a> {
    pub(super) claim: ResourceMilestoneCPolicyPerformanceClaimId,
    pub(super) scenario: ResourceMilestoneCPolicyScenarioId,
    pub(super) scenario_evidence_digest: &'a str,
    pub(super) policy_provenance_digest: &'a str,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCPolicyPerformanceReplayCompatibilityBasis<'a> {
    pub(super) claim: ResourceMilestoneCPolicyPerformanceClaimId,
    pub(super) scenario_matrix_digest: &'a str,
    pub(super) row_digests: &'a [(ResourceMilestoneCPolicyScenarioId, String)],
    pub(super) policy_provenance_digest: &'a str,
    pub(super) performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCPolicyPerformanceReplayPolicyProvenanceBasis<'a> {
    pub(super) claim: ResourceMilestoneCPolicyPerformanceClaimId,
    pub(super) row_policy_provenance: &'a [(ResourceMilestoneCPolicyScenarioId, String)],
}

#[derive(Debug, Serialize)]
pub(super) struct ResourceMilestoneCCertificationRunDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) required_families: &'a [ResourceMilestoneCPolicyCertificationFamily],
    pub(super) required_scenarios: &'a [ResourceMilestoneCPolicyScenarioId],
    pub(super) required_performance_claims: &'a [ResourceMilestoneCPolicyPerformanceClaimId],
    pub(super) summary: &'a ResourceMilestoneCCertificationRunSummary,
    pub(super) bundle_digest: &'a str,
    pub(super) scenario_matrix_digest: &'a str,
    pub(super) performance_closeout_digest: &'a str,
    pub(super) record_digests: Vec<(ResourceMilestoneCPolicyCertificationFamily, &'a str)>,
    pub(super) scenario_digests: Vec<(ResourceMilestoneCPolicyScenarioId, &'a str)>,
    pub(super) performance_claim_digests:
        Vec<(ResourceMilestoneCPolicyPerformanceClaimId, &'a str)>,
}
