mod acceptance_rows;
mod derived_fallout;
mod hostile_categories;
mod mutation_sequence_support;
#[cfg(test)]
mod mutation_tests;
mod naming_continuity_breadth_row;
mod operator_family_proof;
mod query_traversal_proof;
mod replay_branch_breadth_row;
mod replay_step_rows;
mod report;
mod row_accessors;
mod scale_pressure_proof;
mod scenario_programs;
mod shared;
mod side_quest_gate;
mod suite;
#[cfg(all(test, feature = "slow-certification"))]
mod tests;
mod validation_breadth_row;

use report::MilestoneThreeHostileScenario as HostileScenario;

use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologySelectedGraphObligationEnforcementCloseout,
};

pub(crate) use acceptance_rows::milestone_three_validator_expectations;
pub use derived_fallout::{
    MilestoneThreeDerivedFallbackPolicyDenialRow, MilestoneThreeDerivedReuseLegalityRow,
};
pub use derived_fallout::{
    MilestoneThreeDerivedWorkBreadthClass, MilestoneThreeDerivedWorkBreadthRow,
};
pub use hostile_categories::{
    MilestoneThreeHostileCertificationCategory, MilestoneThreeHostileCertificationCategoryRow,
    MilestoneThreeHostileCertificationStatus,
};
pub use naming_continuity_breadth_row::MilestoneThreeNamingContinuityBreadthRow;
pub use operator_family_proof::{
    MilestoneThreeOperatorFamilyClosureRow, MilestoneThreePrimitiveFamilyClosureRow,
};
pub use query_traversal_proof::{
    MilestoneThreeMutationTopologyQueryTraversalRow,
    MilestoneThreeMutationTopologyQueryTraversalView,
};
pub use replay_branch_breadth_row::MilestoneThreeReplayBranchBreadthRow;
pub use report::{
    MilestoneThreeAmbiguousLocalRewireWitness, MilestoneThreeBowtieAdjacentWitness,
    MilestoneThreeBrokenRadialWitness, MilestoneThreeChangedScopeCoverageRow,
    MilestoneThreeDerivedRegionCoverageRow, MilestoneThreeDeterminismRuleKind,
    MilestoneThreeDeterminismRuleRow, MilestoneThreeFailureLocalityRow,
    MilestoneThreeHostileCoverageRow, MilestoneThreeHostileFamilyCoverageRow,
    MilestoneThreeHostileNamingDistributionRow, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileRejectionDistributionRow, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
    MilestoneThreeMutationBranchLocalParityRow, MilestoneThreeMutationBreadthCounterRow,
    MilestoneThreeMutationFalloutBreadthRow, MilestoneThreeMutationFalloutClass,
    MilestoneThreeMutationReplayParityReport, MilestoneThreeMutationReplayParityRow,
    MilestoneThreeMutationReplayStepRow, MilestoneThreeNamingContinuityMatrixRow,
    MilestoneThreeRejectedMutationScopeReportRow, MilestoneThreeScenarioMutationSemanticSummary,
    MilestoneThreeScenarioMutationSynopsis, MilestoneThreeSplitCollapseChurnWitness,
    MilestoneThreeTopologyMutationDigestRow, MilestoneThreeValidatorFamily,
    MilestoneThreeValidatorFamilyCoverageRow,
};
pub use scale_pressure_proof::{MilestoneThreeScalePressureRow, MilestoneThreeScalePressureSweep};
pub(crate) use scenario_programs::{
    certify_milestone_three_ambiguous_local_rewire_continuity_impl,
    certify_milestone_three_bowtie_adjacent_rewire_impl,
    certify_milestone_three_broken_radial_localization_impl,
    certify_milestone_three_cancellation_chain_parity_impl,
    certify_milestone_three_split_collapse_churn_impl,
};
pub use side_quest_gate::{
    MilestoneThreeReturnGateBlockerRow, MilestoneThreeSideQuestBlockerRow,
    MilestoneThreeSideQuestCloseoutReport, MilestoneThreeSideQuestContractRow,
};
pub(crate) use suite::{
    certify_milestone_three_closeout_impl, certify_milestone_three_hostile_suite_impl,
};
pub use validation_breadth_row::MilestoneThreeValidationBreadthRow;

pub fn certify_topology_operator_selected_obligation_cutover_impl(
    enforcement_closeout: &WorthTopologySelectedGraphObligationEnforcementCloseout,
) -> Result<WorthTopologyOperatorCertificationCutoverCloseout, WorthTopologyLegalityCatalogError> {
    WorthTopologyOperatorCertificationCutoverCloseout::from_selected_graph_obligation_enforcement(
        enforcement_closeout,
    )
}

const MILESTONE_THREE_REQUIRED_SCENARIOS: &[HostileScenario] = &[
    HostileScenario::BowtieAdjacentRewire,
    HostileScenario::CancellationChainParity,
    HostileScenario::SplitCollapseChurn,
    HostileScenario::AmbiguousLocalRewireContinuity,
    HostileScenario::BrokenRadialLocalization,
];

const MILESTONE_THREE_REPLAY_SCENARIOS: &[HostileScenario] = &[
    HostileScenario::BowtieAdjacentRewire,
    HostileScenario::CancellationChainParity,
    HostileScenario::SplitCollapseChurn,
    HostileScenario::AmbiguousLocalRewireContinuity,
    HostileScenario::BrokenRadialLocalization,
];

const MILESTONE_THREE_REJECTED_SCENARIOS: &[HostileScenario] = &[
    HostileScenario::BowtieAdjacentRewire,
    HostileScenario::BrokenRadialLocalization,
];

pub(crate) fn milestone_three_required_scenario_names() -> Vec<String> {
    scenario_names(milestone_three_required_scenarios())
}

pub(crate) fn milestone_three_replay_scenario_names() -> Vec<String> {
    scenario_names(milestone_three_replay_scenarios())
}

pub(crate) fn milestone_three_rejected_scenario_names() -> Vec<String> {
    scenario_names(milestone_three_rejected_scenarios())
}

pub(crate) fn milestone_three_required_scenarios() -> &'static [HostileScenario] {
    MILESTONE_THREE_REQUIRED_SCENARIOS
}

pub(crate) fn milestone_three_replay_scenarios() -> &'static [HostileScenario] {
    MILESTONE_THREE_REPLAY_SCENARIOS
}

pub(crate) fn milestone_three_rejected_scenarios() -> &'static [HostileScenario] {
    MILESTONE_THREE_REJECTED_SCENARIOS
}

fn scenario_names(scenarios: &[HostileScenario]) -> Vec<String> {
    scenarios
        .iter()
        .map(|scenario| scenario.as_str().to_string())
        .collect()
}
