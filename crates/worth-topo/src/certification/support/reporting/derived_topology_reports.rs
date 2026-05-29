use super::*;

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
