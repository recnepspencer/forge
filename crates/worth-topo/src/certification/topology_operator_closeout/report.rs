use crate::certification::{DeterministicDigest, ReplayParityStatus};
use crate::derived_topology::materialized_graph::MaterializationFallbackClass;
use crate::topology_operators::{
    NamingMutationContinuityMatrix, RejectedMutationScopeReport, TopologyDerivedRegion,
    TopologyMutationChangedScope, TopologyMutationDerivedFallbackPolicy, TopologyMutationDigest,
    TopologyMutationFamily, TopologyMutationNamingOutcome, TopologyMutationNamingScope,
    TopologyMutationRejectionClass,
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
use super::query_traversal_proof::MilestoneThreeMutationTopologyQueryTraversalRow;
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
pub struct MilestoneThreeMutationReplayStepRow {
    pub step_index: usize,
    pub mutation_families: Vec<TopologyMutationFamily>,
    pub topology_mutation_digest: TopologyMutationDigest,
    pub naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    pub outcome_class: MilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<TopologyMutationRejectionClass>,
    pub resulting_materialized_topology_digest: Option<DeterministicDigest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeMutationReplayParityReport {
    pub replay_checked: bool,
    pub parity_status: ReplayParityStatus,
    pub mismatch_count: usize,
    pub step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
    pub replay_step_rows: Vec<MilestoneThreeMutationReplayStepRow>,
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
    pub mutation_families: Vec<TopologyMutationFamily>,
    pub bowtie_adjacent_witness: Option<MilestoneThreeBowtieAdjacentWitness>,
    pub ambiguous_local_rewire_witness: Option<MilestoneThreeAmbiguousLocalRewireWitness>,
    pub split_collapse_churn_witness: Option<MilestoneThreeSplitCollapseChurnWitness>,
    pub broken_radial_witness: Option<MilestoneThreeBrokenRadialWitness>,
    pub topology_mutation_digest: TopologyMutationDigest,
    pub naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    pub continuity_outcome_class: TopologyMutationNamingOutcome,
    pub continuity_rejection_class: Option<TopologyMutationRejectionClass>,
    pub outcome_class: MilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<TopologyMutationRejectionClass>,
    pub rejected_mutation_scope_report: Option<RejectedMutationScopeReport>,
    pub derived_validation_report: Option<DerivedTopologyValidationReport>,
    pub derived_materialization_fallback_class: Option<MaterializationFallbackClass>,
    pub mutation_replay_parity_report: MilestoneThreeMutationReplayParityReport,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileCoverageRow {
    pub scenario: MilestoneThreeHostileScenario,
    pub outcome_class: MilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<TopologyMutationRejectionClass>,
    pub continuity_outcome_class: TopologyMutationNamingOutcome,
    pub continuity_rejection_class: Option<TopologyMutationRejectionClass>,
    pub replay_checked: bool,
    pub replay_parity_status: ReplayParityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileFamilyCoverageRow {
    pub family: TopologyMutationFamily,
    pub scenario_count: usize,
    pub scenarios: Vec<MilestoneThreeHostileScenario>,
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileRejectionDistributionRow {
    pub rejection_class: TopologyMutationRejectionClass,
    pub case_count: usize,
    pub scenarios: Vec<MilestoneThreeHostileScenario>,
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileNamingDistributionRow {
    pub continuity_outcome_class: TopologyMutationNamingOutcome,
    pub case_count: usize,
    pub scenarios: Vec<MilestoneThreeHostileScenario>,
    pub row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeTopologyMutationDigestRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) topology_mutation_digest: TopologyMutationDigest,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeNamingContinuityMatrixRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    pub(crate) continuity_outcome_class: TopologyMutationNamingOutcome,
    pub(crate) continuity_rejection_class: Option<TopologyMutationRejectionClass>,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeRejectedMutationScopeReportRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) rejection_class: TopologyMutationRejectionClass,
    pub(crate) rejected_mutation_scope_report: RejectedMutationScopeReport,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeMutationReplayParityRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) replay_checked: bool,
    pub(crate) parity_status: ReplayParityStatus,
    pub(crate) mismatch_count: usize,
    pub(crate) step_count: usize,
    pub(crate) replay_step_count: usize,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeMutationBranchLocalParityRow {
    pub(crate) scenario: Option<MilestoneThreeHostileScenario>,
    pub(crate) branch_label: String,
    pub(crate) branch_id: String,
    pub(crate) mutation_origin: String,
    pub(crate) outcome_class: MilestoneThreeHostileOutcomeClass,
    pub(crate) rejection_class: Option<TopologyMutationRejectionClass>,
    pub(crate) mutation_families: Vec<TopologyMutationFamily>,
    pub(crate) topology_mutation_digest: TopologyMutationDigest,
    pub(crate) naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    pub(crate) branch_head_diverged_from_main: bool,
    pub(crate) branch_head_unchanged_after_rejection: bool,
    pub(crate) branch_truth_digest: Option<DeterministicDigest>,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MilestoneThreeValidatorFamily {
    MutationLocalContinuity,
    NamingContinuity,
    DerivedValidationInspection,
    RejectionLocality,
}

impl MilestoneThreeValidatorFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MutationLocalContinuity => "mutation_local_continuity",
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
    pub(crate) mutation_family_count: usize,
    pub(crate) changed_scope_count: usize,
    pub(crate) naming_scope_count: usize,
    pub(crate) derived_region_count: usize,
    pub(crate) derived_validation_row_count: usize,
    pub(crate) localized_rejection_boundary: bool,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeChangedScopeCoverageRow {
    pub(crate) changed_scope: TopologyMutationChangedScope,
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
    StableMutationOrder,
    StableMutationDigest,
    StableRejectionClassification,
    AmbiguousTieBreakEvidence,
}

impl MilestoneThreeDeterminismRuleKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableMutationOrder => "stable_mutation_order",
            Self::StableMutationDigest => "stable_mutation_digest",
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
pub struct MilestoneThreeMutationBreadthCounterRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) mutation_record_count: usize,
    pub(crate) family_count: usize,
    pub(crate) changed_scope_count: usize,
    pub(crate) naming_scope_count: usize,
    pub(crate) derived_region_count: usize,
    pub(crate) replay_step_count: usize,
    pub(crate) replay_checked: bool,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MilestoneThreeMutationFalloutClass {
    Localized,
    Widened,
    WholeViewFallback,
    WholeHistoryFallback,
    RejectedBeforeDerivedWork,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeMutationFalloutBreadthRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) fallout_class: MilestoneThreeMutationFalloutClass,
    pub(crate) fallback_policy: TopologyMutationDerivedFallbackPolicy,
    pub(crate) fallback_policy_exceeded: bool,
    pub(crate) fallback_rejection_class: Option<TopologyMutationRejectionClass>,
    pub(crate) declared_derived_region_count: usize,
    pub(crate) derived_validation_row_count: usize,
    pub(crate) fallback_count: usize,
    pub(crate) locality_claim_mismatch: bool,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeFailureLocalityRow {
    pub(crate) scenario: MilestoneThreeHostileScenario,
    pub(crate) rejection_class: TopologyMutationRejectionClass,
    pub(crate) scope_row_count: usize,
    pub(crate) families: Vec<TopologyMutationFamily>,
    pub(crate) changed_scopes: Vec<TopologyMutationChangedScope>,
    pub(crate) naming_scopes: Vec<TopologyMutationNamingScope>,
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
    pub topology_mutation_digest_rows: Vec<MilestoneThreeTopologyMutationDigestRow>,
    pub naming_mutation_continuity_matrix_rows: Vec<MilestoneThreeNamingContinuityMatrixRow>,
    pub naming_continuity_breadth_rows: Vec<MilestoneThreeNamingContinuityBreadthRow>,
    pub rejected_mutation_scope_report_rows: Vec<MilestoneThreeRejectedMutationScopeReportRow>,
    pub mutation_replay_parity_rows: Vec<MilestoneThreeMutationReplayParityRow>,
    pub mutation_branch_local_parity_rows: Vec<MilestoneThreeMutationBranchLocalParityRow>,
    pub replay_branch_breadth_rows: Vec<MilestoneThreeReplayBranchBreadthRow>,
    pub mutation_query_traversal_rows: Vec<MilestoneThreeMutationTopologyQueryTraversalRow>,
    pub validator_family_coverage_rows: Vec<MilestoneThreeValidatorFamilyCoverageRow>,
    pub validation_breadth_rows: Vec<MilestoneThreeValidationBreadthRow>,
    pub changed_scope_coverage_rows: Vec<MilestoneThreeChangedScopeCoverageRow>,
    pub derived_region_coverage_rows: Vec<MilestoneThreeDerivedRegionCoverageRow>,
    pub determinism_rule_rows: Vec<MilestoneThreeDeterminismRuleRow>,
    pub mutation_breadth_counter_rows: Vec<MilestoneThreeMutationBreadthCounterRow>,
    pub mutation_fallout_breadth_rows: Vec<MilestoneThreeMutationFalloutBreadthRow>,
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
