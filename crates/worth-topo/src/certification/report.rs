use forge_relational::facade::diagnostics::DiagnosticCode;
use forge_relational::facade::errors::ErrorContext;
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::replay::{ReplayFailureClass, ReplayObservableSurface};
use serde::{Deserialize, Serialize};
use worth_schema::facade::{
    CertifiedTopologyInterpretation, WorthMilestoneOnePrimitiveCase,
    WorthMilestoneOnePrimitiveExpectedOutcome, WorthMilestoneOnePrimitiveRole,
    WorthMutationOrigin, WorthTopologyReadArtifact,
};
use crate::validators::WorthTopologyValidationReport;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthPrimitiveCorpusReport {
    pub coverage_matrix: WorthPrimitiveCorpusCoverageMatrix,
    pub parity_report: WorthPrimitiveCorpusParityReport,
    pub cases: Vec<WorthPrimitiveCorpusCaseReport>,
    pub rejected_cases: Vec<WorthPrimitiveCorpusRejectedCaseReport>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthBridgeProofReport {
    pub bridge_routing_digest: WorthDeterministicDigest,
    pub bridge_historical_evaluation_digest: WorthDeterministicDigest,
    pub route_record_count: usize,
    pub historical_evaluation_record_count: usize,
    pub source_branch: String,
    pub source_commit: String,
    pub source_snapshot: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthMilestoneOneCloseoutReport {
    pub seeded_bootstrap: WorthMilestoneOneCertificationReport,
    pub primitive_corpus: WorthPrimitiveCorpusReport,
    pub bridge_proof: WorthBridgeProofReport,
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
    pub diagnostic_code: Option<DiagnosticCode>,
    pub detail: String,
    pub fields_json: Option<String>,
    pub context: Option<ErrorContext>,
}
