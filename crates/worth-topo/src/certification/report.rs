use crate::diagnostics::{
    WorthDerivedFallbackReport, WorthDerivedInvalidationReport, WorthDerivedReadDiagnostics,
    WorthDerivedRebuildReport,
};
use crate::parity::WorthDerivedEquivalenceContractReport;
use crate::validators::WorthTopologyValidationReport;
use forge_relational::facade::diagnostics::DiagnosticCode;
use forge_relational::facade::errors::ErrorContext;
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::replay::{ReplayFailureClass, ReplayObservableSurface};
use serde::{Deserialize, Serialize};
use worth_schema::facade::topology_authoring::{
    WorthMilestoneOnePrimitiveCase, WorthMilestoneOnePrimitiveExpectedOutcome,
    WorthMilestoneOnePrimitiveRole,
};
use worth_schema::facade::{
    CertifiedTopologyInterpretation, WorthBridgeTraceAnchor, WorthMutationOrigin,
    WorthTopologyReadArtifact,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDeterministicDigest {
    pub algorithm: String,
    pub digest_hex: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLocalizationEntityRow {
    pub entity_id: EntityId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLocalizationRelationRow {
    pub relation_id: RelationId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLocalizationReport {
    pub topology_entities: Vec<WorthTopologyLocalizationEntityRow>,
    pub topology_relations: Vec<WorthTopologyLocalizationRelationRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLocalizationAggregateEntityRow {
    pub source: String,
    pub entity_id: EntityId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLocalizationAggregateRelationRow {
    pub source: String,
    pub relation_id: RelationId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLocalizationAggregateReport {
    pub topology_entities: Vec<WorthTopologyLocalizationAggregateEntityRow>,
    pub topology_relations: Vec<WorthTopologyLocalizationAggregateRelationRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthNamingAttachmentRow {
    pub topology_entity_id: EntityId,
    pub topology_kind_name: String,
    pub attached_persistent_name_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthNamingAttachmentReport {
    pub fully_named: bool,
    pub orphan_persistent_name_ids: Vec<EntityId>,
    pub attachments: Vec<WorthNamingAttachmentRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthNamingAttachmentAggregateRow {
    pub source: String,
    pub topology_entity_id: EntityId,
    pub topology_kind_name: String,
    pub attached_persistent_name_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthNamingAttachmentAggregateReport {
    pub fully_named: bool,
    pub orphan_persistent_name_ids: Vec<EntityId>,
    pub attachments: Vec<WorthNamingAttachmentAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthPrimitiveFamilyCoverageEntry {
    pub family: String,
    pub observed: bool,
    pub observed_member_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthPrimitiveFamilyCoverageMatrix {
    pub entries: Vec<WorthPrimitiveFamilyCoverageEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthBranchLocalTopologyReport {
    pub mutation_origin: WorthMutationOrigin,
    pub branch_local: bool,
    pub branch_id: BranchId,
    pub snapshot_id: u64,
    pub touched_aspect_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthReplayParityStatus {
    NotChecked,
    Match,
    Mismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthReplayParityReport {
    pub mutation_origin: WorthMutationOrigin,
    pub replay_origin: bool,
    pub branch_id: BranchId,
    pub parity_status: WorthReplayParityStatus,
    pub equivalence_contract: WorthDerivedEquivalenceContractReport,
    pub replay_equivalence_contract: Option<WorthDerivedEquivalenceContractReport>,
    pub relational_replay_checked: bool,
    pub relational_replay_verified: bool,
    pub replayed_commit_id: Option<String>,
    pub compared_surfaces: Vec<ReplayObservableSurface>,
    pub mismatch_count: usize,
    pub replay_failure: Option<ReplayFailureClass>,
    pub interpretation_digest_match: bool,
    pub truth_digest_match: bool,
    pub validation_digest_match: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOneCounters {
    pub topology_entity_upsert_count: usize,
    pub topology_relation_upsert_count: usize,
    pub topology_relation_remove_count: usize,
    pub commit_boundary_validator_count: usize,
    pub commit_boundary_rejection_count: usize,
    pub derived_topology_interpretation_count: usize,
    pub derived_topology_full_fallback_count: usize,
    pub naming_target_lookup_count: usize,
    pub primitive_family_member_count: usize,
    pub replay_history_length: usize,
    pub replay_interpretation_rerun_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthMilestoneOneCertificationReport {
    pub named_truth_validated: bool,
    pub topology_validated: bool,
    pub topology_truth_digest: WorthDeterministicDigest,
    pub naming_truth_digest: WorthDeterministicDigest,
    pub topology_validation_digest: WorthDeterministicDigest,
    pub topology_validation_report: WorthTopologyValidationReport,
    pub topology_localization_report: WorthTopologyLocalizationReport,
    pub naming_attachment_report: WorthNamingAttachmentReport,
    pub primitive_family_coverage_matrix: WorthPrimitiveFamilyCoverageMatrix,
    pub branch_local_topology_report: WorthBranchLocalTopologyReport,
    pub milestone_1_replay_parity_report: WorthReplayParityReport,
    pub derived_invalidation_report: WorthDerivedInvalidationReport,
    pub derived_rebuild_report: WorthDerivedRebuildReport,
    pub derived_fallback_report: WorthDerivedFallbackReport,
    pub derived_equivalence_contract_report: WorthDerivedEquivalenceContractReport,
    pub derived_read_diagnostics: WorthDerivedReadDiagnostics,
    pub counters: WorthMilestoneOneCounters,
    pub read_artifact: WorthTopologyReadArtifact,
    pub certified_interpretation: CertifiedTopologyInterpretation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthPrimitiveCorpusCaseReport {
    pub stem: String,
    pub family: String,
    pub role: WorthMilestoneOnePrimitiveRole,
    pub primitive: WorthMilestoneOnePrimitiveCase,
    pub expected_outcome: WorthMilestoneOnePrimitiveExpectedOutcome,
    pub certification: WorthMilestoneOneCertificationReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthPrimitiveCorpusCoverageEntry {
    pub family: String,
    pub admitted_smallest_count: usize,
    pub admitted_generic_count: usize,
    pub admitted_hostile_count: usize,
    pub rejected_out_of_class_count: usize,
    pub role_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthPrimitiveCorpusCoverageMatrix {
    pub entries: Vec<WorthPrimitiveCorpusCoverageEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthPrimitiveCorpusParityEntry {
    pub family: String,
    pub mainline_case_count: usize,
    pub branch_local_case_count: usize,
    pub branch_ids: Vec<String>,
    pub mainline_replay_checked_case_count: usize,
    pub mainline_replay_verified_case_count: usize,
    pub branch_local_replay_checked_case_count: usize,
    pub branch_local_replay_verified_case_count: usize,
    pub mainline_digest_parity_case_count: usize,
    pub branch_local_digest_parity_case_count: usize,
    pub cross_branch_parity_case_count: usize,
    pub parity_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthPrimitiveCorpusParityReport {
    pub entries: Vec<WorthPrimitiveCorpusParityEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthAdmittedRangeSweepRow {
    pub family: String,
    pub mainline_case_count: usize,
    pub branch_local_case_count: usize,
    pub mainline_replay_verified_case_count: usize,
    pub branch_local_replay_verified_case_count: usize,
    pub out_of_class_case_count: usize,
    pub out_of_class_rejection_count: usize,
    pub sweep_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthAdmittedRangeSweepReport {
    pub rows: Vec<WorthAdmittedRangeSweepRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthPrimitiveCorpusReport {
    pub coverage_matrix: WorthPrimitiveCorpusCoverageMatrix,
    pub parity_report: WorthPrimitiveCorpusParityReport,
    pub cases: Vec<WorthPrimitiveCorpusCaseReport>,
    pub rejected_cases: Vec<WorthPrimitiveCorpusRejectedCaseReport>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthIllegalTopologyRejectionCaseReport {
    pub name: String,
    pub family: String,
    pub role: String,
    pub rejection: WorthPrimitiveRejectionReport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthIllegalTopologyRejectionReport {
    pub case_count: usize,
    pub cases: Vec<WorthIllegalTopologyRejectionCaseReport>,
    pub rejection_digest: WorthDeterministicDigest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthBridgeProofReport {
    pub proof_case_count: usize,
    pub proved_families: Vec<String>,
    pub family_coverage_report: WorthBridgeFamilyCoverageReport,
    pub bridge_trace_anchor: WorthBridgeTraceAnchor,
    pub bridge_routing_digest: WorthDeterministicDigest,
    pub bridge_historical_evaluation_digest: WorthDeterministicDigest,
    pub route_record_count: usize,
    pub historical_evaluation_record_count: usize,
    pub source_branch: String,
    pub source_commit: String,
    pub source_snapshot: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOneValidationAggregateRow {
    pub source: String,
    pub family: String,
    pub validator: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOneValidationAggregateReport {
    pub rows: Vec<WorthMilestoneOneValidationAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOneValidatorCoverageRow {
    pub family: String,
    pub validator: String,
    pub passed_count: usize,
    pub source_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOneValidatorCoverageReport {
    pub rows: Vec<WorthMilestoneOneValidatorCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOneBranchLocalAggregateReport {
    pub mainline_case_count: usize,
    pub branch_local_case_count: usize,
    pub branch_ids: Vec<String>,
    pub branch_local_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOneReplayAggregateReport {
    pub replay_checked_case_count: usize,
    pub replay_verified_case_count: usize,
    pub replay_mismatch_case_count: usize,
    pub branch_local_replay_checked_case_count: usize,
    pub branch_local_replay_verified_case_count: usize,
    pub replay_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOneRejectionClassRow {
    pub family: String,
    pub rejection_class: String,
    pub case_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneOneRejectionClassReport {
    pub rows: Vec<WorthMilestoneOneRejectionClassRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthFailureLocalityRow {
    pub family: String,
    pub role: String,
    pub validator_family: Option<String>,
    pub rejection_class: String,
    pub diagnostic_code: Option<DiagnosticCode>,
    pub localized_entity_count: usize,
    pub localized_relation_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthFailureLocalityReport {
    pub rows: Vec<WorthFailureLocalityRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthBridgeFamilyCoverageRow {
    pub family: String,
    pub routed_case_count: usize,
    pub historical_evaluation_count: usize,
    pub proof_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthBridgeFamilyCoverageReport {
    pub rows: Vec<WorthBridgeFamilyCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthMilestoneOneCloseoutReport {
    pub topology_truth_digest: WorthDeterministicDigest,
    pub naming_truth_digest: WorthDeterministicDigest,
    pub topology_validation_digest: WorthDeterministicDigest,
    pub topology_validation_report: WorthMilestoneOneValidationAggregateReport,
    pub topology_localization_report: WorthTopologyLocalizationAggregateReport,
    pub naming_attachment_report: WorthNamingAttachmentAggregateReport,
    pub primitive_family_coverage_matrix: WorthPrimitiveCorpusCoverageMatrix,
    pub primitive_corpus_parity_report: WorthPrimitiveCorpusParityReport,
    pub admitted_range_sweep_report: WorthAdmittedRangeSweepReport,
    pub validator_coverage_report: WorthMilestoneOneValidatorCoverageReport,
    pub branch_local_topology_report: WorthMilestoneOneBranchLocalAggregateReport,
    pub milestone_1_replay_parity_report: WorthMilestoneOneReplayAggregateReport,
    pub rejection_class_report: WorthMilestoneOneRejectionClassReport,
    pub failure_locality_report: WorthFailureLocalityReport,
    pub bridge_family_coverage_report: WorthBridgeFamilyCoverageReport,
    pub seeded_bootstrap: WorthMilestoneOneCertificationReport,
    pub primitive_corpus: WorthPrimitiveCorpusReport,
    pub illegal_topology_rejection_report: WorthIllegalTopologyRejectionReport,
    pub bridge_proof_report: WorthBridgeProofReport,
    pub milestone_1_counter_report: WorthMilestoneOneCounters,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthPrimitiveCorpusRejectedCaseReport {
    pub stem: String,
    pub family: String,
    pub role: WorthMilestoneOnePrimitiveRole,
    pub primitive: WorthMilestoneOnePrimitiveCase,
    pub expected_outcome: WorthMilestoneOnePrimitiveExpectedOutcome,
    pub rejection: WorthPrimitiveRejectionReport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthPrimitiveRejectionReport {
    pub rejection_class: String,
    pub validator_family: Option<String>,
    pub diagnostic_code: Option<DiagnosticCode>,
    pub detail: String,
    pub fields_json: Option<String>,
    pub context: Option<ErrorContext>,
    pub localized_entity_count: usize,
    pub localized_relation_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedFamilyCoverageRow {
    pub family: String,
    pub admitted_case_count: usize,
    pub out_of_class_rejection_count: usize,
    pub coverage_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedFamilyCoverageMatrix {
    pub rows: Vec<WorthDerivedFamilyCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedFamilyParityRow {
    pub family: String,
    pub mainline_case_count: usize,
    pub branch_local_case_count: usize,
    pub replay_verified_case_count: usize,
    pub branch_local_replay_verified_case_count: usize,
    pub cross_branch_parity_case_count: usize,
    pub parity_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedFamilyParityMatrix {
    pub rows: Vec<WorthDerivedFamilyParityRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedValidatorCoverageRow {
    pub family: String,
    pub validator: String,
    pub phase: String,
    pub passed_count: usize,
    pub source_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedValidatorCoverageReport {
    pub rows: Vec<WorthDerivedValidatorCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneTwoCounters {
    pub derived_read_count: usize,
    pub touched_aspect_count: usize,
    pub triggered_invalidation_target_count: usize,
    pub validation_row_count: usize,
    pub whole_view_rebuild_count: usize,
    pub explicit_fallback_count: usize,
    pub replay_checked_count: usize,
    pub branch_local_case_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneTwoBranchLocalParityReport {
    pub mainline_case_count: usize,
    pub branch_local_case_count: usize,
    pub branch_ids: Vec<String>,
    pub branch_local_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneTwoReplayParityReport {
    pub replay_checked_case_count: usize,
    pub replay_verified_case_count: usize,
    pub replay_mismatch_case_count: usize,
    pub branch_local_replay_checked_case_count: usize,
    pub branch_local_replay_verified_case_count: usize,
    pub replay_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthMilestoneTwoDerivedReadReport {
    pub materialized_topology_digest: WorthDeterministicDigest,
    pub interpreted_topology_digest: WorthDeterministicDigest,
    pub derived_validation_digest: WorthDeterministicDigest,
    pub derived_invalidation_report: WorthDerivedInvalidationReport,
    pub derived_rebuild_report: WorthDerivedRebuildReport,
    pub derived_fallback_report: WorthDerivedFallbackReport,
    pub derived_equivalence_contract_report: WorthDerivedEquivalenceContractReport,
    pub derived_branch_local_parity_report: WorthBranchLocalTopologyReport,
    pub derived_replay_parity_report: WorthReplayParityReport,
    pub milestone_2_counter_report: WorthMilestoneTwoCounters,
    pub read_artifact: WorthTopologyReadArtifact,
    pub certified_interpretation: CertifiedTopologyInterpretation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthMilestoneTwoDerivedCorpusReport {
    pub materialized_topology_digest: WorthDeterministicDigest,
    pub interpreted_topology_digest: WorthDeterministicDigest,
    pub derived_validation_digest: WorthDeterministicDigest,
    pub derived_truth_basis_digest: WorthDeterministicDigest,
    pub derived_family_coverage_matrix: WorthDerivedFamilyCoverageMatrix,
    pub derived_family_parity_matrix: WorthDerivedFamilyParityMatrix,
    pub derived_branch_local_parity_report: WorthMilestoneTwoBranchLocalParityReport,
    pub derived_replay_parity_report: WorthMilestoneTwoReplayParityReport,
    pub derived_bridge_family_coverage_report: WorthBridgeFamilyCoverageReport,
    pub bridge_routing_digest: WorthDeterministicDigest,
    pub bridge_historical_evaluation_digest: WorthDeterministicDigest,
    pub milestone_2_counter_report: WorthMilestoneTwoCounters,
    pub primitive_corpus: WorthPrimitiveCorpusReport,
    pub bridge_proof_report: WorthBridgeProofReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedInvalidationAggregateRow {
    pub family: String,
    pub target: String,
    pub bridge_scope: String,
    pub source_count: usize,
    pub triggered_case_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedInvalidationAggregateReport {
    pub touched_aspect_count: usize,
    pub triggered_target_count: usize,
    pub rows: Vec<WorthDerivedInvalidationAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedRebuildAggregateRow {
    pub family: String,
    pub source_count: usize,
    pub whole_view_rebuild_count: usize,
    pub topology_entity_count: usize,
    pub topology_relation_count: usize,
    pub interpreted_wire_count: usize,
    pub interpreted_shell_count: usize,
    pub validation_row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedRebuildAggregateReport {
    pub rows: Vec<WorthDerivedRebuildAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedFallbackAggregateRow {
    pub family: String,
    pub source_count: usize,
    pub whole_view_materialization_count: usize,
    pub explicit_fallback_count: usize,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedFallbackAggregateReport {
    pub rows: Vec<WorthDerivedFallbackAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedEquivalenceContractAggregateRow {
    pub source: String,
    pub family: String,
    pub truth_basis_digest_hex: String,
    pub touched_aspect_count: usize,
    pub triggered_invalidation_target_count: usize,
    pub materialized_topology_digest: WorthDeterministicDigest,
    pub interpreted_topology_digest: WorthDeterministicDigest,
    pub derived_validation_digest: WorthDeterministicDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedEquivalenceContractAggregateReport {
    pub rows: Vec<WorthDerivedEquivalenceContractAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthMilestoneTwoCloseoutReport {
    pub materialized_topology_digest: WorthDeterministicDigest,
    pub interpreted_topology_digest: WorthDeterministicDigest,
    pub derived_validation_digest: WorthDeterministicDigest,
    pub derived_truth_basis_digest: WorthDeterministicDigest,
    pub bridge_routing_digest: WorthDeterministicDigest,
    pub bridge_historical_evaluation_digest: WorthDeterministicDigest,
    pub derived_family_coverage_matrix: WorthDerivedFamilyCoverageMatrix,
    pub derived_family_parity_matrix: WorthDerivedFamilyParityMatrix,
    pub derived_validator_coverage_report: WorthDerivedValidatorCoverageReport,
    pub derived_invalidation_report: WorthDerivedInvalidationAggregateReport,
    pub derived_rebuild_report: WorthDerivedRebuildAggregateReport,
    pub derived_equivalence_contract_report: WorthDerivedEquivalenceContractAggregateReport,
    pub derived_fallback_report: WorthDerivedFallbackAggregateReport,
    pub derived_failure_locality_report: WorthFailureLocalityReport,
    pub derived_branch_local_parity_report: WorthMilestoneTwoBranchLocalParityReport,
    pub derived_replay_parity_report: WorthMilestoneTwoReplayParityReport,
    pub derived_bridge_family_coverage_report: WorthBridgeFamilyCoverageReport,
    pub milestone_2_counter_report: WorthMilestoneTwoCounters,
    pub derived_corpus: WorthMilestoneTwoDerivedCorpusReport,
}
