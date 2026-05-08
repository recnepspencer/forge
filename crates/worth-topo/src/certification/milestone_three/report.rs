use crate::certification::{DeterministicDigest, ReplayParityStatus};
use crate::edit::{
    NamingEditContinuityMatrix, RejectedEditScopeReport, TopologyDerivedRegion,
    TopologyEditChangedScope, TopologyEditDigest, TopologyEditFamily, TopologyEditNamingOutcome,
    TopologyEditNamingScope, TopologyEditRejectionClass,
};
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MilestoneThreeHostileScenario {
    BowtieAdjacentRewire,
    CancellationChainParity,
    SplitCollapseChurn,
    AmbiguousLocalRewireContinuity,
    BrokenRadialLocalization,
}

impl MilestoneThreeHostileScenario {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BowtieAdjacentRewire => "BowtieAdjacentRewire",
            Self::CancellationChainParity => "CancellationChainParity",
            Self::SplitCollapseChurn => "SplitCollapseChurn",
            Self::AmbiguousLocalRewireContinuity => "AmbiguousLocalRewireContinuity",
            Self::BrokenRadialLocalization => "BrokenRadialLocalization",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MilestoneThreeHostileOutcomeClass {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeBowtieAdjacentWitness {
    pub source_half_edge_identity: String,
    pub target_half_edge_identity: String,
    pub source_edge_identity: String,
    pub target_edge_identity: String,
    pub shared_vertex_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeAmbiguousLocalRewireWitness {
    pub moved_half_edge_identity: String,
    pub alternate_moved_half_edge_identity: String,
    pub old_successor_identity: String,
    pub alternate_old_successor_identity: String,
    pub chosen_successor_identity: String,
    pub alternate_successor_identity: String,
    pub chosen_materialized_topology_digest: DeterministicDigest,
    pub alternate_materialized_topology_digest: DeterministicDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeBrokenRadialWitness {
    pub source_half_edge_identity: String,
    pub current_target_half_edge_identity: String,
    pub illegal_target_half_edge_identity: String,
    pub source_edge_identity: String,
    pub current_target_edge_identity: String,
    pub illegal_target_edge_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeSplitCollapseChurnWitness {
    pub original_wire_identity: String,
    pub split_wire_identity: String,
    pub collapse_wire_identity: String,
    pub moved_half_edge_identities: Vec<String>,
    pub retained_half_edge_identities: Vec<String>,
    pub split_step_wire_count: usize,
    pub final_wire_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeEditReplayStepRow {
    pub step_index: usize,
    pub edit_families: Vec<TopologyEditFamily>,
    pub topology_edit_digest: TopologyEditDigest,
    pub naming_edit_continuity_matrix: NamingEditContinuityMatrix,
    pub outcome_class: MilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<TopologyEditRejectionClass>,
    pub resulting_materialized_topology_digest: Option<DeterministicDigest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeEditReplayParityReport {
    pub replay_checked: bool,
    pub parity_status: ReplayParityStatus,
    pub mismatch_count: usize,
    pub step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    pub replay_step_rows: Vec<MilestoneThreeEditReplayStepRow>,
    pub baseline_materialized_topology_digest: Option<DeterministicDigest>,
    pub final_materialized_topology_digest: Option<DeterministicDigest>,
    pub replay_final_materialized_topology_digest: Option<DeterministicDigest>,
    pub returned_to_baseline: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileScenarioReport {
    pub scenario: MilestoneThreeHostileScenario,
    pub primitive_family: String,
    pub primitive: MilestoneOnePrimitiveCase,
    pub edit_families: Vec<TopologyEditFamily>,
    pub bowtie_adjacent_witness: Option<MilestoneThreeBowtieAdjacentWitness>,
    pub ambiguous_local_rewire_witness: Option<MilestoneThreeAmbiguousLocalRewireWitness>,
    pub split_collapse_churn_witness: Option<MilestoneThreeSplitCollapseChurnWitness>,
    pub broken_radial_witness: Option<MilestoneThreeBrokenRadialWitness>,
    pub topology_edit_digest: TopologyEditDigest,
    pub naming_edit_continuity_matrix: NamingEditContinuityMatrix,
    pub continuity_outcome_class: TopologyEditNamingOutcome,
    pub continuity_rejection_class: Option<TopologyEditRejectionClass>,
    pub outcome_class: MilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<TopologyEditRejectionClass>,
    pub rejected_edit_scope_report: Option<RejectedEditScopeReport>,
    pub edit_replay_parity_report: MilestoneThreeEditReplayParityReport,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileCoverageRow {
    pub scenario: MilestoneThreeHostileScenario,
    pub outcome_class: MilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<TopologyEditRejectionClass>,
    pub continuity_outcome_class: TopologyEditNamingOutcome,
    pub continuity_rejection_class: Option<TopologyEditRejectionClass>,
    pub replay_checked: bool,
    pub replay_parity_status: ReplayParityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileFamilyCoverageRow {
    pub family: TopologyEditFamily,
    pub scenario_count: usize,
    pub scenarios: Vec<MilestoneThreeHostileScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileRejectionDistributionRow {
    pub rejection_class: TopologyEditRejectionClass,
    pub case_count: usize,
    pub scenarios: Vec<MilestoneThreeHostileScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileNamingDistributionRow {
    pub continuity_outcome_class: TopologyEditNamingOutcome,
    pub case_count: usize,
    pub scenarios: Vec<MilestoneThreeHostileScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeTopologyEditDigestRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) topology_edit_digest: TopologyEditDigest,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeNamingContinuityMatrixRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) naming_edit_continuity_matrix: NamingEditContinuityMatrix,
    pub(crate) continuity_outcome_class: TopologyEditNamingOutcome,
    pub(crate) continuity_rejection_class: Option<TopologyEditRejectionClass>,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeRejectedEditScopeReportRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) rejection_class: TopologyEditRejectionClass,
    pub(crate) rejected_edit_scope_report: RejectedEditScopeReport,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeEditReplayParityRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) replay_checked: bool,
    pub(crate) parity_status: ReplayParityStatus,
    pub(crate) mismatch_count: usize,
    pub(crate) step_count: usize,
    pub(crate) replay_step_count: usize,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeChangedScopeCoverageRow {
    pub(crate) changed_scope: TopologyEditChangedScope,
    pub(crate) scenario_count: usize,
    pub(crate) scenarios: Vec<MilestoneThreeHostileScenario>,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeDerivedRegionCoverageRow {
    pub(crate) derived_region: TopologyDerivedRegion,
    pub(crate) scenario_count: usize,
    pub(crate) scenarios: Vec<MilestoneThreeHostileScenario>,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeEditBreadthCounterRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) contract_count: usize,
    pub(crate) family_count: usize,
    pub(crate) changed_scope_count: usize,
    pub(crate) naming_scope_count: usize,
    pub(crate) derived_region_count: usize,
    pub(crate) replay_step_count: usize,
    pub(crate) replay_checked: bool,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeFailureLocalityRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) rejection_class: TopologyEditRejectionClass,
    pub(crate) scope_row_count: usize,
    pub(crate) families: Vec<TopologyEditFamily>,
    pub(crate) changed_scopes: Vec<TopologyEditChangedScope>,
    pub(crate) naming_scopes: Vec<TopologyEditNamingScope>,
    pub(crate) derived_regions: Vec<TopologyDerivedRegion>,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeSideQuestContractRow {
    pub contract_name: String,
    pub status: String,
    pub reason: String,
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeSideQuestBlockerRow {
    pub blocker_name: String,
    pub status: String,
    pub reason: String,
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeSideQuestCloseoutReport {
    pub domain_read_request_count: usize,
    pub domain_read_parity_count: usize,
    pub replay_checked_count: usize,
    pub replay_verified_count: usize,
    pub branch_local_checked_count: usize,
    pub branch_local_verified_count: usize,
    pub contract_rows: Vec<MilestoneThreeSideQuestContractRow>,
    pub blocker_rows: Vec<MilestoneThreeSideQuestBlockerRow>,
    pub phase_three_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeReturnGateBlockerRow {
    pub blocker_name: String,
    pub reason: String,
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileSuiteReport {
    pub scenario_reports: Vec<MilestoneThreeHostileScenarioReport>,
    pub coverage_rows: Vec<MilestoneThreeHostileCoverageRow>,
    pub family_coverage_rows: Vec<MilestoneThreeHostileFamilyCoverageRow>,
    pub rejection_distribution_rows: Vec<MilestoneThreeHostileRejectionDistributionRow>,
    pub naming_distribution_rows: Vec<MilestoneThreeHostileNamingDistributionRow>,
    pub topology_edit_digest_rows: Vec<MilestoneThreeTopologyEditDigestRow>,
    pub naming_edit_continuity_matrix_rows: Vec<MilestoneThreeNamingContinuityMatrixRow>,
    pub rejected_edit_scope_report_rows: Vec<MilestoneThreeRejectedEditScopeReportRow>,
    pub edit_replay_parity_rows: Vec<MilestoneThreeEditReplayParityRow>,
    pub changed_scope_coverage_rows: Vec<MilestoneThreeChangedScopeCoverageRow>,
    pub derived_region_coverage_rows: Vec<MilestoneThreeDerivedRegionCoverageRow>,
    pub edit_breadth_counter_rows: Vec<MilestoneThreeEditBreadthCounterRow>,
    pub failure_locality_rows: Vec<MilestoneThreeFailureLocalityRow>,
    pub side_quest_closeout_report: MilestoneThreeSideQuestCloseoutReport,
    pub side_quest_gate_ready: bool,
    pub missing_required_scenarios: Vec<String>,
    pub milestone_three_return_gate_blocker_rows: Vec<MilestoneThreeReturnGateBlockerRow>,
    pub implemented_scenario_count: usize,
    pub required_scenario_count: usize,
    pub coverage_complete: bool,
    pub milestone_three_return_gate_ready: bool,
}
