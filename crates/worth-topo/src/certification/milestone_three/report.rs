use crate::certification::{WorthDeterministicDigest, WorthReplayParityStatus};
use crate::edit::{
    WorthNamingEditContinuityMatrix, WorthRejectedEditScopeReport, WorthTopologyEditDigest,
    WorthTopologyEditFamily, WorthTopologyEditNamingOutcome, WorthTopologyEditRejectionClass,
};
use serde::{Deserialize, Serialize};
use worth_schema::facade::topology_authoring::WorthMilestoneOnePrimitiveCase;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorthMilestoneThreeHostileScenario {
    BowtieAdjacentRewire,
    CancellationChainParity,
    AmbiguousLocalRewireContinuity,
    BrokenRadialLocalization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorthMilestoneThreeHostileOutcomeClass {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeBowtieAdjacentWitness {
    pub source_half_edge_identity: String,
    pub target_half_edge_identity: String,
    pub source_edge_identity: String,
    pub target_edge_identity: String,
    pub shared_vertex_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeAmbiguousLocalRewireWitness {
    pub moved_half_edge_identity: String,
    pub alternate_moved_half_edge_identity: String,
    pub old_successor_identity: String,
    pub alternate_old_successor_identity: String,
    pub chosen_successor_identity: String,
    pub alternate_successor_identity: String,
    pub chosen_materialized_topology_digest: WorthDeterministicDigest,
    pub alternate_materialized_topology_digest: WorthDeterministicDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeBrokenRadialWitness {
    pub source_half_edge_identity: String,
    pub current_target_half_edge_identity: String,
    pub illegal_target_half_edge_identity: String,
    pub source_edge_identity: String,
    pub current_target_edge_identity: String,
    pub illegal_target_edge_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeEditReplayStepRow {
    pub step_index: usize,
    pub edit_families: Vec<WorthTopologyEditFamily>,
    pub topology_edit_digest: WorthTopologyEditDigest,
    pub naming_edit_continuity_matrix: WorthNamingEditContinuityMatrix,
    pub outcome_class: WorthMilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<WorthTopologyEditRejectionClass>,
    pub resulting_materialized_topology_digest: Option<WorthDeterministicDigest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeEditReplayParityReport {
    pub replay_checked: bool,
    pub parity_status: WorthReplayParityStatus,
    pub mismatch_count: usize,
    pub step_rows: Vec<WorthMilestoneThreeEditReplayStepRow>,
    pub replay_step_rows: Vec<WorthMilestoneThreeEditReplayStepRow>,
    pub baseline_materialized_topology_digest: Option<WorthDeterministicDigest>,
    pub final_materialized_topology_digest: Option<WorthDeterministicDigest>,
    pub replay_final_materialized_topology_digest: Option<WorthDeterministicDigest>,
    pub returned_to_baseline: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeHostileScenarioReport {
    pub scenario: WorthMilestoneThreeHostileScenario,
    pub primitive_family: String,
    pub primitive: WorthMilestoneOnePrimitiveCase,
    pub edit_families: Vec<WorthTopologyEditFamily>,
    pub bowtie_adjacent_witness: Option<WorthMilestoneThreeBowtieAdjacentWitness>,
    pub ambiguous_local_rewire_witness: Option<WorthMilestoneThreeAmbiguousLocalRewireWitness>,
    pub broken_radial_witness: Option<WorthMilestoneThreeBrokenRadialWitness>,
    pub topology_edit_digest: WorthTopologyEditDigest,
    pub naming_edit_continuity_matrix: WorthNamingEditContinuityMatrix,
    pub continuity_outcome_class: WorthTopologyEditNamingOutcome,
    pub continuity_rejection_class: Option<WorthTopologyEditRejectionClass>,
    pub outcome_class: WorthMilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<WorthTopologyEditRejectionClass>,
    pub rejected_edit_scope_report: Option<WorthRejectedEditScopeReport>,
    pub edit_replay_parity_report: WorthMilestoneThreeEditReplayParityReport,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeHostileCoverageRow {
    pub scenario: WorthMilestoneThreeHostileScenario,
    pub outcome_class: WorthMilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<WorthTopologyEditRejectionClass>,
    pub continuity_outcome_class: WorthTopologyEditNamingOutcome,
    pub continuity_rejection_class: Option<WorthTopologyEditRejectionClass>,
    pub replay_checked: bool,
    pub replay_parity_status: WorthReplayParityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeHostileFamilyCoverageRow {
    pub family: WorthTopologyEditFamily,
    pub scenario_count: usize,
    pub scenarios: Vec<WorthMilestoneThreeHostileScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeHostileRejectionDistributionRow {
    pub rejection_class: WorthTopologyEditRejectionClass,
    pub case_count: usize,
    pub scenarios: Vec<WorthMilestoneThreeHostileScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeHostileNamingDistributionRow {
    pub continuity_outcome_class: WorthTopologyEditNamingOutcome,
    pub case_count: usize,
    pub scenarios: Vec<WorthMilestoneThreeHostileScenario>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthMilestoneThreeHostileSuiteReport {
    pub scenario_reports: Vec<WorthMilestoneThreeHostileScenarioReport>,
    pub coverage_rows: Vec<WorthMilestoneThreeHostileCoverageRow>,
    pub family_coverage_rows: Vec<WorthMilestoneThreeHostileFamilyCoverageRow>,
    pub rejection_distribution_rows: Vec<WorthMilestoneThreeHostileRejectionDistributionRow>,
    pub naming_distribution_rows: Vec<WorthMilestoneThreeHostileNamingDistributionRow>,
    pub missing_required_scenarios: Vec<String>,
    pub implemented_scenario_count: usize,
    pub required_scenario_count: usize,
    pub coverage_complete: bool,
}
