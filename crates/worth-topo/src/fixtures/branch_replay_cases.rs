use worth_schema::facade::{
    milestone_one_heavy_branch_local_sweep_scenarios, WorthMilestoneOnePrimitiveExpectedOutcome,
    WorthMilestoneOnePrimitiveScenario,
};

use crate::fixtures::authored_topology::milestone_one_default_corpus_scenarios;

pub(crate) fn milestone_one_default_branch_local_admitted_scenarios(
) -> Vec<WorthMilestoneOnePrimitiveScenario> {
    milestone_one_default_corpus_scenarios()
        .into_iter()
        .filter(|scenario| {
            scenario.expected_outcome == WorthMilestoneOnePrimitiveExpectedOutcome::Admit
        })
        .collect()
}

pub(crate) fn milestone_one_heavy_branch_local_scenarios() -> Vec<WorthMilestoneOnePrimitiveScenario>
{
    milestone_one_heavy_branch_local_sweep_scenarios()
}
