use schema::facade::topology_authoring::{
    milestone_one_heavy_branch_local_sweep_scenarios, MilestoneOnePrimitiveExpectedOutcome,
    MilestoneOnePrimitiveScenario,
};

use crate::test_support::primitive_corpus::authored_topology::milestone_one_default_corpus_scenarios;

pub(crate) fn milestone_one_default_branch_local_admitted_scenarios(
) -> Vec<MilestoneOnePrimitiveScenario> {
    milestone_one_default_corpus_scenarios()
        .into_iter()
        .filter(|scenario| scenario.expected_outcome == MilestoneOnePrimitiveExpectedOutcome::Admit)
        .collect()
}

pub(crate) fn milestone_one_heavy_branch_local_scenarios() -> Vec<MilestoneOnePrimitiveScenario> {
    milestone_one_heavy_branch_local_sweep_scenarios()
}




