use crate::certification::{DeterministicDigest, ReplayParityStatus};
use crate::derived_topology::materialized_graph::MaterializationFallbackClass;
use crate::topology_operators::{
    NamingMutationContinuityMatrix, RejectedMutationScopeReport,
    TopologyMutationDerivedFallbackPolicy, TopologyMutationDigest, TopologyMutationFamily,
    TopologyMutationNamingOutcome, TopologyMutationRejectionClass,
};
use crate::validation::DerivedTopologyValidationReport;
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
pub struct MilestoneThreeMutationReplayStepRow {
    pub step_index: usize,
    pub mutation_families: Vec<TopologyMutationFamily>,
    pub topology_mutation_digest: TopologyMutationDigest,
    pub naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    pub derived_fallback_policy: Option<TopologyMutationDerivedFallbackPolicy>,
    pub outcome_class: MilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<TopologyMutationRejectionClass>,
    pub resulting_materialized_topology_digest: Option<DeterministicDigest>,
}

impl MilestoneThreeMutationReplayStepRow {
    pub fn fallback_explanation_detail(&self) -> Option<&str> {
        self.derived_fallback_policy
            .map(certification_fallback_explanation_detail_for_policy)
    }
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
pub struct MilestoneThreeScenarioMutationSynopsis {
    pub mutation_families: Vec<TopologyMutationFamily>,
    pub topology_mutation_digest: TopologyMutationDigest,
}

impl MilestoneThreeScenarioMutationSynopsis {
    pub fn mutation_families(&self) -> &[TopologyMutationFamily] {
        &self.mutation_families
    }

    pub fn topology_mutation_digest(&self) -> &TopologyMutationDigest {
        &self.topology_mutation_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeScenarioMutationSemanticSummary {
    pub naming_mutation_continuity_matrix: NamingMutationContinuityMatrix,
    pub derived_fallback_policy: Option<TopologyMutationDerivedFallbackPolicy>,
    pub continuity_outcome_class: TopologyMutationNamingOutcome,
    pub continuity_rejection_class: Option<TopologyMutationRejectionClass>,
}

impl MilestoneThreeScenarioMutationSemanticSummary {
    pub fn naming_mutation_continuity_matrix(&self) -> &NamingMutationContinuityMatrix {
        &self.naming_mutation_continuity_matrix
    }

    pub fn derived_fallback_policy(&self) -> Option<TopologyMutationDerivedFallbackPolicy> {
        self.derived_fallback_policy
    }

    pub fn fallback_explanation_detail(&self) -> Option<&str> {
        self.derived_fallback_policy
            .map(certification_fallback_explanation_detail_for_policy)
    }

    pub fn continuity_outcome_class(&self) -> TopologyMutationNamingOutcome {
        self.continuity_outcome_class
    }

    pub fn continuity_rejection_class(&self) -> Option<TopologyMutationRejectionClass> {
        self.continuity_rejection_class
    }
}

fn certification_fallback_explanation_detail_for_policy(
    policy: TopologyMutationDerivedFallbackPolicy,
) -> &'static str {
    match policy {
        TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback => {
            "declared topology mutation allows explicit fallback when runtime reconciliation needs a non-canonical resolution"
        }
        TopologyMutationDerivedFallbackPolicy::RejectAnyFallback => {
            "declared topology mutation rejects fallback and requires canonical continuity if runtime reconciliation would otherwise drift"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeHostileScenarioReport {
    pub scenario: MilestoneThreeHostileScenario,
    pub primitive_family: String,
    pub primitive: MilestoneOnePrimitiveCase,
    pub declared_mutation_synopsis: MilestoneThreeScenarioMutationSynopsis,
    pub semantic_summary: MilestoneThreeScenarioMutationSemanticSummary,
    pub bowtie_adjacent_witness: Option<MilestoneThreeBowtieAdjacentWitness>,
    pub ambiguous_local_rewire_witness: Option<MilestoneThreeAmbiguousLocalRewireWitness>,
    pub split_collapse_churn_witness: Option<MilestoneThreeSplitCollapseChurnWitness>,
    pub broken_radial_witness: Option<MilestoneThreeBrokenRadialWitness>,
    pub outcome_class: MilestoneThreeHostileOutcomeClass,
    pub rejection_class: Option<TopologyMutationRejectionClass>,
    pub rejected_mutation_scope_report: Option<RejectedMutationScopeReport>,
    pub derived_validation_report: Option<DerivedTopologyValidationReport>,
    pub derived_materialization_fallback_class: Option<MaterializationFallbackClass>,
    pub mutation_replay_parity_report: MilestoneThreeMutationReplayParityReport,
    pub detail: String,
}

impl MilestoneThreeHostileScenarioReport {
    pub fn mutation_families(&self) -> &[TopologyMutationFamily] {
        self.declared_mutation_synopsis.mutation_families()
    }

    pub fn topology_mutation_digest(&self) -> &TopologyMutationDigest {
        self.declared_mutation_synopsis.topology_mutation_digest()
    }

    pub fn naming_mutation_continuity_matrix(&self) -> &NamingMutationContinuityMatrix {
        self.semantic_summary.naming_mutation_continuity_matrix()
    }

    pub fn derived_fallback_policy(&self) -> Option<TopologyMutationDerivedFallbackPolicy> {
        self.semantic_summary.derived_fallback_policy()
    }

    pub fn fallback_explanation_detail(&self) -> Option<&str> {
        self.semantic_summary.fallback_explanation_detail()
    }

    pub fn continuity_outcome_class(&self) -> TopologyMutationNamingOutcome {
        self.semantic_summary.continuity_outcome_class()
    }

    pub fn continuity_rejection_class(&self) -> Option<TopologyMutationRejectionClass> {
        self.semantic_summary.continuity_rejection_class()
    }
}
