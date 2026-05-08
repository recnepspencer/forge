mod accessors;
mod aggregate_acceptance;
mod ambiguous_local_rewire;
mod bowtie_adjacent;
mod broken_radial_localization;
mod cancellation_chain;
mod direct_acceptance;
mod report;
mod shared;
mod side_quest_closeout;
mod split_collapse_churn;
mod suite;

use report::MilestoneThreeHostileScenario as HostileScenario;

pub(crate) use ambiguous_local_rewire::certify_milestone_three_ambiguous_local_rewire_continuity_impl;
pub(crate) use bowtie_adjacent::certify_milestone_three_bowtie_adjacent_rewire_impl;
pub(crate) use broken_radial_localization::certify_milestone_three_broken_radial_localization_impl;
pub(crate) use cancellation_chain::certify_milestone_three_cancellation_chain_parity_impl;
pub use report::{
    MilestoneThreeAmbiguousLocalRewireWitness, MilestoneThreeBowtieAdjacentWitness,
    MilestoneThreeBrokenRadialWitness, MilestoneThreeChangedScopeCoverageRow,
    MilestoneThreeDerivedRegionCoverageRow, MilestoneThreeEditBreadthCounterRow,
    MilestoneThreeEditReplayParityReport, MilestoneThreeEditReplayParityRow,
    MilestoneThreeEditReplayStepRow, MilestoneThreeFailureLocalityRow,
    MilestoneThreeHostileCoverageRow, MilestoneThreeHostileFamilyCoverageRow,
    MilestoneThreeHostileNamingDistributionRow, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileRejectionDistributionRow, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
    MilestoneThreeNamingContinuityMatrixRow, MilestoneThreeRejectedEditScopeReportRow,
    MilestoneThreeReturnGateBlockerRow, MilestoneThreeSideQuestBlockerRow,
    MilestoneThreeSideQuestCloseoutReport, MilestoneThreeSideQuestContractRow,
    MilestoneThreeSplitCollapseChurnWitness, MilestoneThreeTopologyEditDigestRow,
};
pub(crate) use split_collapse_churn::certify_milestone_three_split_collapse_churn_impl;
pub(crate) use suite::{
    certify_milestone_three_closeout_impl, certify_milestone_three_hostile_suite_impl,
};

const MILESTONE_THREE_REQUIRED_SCENARIOS: &[HostileScenario] = &[
    HostileScenario::BowtieAdjacentRewire,
    HostileScenario::CancellationChainParity,
    HostileScenario::SplitCollapseChurn,
    HostileScenario::AmbiguousLocalRewireContinuity,
    HostileScenario::BrokenRadialLocalization,
];

const MILESTONE_THREE_REPLAY_SCENARIOS: &[HostileScenario] = &[
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
