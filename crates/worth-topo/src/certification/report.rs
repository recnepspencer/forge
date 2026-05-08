use crate::diagnostics::{
    DerivedFallbackReport, DerivedInvalidationReport, DerivedReadDiagnostics, DerivedRebuildReport,
};
use crate::parity::DerivedEquivalenceContractReport;
use crate::validators::TopologyValidationReport;
use forge_relational::facade::diagnostics::DiagnosticCode;
use forge_relational::facade::errors::ErrorContext;
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::replay::{ReplayFailureClass, ReplayObservableSurface};
use schema::facade::topology_authoring::{
    MilestoneOnePrimitiveCase, MilestoneOnePrimitiveExpectedOutcome, MilestoneOnePrimitiveRole,
};
use schema::facade::{
    BridgeTraceAnchor, CertifiedTopologyInterpretation, MutationOrigin, TopologyReadArtifact,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicDigest {
    pub algorithm: String,
    pub digest_hex: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationEntityRow {
    pub entity_id: EntityId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationRelationRow {
    pub relation_id: RelationId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationReport {
    pub topology_entities: Vec<TopologyLocalizationEntityRow>,
    pub topology_relations: Vec<TopologyLocalizationRelationRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationAggregateEntityRow {
    pub source: String,
    pub entity_id: EntityId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationAggregateRelationRow {
    pub source: String,
    pub relation_id: RelationId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationAggregateReport {
    pub topology_entities: Vec<TopologyLocalizationAggregateEntityRow>,
    pub topology_relations: Vec<TopologyLocalizationAggregateRelationRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingAttachmentRow {
    pub topology_entity_id: EntityId,
    pub topology_kind_name: String,
    pub attached_persistent_name_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingAttachmentReport {
    pub fully_named: bool,
    pub orphan_persistent_name_ids: Vec<EntityId>,
    pub attachments: Vec<NamingAttachmentRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingAttachmentAggregateRow {
    pub source: String,
    pub topology_entity_id: EntityId,
    pub topology_kind_name: String,
    pub attached_persistent_name_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingAttachmentAggregateReport {
    pub fully_named: bool,
    pub orphan_persistent_name_ids: Vec<EntityId>,
    pub attachments: Vec<NamingAttachmentAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveFamilyCoverageEntry {
    pub family: String,
    pub observed: bool,
    pub observed_member_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveFamilyCoverageMatrix {
    pub entries: Vec<PrimitiveFamilyCoverageEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchLocalTopologyReport {
    pub mutation_origin: MutationOrigin,
    pub branch_local: bool,
    pub branch_id: BranchId,
    pub snapshot_id: u64,
    pub touched_aspect_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayParityStatus {
    NotChecked,
    Match,
    Mismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayParityReport {
    pub mutation_origin: MutationOrigin,
    pub replay_origin: bool,
    pub branch_id: BranchId,
    pub parity_status: ReplayParityStatus,
    pub equivalence_contract: DerivedEquivalenceContractReport,
    pub replay_equivalence_contract: Option<DerivedEquivalenceContractReport>,
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
pub struct MilestoneOneCounters {
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
pub struct MilestoneOneCertificationReport {
    pub named_truth_validated: bool,
    pub topology_validated: bool,
    pub topology_truth_digest: DeterministicDigest,
    pub naming_truth_digest: DeterministicDigest,
    pub topology_validation_digest: DeterministicDigest,
    pub topology_validation_report: TopologyValidationReport,
    pub topology_localization_report: TopologyLocalizationReport,
    pub naming_attachment_report: NamingAttachmentReport,
    pub primitive_family_coverage_matrix: PrimitiveFamilyCoverageMatrix,
    pub branch_local_topology_report: BranchLocalTopologyReport,
    pub milestone_1_replay_parity_report: ReplayParityReport,
    pub derived_invalidation_report: DerivedInvalidationReport,
    pub derived_rebuild_report: DerivedRebuildReport,
    pub derived_fallback_report: DerivedFallbackReport,
    pub derived_equivalence_contract_report: DerivedEquivalenceContractReport,
    pub derived_read_diagnostics: DerivedReadDiagnostics,
    pub counters: MilestoneOneCounters,
    pub read_artifact: TopologyReadArtifact,
    pub certified_interpretation: CertifiedTopologyInterpretation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimitiveCorpusCaseReport {
    pub stem: String,
    pub family: String,
    pub role: MilestoneOnePrimitiveRole,
    pub primitive: MilestoneOnePrimitiveCase,
    pub expected_outcome: MilestoneOnePrimitiveExpectedOutcome,
    pub certification: MilestoneOneCertificationReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveCorpusCoverageEntry {
    pub family: String,
    pub admitted_smallest_count: usize,
    pub admitted_generic_count: usize,
    pub admitted_hostile_count: usize,
    pub rejected_out_of_class_count: usize,
    pub role_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveCorpusCoverageMatrix {
    pub entries: Vec<PrimitiveCorpusCoverageEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveCorpusParityEntry {
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
pub struct PrimitiveCorpusParityReport {
    pub entries: Vec<PrimitiveCorpusParityEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmittedRangeSweepRow {
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
pub struct AdmittedRangeSweepReport {
    pub rows: Vec<AdmittedRangeSweepRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimitiveCorpusReport {
    pub coverage_matrix: PrimitiveCorpusCoverageMatrix,
    pub parity_report: PrimitiveCorpusParityReport,
    pub cases: Vec<PrimitiveCorpusCaseReport>,
    pub rejected_cases: Vec<PrimitiveCorpusRejectedCaseReport>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IllegalTopologyRejectionCaseReport {
    pub name: String,
    pub family: String,
    pub role: String,
    pub rejection: PrimitiveRejectionReport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IllegalTopologyRejectionReport {
    pub case_count: usize,
    pub cases: Vec<IllegalTopologyRejectionCaseReport>,
    pub rejection_digest: DeterministicDigest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BridgeProofReport {
    pub proof_case_count: usize,
    pub proved_families: Vec<String>,
    pub family_coverage_report: BridgeFamilyCoverageReport,
    pub bridge_trace_anchor: BridgeTraceAnchor,
    pub bridge_routing_digest: DeterministicDigest,
    pub bridge_historical_evaluation_digest: DeterministicDigest,
    pub route_record_count: usize,
    pub historical_evaluation_record_count: usize,
    pub source_branch: String,
    pub source_commit: String,
    pub source_snapshot: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneOneValidationAggregateRow {
    pub source: String,
    pub family: String,
    pub validator: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneOneValidationAggregateReport {
    pub rows: Vec<MilestoneOneValidationAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneOneValidatorCoverageRow {
    pub family: String,
    pub validator: String,
    pub passed_count: usize,
    pub source_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneOneValidatorCoverageReport {
    pub rows: Vec<MilestoneOneValidatorCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneOneBranchLocalAggregateReport {
    pub mainline_case_count: usize,
    pub branch_local_case_count: usize,
    pub branch_ids: Vec<String>,
    pub branch_local_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneOneReplayAggregateReport {
    pub replay_checked_case_count: usize,
    pub replay_verified_case_count: usize,
    pub replay_mismatch_case_count: usize,
    pub branch_local_replay_checked_case_count: usize,
    pub branch_local_replay_verified_case_count: usize,
    pub replay_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneOneRejectionClassRow {
    pub family: String,
    pub rejection_class: String,
    pub case_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneOneRejectionClassReport {
    pub rows: Vec<MilestoneOneRejectionClassRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureLocalityRow {
    pub family: String,
    pub role: String,
    pub validator_family: Option<String>,
    pub rejection_class: String,
    pub diagnostic_code: Option<DiagnosticCode>,
    pub localized_entity_count: usize,
    pub localized_relation_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureLocalityReport {
    pub rows: Vec<FailureLocalityRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeFamilyCoverageRow {
    pub family: String,
    pub routed_case_count: usize,
    pub historical_evaluation_count: usize,
    pub proof_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeFamilyCoverageReport {
    pub rows: Vec<BridgeFamilyCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilestoneOneCloseoutReport {
    pub topology_truth_digest: DeterministicDigest,
    pub naming_truth_digest: DeterministicDigest,
    pub topology_validation_digest: DeterministicDigest,
    pub topology_validation_report: MilestoneOneValidationAggregateReport,
    pub topology_localization_report: TopologyLocalizationAggregateReport,
    pub naming_attachment_report: NamingAttachmentAggregateReport,
    pub primitive_family_coverage_matrix: PrimitiveCorpusCoverageMatrix,
    pub primitive_corpus_parity_report: PrimitiveCorpusParityReport,
    pub admitted_range_sweep_report: AdmittedRangeSweepReport,
    pub validator_coverage_report: MilestoneOneValidatorCoverageReport,
    pub branch_local_topology_report: MilestoneOneBranchLocalAggregateReport,
    pub milestone_1_replay_parity_report: MilestoneOneReplayAggregateReport,
    pub rejection_class_report: MilestoneOneRejectionClassReport,
    pub failure_locality_report: FailureLocalityReport,
    pub bridge_family_coverage_report: BridgeFamilyCoverageReport,
    pub seeded_bootstrap: MilestoneOneCertificationReport,
    pub primitive_corpus: PrimitiveCorpusReport,
    pub illegal_topology_rejection_report: IllegalTopologyRejectionReport,
    pub bridge_proof_report: BridgeProofReport,
    pub milestone_1_counter_report: MilestoneOneCounters,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimitiveCorpusRejectedCaseReport {
    pub stem: String,
    pub family: String,
    pub role: MilestoneOnePrimitiveRole,
    pub primitive: MilestoneOnePrimitiveCase,
    pub expected_outcome: MilestoneOnePrimitiveExpectedOutcome,
    pub rejection: PrimitiveRejectionReport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimitiveRejectionReport {
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
pub struct DerivedFamilyCoverageRow {
    pub family: String,
    pub admitted_case_count: usize,
    pub out_of_class_rejection_count: usize,
    pub coverage_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedFamilyCoverageMatrix {
    pub rows: Vec<DerivedFamilyCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedFamilyParityRow {
    pub family: String,
    pub mainline_case_count: usize,
    pub branch_local_case_count: usize,
    pub replay_verified_case_count: usize,
    pub branch_local_replay_verified_case_count: usize,
    pub cross_branch_parity_case_count: usize,
    pub parity_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedFamilyParityMatrix {
    pub rows: Vec<DerivedFamilyParityRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedValidatorCoverageRow {
    pub family: String,
    pub validator: String,
    pub phase: String,
    pub passed_count: usize,
    pub source_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedValidatorCoverageReport {
    pub rows: Vec<DerivedValidatorCoverageRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneTwoCounters {
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
pub struct MilestoneTwoBranchLocalParityReport {
    pub mainline_case_count: usize,
    pub branch_local_case_count: usize,
    pub branch_ids: Vec<String>,
    pub branch_local_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneTwoReplayParityReport {
    pub replay_checked_case_count: usize,
    pub replay_verified_case_count: usize,
    pub replay_mismatch_case_count: usize,
    pub branch_local_replay_checked_case_count: usize,
    pub branch_local_replay_verified_case_count: usize,
    pub replay_closure_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilestoneTwoDerivedReadReport {
    pub materialized_topology_digest: DeterministicDigest,
    pub interpreted_topology_digest: DeterministicDigest,
    pub derived_validation_digest: DeterministicDigest,
    pub derived_invalidation_report: DerivedInvalidationReport,
    pub derived_rebuild_report: DerivedRebuildReport,
    pub derived_fallback_report: DerivedFallbackReport,
    pub derived_equivalence_contract_report: DerivedEquivalenceContractReport,
    pub derived_branch_local_parity_report: BranchLocalTopologyReport,
    pub derived_replay_parity_report: ReplayParityReport,
    pub milestone_2_counter_report: MilestoneTwoCounters,
    pub read_artifact: TopologyReadArtifact,
    pub certified_interpretation: CertifiedTopologyInterpretation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilestoneTwoDerivedCorpusReport {
    pub materialized_topology_digest: DeterministicDigest,
    pub interpreted_topology_digest: DeterministicDigest,
    pub derived_validation_digest: DeterministicDigest,
    pub derived_truth_basis_digest: DeterministicDigest,
    pub derived_family_coverage_matrix: DerivedFamilyCoverageMatrix,
    pub derived_family_parity_matrix: DerivedFamilyParityMatrix,
    pub derived_branch_local_parity_report: MilestoneTwoBranchLocalParityReport,
    pub derived_replay_parity_report: MilestoneTwoReplayParityReport,
    pub derived_bridge_family_coverage_report: BridgeFamilyCoverageReport,
    pub bridge_routing_digest: DeterministicDigest,
    pub bridge_historical_evaluation_digest: DeterministicDigest,
    pub milestone_2_counter_report: MilestoneTwoCounters,
    pub primitive_corpus: PrimitiveCorpusReport,
    pub bridge_proof_report: BridgeProofReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedInvalidationAggregateRow {
    pub family: String,
    pub target: String,
    pub bridge_scope: String,
    pub source_count: usize,
    pub triggered_case_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedInvalidationAggregateReport {
    pub touched_aspect_count: usize,
    pub triggered_target_count: usize,
    pub rows: Vec<DerivedInvalidationAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedRebuildAggregateRow {
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
pub struct DerivedRebuildAggregateReport {
    pub rows: Vec<DerivedRebuildAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedFallbackAggregateRow {
    pub family: String,
    pub source_count: usize,
    pub whole_view_materialization_count: usize,
    pub explicit_fallback_count: usize,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedFallbackAggregateReport {
    pub rows: Vec<DerivedFallbackAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedEquivalenceContractAggregateRow {
    pub source: String,
    pub family: String,
    pub truth_basis_digest_hex: String,
    pub touched_aspect_count: usize,
    pub triggered_invalidation_target_count: usize,
    pub materialized_topology_digest: DeterministicDigest,
    pub interpreted_topology_digest: DeterministicDigest,
    pub derived_validation_digest: DeterministicDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedEquivalenceContractAggregateReport {
    pub rows: Vec<DerivedEquivalenceContractAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilestoneTwoCloseoutReport {
    pub materialized_topology_digest: DeterministicDigest,
    pub interpreted_topology_digest: DeterministicDigest,
    pub derived_validation_digest: DeterministicDigest,
    pub derived_truth_basis_digest: DeterministicDigest,
    pub bridge_routing_digest: DeterministicDigest,
    pub bridge_historical_evaluation_digest: DeterministicDigest,
    pub derived_family_coverage_matrix: DerivedFamilyCoverageMatrix,
    pub derived_family_parity_matrix: DerivedFamilyParityMatrix,
    pub derived_validator_coverage_report: DerivedValidatorCoverageReport,
    pub derived_invalidation_report: DerivedInvalidationAggregateReport,
    pub derived_rebuild_report: DerivedRebuildAggregateReport,
    pub derived_equivalence_contract_report: DerivedEquivalenceContractAggregateReport,
    pub derived_fallback_report: DerivedFallbackAggregateReport,
    pub derived_failure_locality_report: FailureLocalityReport,
    pub derived_branch_local_parity_report: MilestoneTwoBranchLocalParityReport,
    pub derived_replay_parity_report: MilestoneTwoReplayParityReport,
    pub derived_bridge_family_coverage_report: BridgeFamilyCoverageReport,
    pub milestone_2_counter_report: MilestoneTwoCounters,
    pub derived_corpus: MilestoneTwoDerivedCorpusReport,
}
