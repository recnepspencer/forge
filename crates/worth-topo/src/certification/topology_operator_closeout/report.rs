use crate::certification::{DeterministicDigest, ReplayParityStatus};
use crate::derived_topology::materialized_graph::MaterializationFallbackClass;
use crate::topology_operators::{
    NamingEditContinuityMatrix, RejectedEditScopeReport, TopologyDerivedRegion,
    TopologyEditChangedScope, TopologyEditDerivedFallbackPolicy, TopologyEditDigest,
    TopologyEditFamily, TopologyEditNamingOutcome, TopologyEditNamingScope,
    TopologyEditRejectionClass,
};
use crate::validation::DerivedTopologyValidationReport;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde::{Deserialize, Serialize};

use super::derived_fallout::{
    MilestoneThreeDerivedFallbackPolicyDenialRow, MilestoneThreeDerivedReuseLegalityRow,
    MilestoneThreeDerivedWorkBreadthRow,
};
use super::hostile_categories::MilestoneThreeHostileCertificationCategoryRow;
use super::naming_continuity_breadth_row::MilestoneThreeNamingContinuityBreadthRow;
use super::operator_family_proof::{
    MilestoneThreeOperatorFamilyClosureRow, MilestoneThreePrimitiveFamilyClosureRow,
};
use super::query_traversal_proof::MilestoneThreeEditedTopologyQueryTraversalRow;
use super::replay_branch_breadth_row::MilestoneThreeReplayBranchBreadthRow;
use super::scale_pressure_proof::MilestoneThreeScalePressureRow;
use super::side_quest_gate::{
    MilestoneThreeReturnGateBlockerRow, MilestoneThreeSideQuestCloseoutReport,
};
use super::validation_breadth_row::MilestoneThreeValidationBreadthRow;

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
    pub derived_validation_report: Option<DerivedTopologyValidationReport>,
    pub derived_materialization_fallback_class: Option<MaterializationFallbackClass>,
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
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileRejectionDistributionRow {
    pub rejection_class: TopologyEditRejectionClass,
    pub case_count: usize,
    pub scenarios: Vec<MilestoneThreeHostileScenario>,
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileNamingDistributionRow {
    pub continuity_outcome_class: TopologyEditNamingOutcome,
    pub case_count: usize,
    pub scenarios: Vec<MilestoneThreeHostileScenario>,
    pub row_digest: String,
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
pub struct MilestoneThreeEditBranchLocalParityRow {
    pub(crate) scenario: Option<MilestoneThreeHostileScenario>,
    pub(crate) branch_label: String,
    pub(crate) branch_id: String,
    pub(crate) mutation_origin: String,
    pub(crate) outcome_class: MilestoneThreeHostileOutcomeClass,
    pub(crate) rejection_class: Option<TopologyEditRejectionClass>,
    pub(crate) edit_families: Vec<TopologyEditFamily>,
    pub(crate) topology_edit_digest: TopologyEditDigest,
    pub(crate) naming_edit_continuity_matrix: NamingEditContinuityMatrix,
    pub(crate) branch_head_diverged_from_main: bool,
    pub(crate) branch_head_unchanged_after_rejection: bool,
    pub(crate) branch_truth_digest: Option<DeterministicDigest>,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MilestoneThreeValidatorFamily {
    EditLocalContinuity,
    NamingContinuity,
    DerivedValidationInspection,
    RejectionLocality,
}

impl MilestoneThreeValidatorFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditLocalContinuity => "edit_local_continuity",
            Self::NamingContinuity => "naming_continuity",
            Self::DerivedValidationInspection => "derived_validation_inspection",
            Self::RejectionLocality => "rejection_locality",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeValidatorFamilyCoverageRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) validator_family: MilestoneThreeValidatorFamily,
    pub(crate) validator_names: Vec<String>,
    pub(crate) edit_family_count: usize,
    pub(crate) changed_scope_count: usize,
    pub(crate) naming_scope_count: usize,
    pub(crate) derived_region_count: usize,
    pub(crate) derived_validation_row_count: usize,
    pub(crate) localized_rejection_boundary: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MilestoneThreeDeterminismRuleKind {
    StableEditOrder,
    StableEditDigest,
    StableRejectionClassification,
    AmbiguousTieBreakEvidence,
}

impl MilestoneThreeDeterminismRuleKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableEditOrder => "stable_edit_order",
            Self::StableEditDigest => "stable_edit_digest",
            Self::StableRejectionClassification => "stable_rejection_classification",
            Self::AmbiguousTieBreakEvidence => "ambiguous_tie_break_evidence",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeDeterminismRuleRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) rule_kind: MilestoneThreeDeterminismRuleKind,
    pub(crate) evidence_count: usize,
    pub(crate) replay_verified: bool,
    pub(crate) diagnostic_classification_stable: bool,
    pub(crate) tie_break_evidence_stable: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MilestoneThreeEditFalloutClass {
    Localized,
    Widened,
    WholeViewFallback,
    WholeHistoryFallback,
    RejectedBeforeDerivedWork,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeEditFalloutBreadthRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) fallout_class: MilestoneThreeEditFalloutClass,
    pub(crate) fallback_policy: TopologyEditDerivedFallbackPolicy,
    pub(crate) fallback_policy_exceeded: bool,
    pub(crate) fallback_rejection_class: Option<TopologyEditRejectionClass>,
    pub(crate) declared_derived_region_count: usize,
    pub(crate) derived_validation_row_count: usize,
    pub(crate) fallback_count: usize,
    pub(crate) locality_claim_mismatch: bool,
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
pub struct MilestoneThreeHostileSuiteReport {
    pub scenario_reports: Vec<MilestoneThreeHostileScenarioReport>,
    pub coverage_rows: Vec<MilestoneThreeHostileCoverageRow>,
    pub family_coverage_rows: Vec<MilestoneThreeHostileFamilyCoverageRow>,
    pub rejection_distribution_rows: Vec<MilestoneThreeHostileRejectionDistributionRow>,
    pub naming_distribution_rows: Vec<MilestoneThreeHostileNamingDistributionRow>,
    pub hostile_certification_category_rows: Vec<MilestoneThreeHostileCertificationCategoryRow>,
    pub operator_family_closure_rows: Vec<MilestoneThreeOperatorFamilyClosureRow>,
    pub primitive_family_closure_rows: Vec<MilestoneThreePrimitiveFamilyClosureRow>,
    pub scale_pressure_rows: Vec<MilestoneThreeScalePressureRow>,
    pub topology_edit_digest_rows: Vec<MilestoneThreeTopologyEditDigestRow>,
    pub naming_edit_continuity_matrix_rows: Vec<MilestoneThreeNamingContinuityMatrixRow>,
    pub naming_continuity_breadth_rows: Vec<MilestoneThreeNamingContinuityBreadthRow>,
    pub rejected_edit_scope_report_rows: Vec<MilestoneThreeRejectedEditScopeReportRow>,
    pub edit_replay_parity_rows: Vec<MilestoneThreeEditReplayParityRow>,
    pub edit_branch_local_parity_rows: Vec<MilestoneThreeEditBranchLocalParityRow>,
    pub replay_branch_breadth_rows: Vec<MilestoneThreeReplayBranchBreadthRow>,
    pub edited_query_traversal_rows: Vec<MilestoneThreeEditedTopologyQueryTraversalRow>,
    pub validator_family_coverage_rows: Vec<MilestoneThreeValidatorFamilyCoverageRow>,
    pub validation_breadth_rows: Vec<MilestoneThreeValidationBreadthRow>,
    pub changed_scope_coverage_rows: Vec<MilestoneThreeChangedScopeCoverageRow>,
    pub derived_region_coverage_rows: Vec<MilestoneThreeDerivedRegionCoverageRow>,
    pub determinism_rule_rows: Vec<MilestoneThreeDeterminismRuleRow>,
    pub edit_breadth_counter_rows: Vec<MilestoneThreeEditBreadthCounterRow>,
    pub edit_fallout_breadth_rows: Vec<MilestoneThreeEditFalloutBreadthRow>,
    pub derived_fallback_policy_denial_rows: Vec<MilestoneThreeDerivedFallbackPolicyDenialRow>,
    pub derived_reuse_legality_rows: Vec<MilestoneThreeDerivedReuseLegalityRow>,
    pub derived_work_breadth_rows: Vec<MilestoneThreeDerivedWorkBreadthRow>,
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




