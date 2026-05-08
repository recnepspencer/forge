mod accessors;
mod aggregate_acceptance;
mod ambiguous_local_rewire;
mod bowtie_adjacent;
mod branch_local_acceptance;
mod branch_local_parity;
mod broken_radial_localization;
mod cancellation_chain;
#[cfg(test)]
mod contract_tests;
mod determinism_rules;
mod direct_acceptance;
mod edited_query_traversal;
mod edited_query_traversal_accessors;
mod edited_query_traversal_types;
mod hostile_category_accessors;
mod hostile_category_posture;
mod hostile_category_requirements;
mod hostile_category_types;
mod local_successor_rewire;
mod primitive_family_closure;
mod primitive_family_closure_accessors;
mod primitive_family_closure_types;
mod report;
mod shared;
mod side_quest_closeout;
mod side_quest_types;
mod split_collapse_churn;
mod suite;
#[cfg(test)]
mod tests;
mod validator_family_coverage;

use report::MilestoneThreeHostileScenario as HostileScenario;

pub(crate) use ambiguous_local_rewire::certify_milestone_three_ambiguous_local_rewire_continuity_impl;
pub(crate) use bowtie_adjacent::certify_milestone_three_bowtie_adjacent_rewire_impl;
pub(crate) use broken_radial_localization::certify_milestone_three_broken_radial_localization_impl;
pub(crate) use cancellation_chain::certify_milestone_three_cancellation_chain_parity_impl;
pub use edited_query_traversal_types::{
    MilestoneThreeEditedTopologyQueryTraversalRow, MilestoneThreeEditedTopologyQueryTraversalView,
};
pub use hostile_category_types::{
    MilestoneThreeHostileCertificationCategory, MilestoneThreeHostileCertificationCategoryRow,
    MilestoneThreeHostileCertificationStatus,
};
pub use primitive_family_closure_types::MilestoneThreePrimitiveFamilyClosureRow;
pub use report::{
    MilestoneThreeAmbiguousLocalRewireWitness, MilestoneThreeBowtieAdjacentWitness,
    MilestoneThreeBrokenRadialWitness, MilestoneThreeChangedScopeCoverageRow,
    MilestoneThreeDerivedRegionCoverageRow, MilestoneThreeDeterminismRuleKind,
    MilestoneThreeDeterminismRuleRow, MilestoneThreeEditBranchLocalParityRow,
    MilestoneThreeEditBreadthCounterRow, MilestoneThreeEditFalloutBreadthRow,
    MilestoneThreeEditFalloutClass, MilestoneThreeEditReplayParityReport,
    MilestoneThreeEditReplayParityRow, MilestoneThreeEditReplayStepRow,
    MilestoneThreeFailureLocalityRow, MilestoneThreeHostileCoverageRow,
    MilestoneThreeHostileFamilyCoverageRow, MilestoneThreeHostileNamingDistributionRow,
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileRejectionDistributionRow,
    MilestoneThreeHostileScenario, MilestoneThreeHostileScenarioReport,
    MilestoneThreeHostileSuiteReport, MilestoneThreeNamingContinuityMatrixRow,
    MilestoneThreeRejectedEditScopeReportRow, MilestoneThreeSplitCollapseChurnWitness,
    MilestoneThreeTopologyEditDigestRow, MilestoneThreeValidatorFamily,
    MilestoneThreeValidatorFamilyCoverageRow,
};
pub use side_quest_types::{
    MilestoneThreeReturnGateBlockerRow, MilestoneThreeSideQuestBlockerRow,
    MilestoneThreeSideQuestCloseoutReport, MilestoneThreeSideQuestContractRow,
};
pub(crate) use split_collapse_churn::certify_milestone_three_split_collapse_churn_impl;
pub(crate) use suite::{
    certify_milestone_three_closeout_impl, certify_milestone_three_hostile_suite_impl,
};
pub(crate) use validator_family_coverage::milestone_three_validator_expectations;

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
