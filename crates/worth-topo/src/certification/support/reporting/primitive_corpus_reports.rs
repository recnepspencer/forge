use super::*;

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
