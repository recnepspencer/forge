use schema::facade::topology_authoring::{
    milestone_one_admitted_range_sweep_out_of_class_scenarios,
    milestone_one_admitted_range_sweep_scenarios, milestone_one_default_primitive_corpus,
    MilestoneOnePrimitiveScenario,
};

pub(crate) fn milestone_one_default_corpus_scenarios() -> Vec<MilestoneOnePrimitiveScenario> {
    milestone_one_default_primitive_corpus()
}

pub(crate) fn milestone_one_admitted_range_scenarios() -> Vec<MilestoneOnePrimitiveScenario> {
    milestone_one_admitted_range_sweep_scenarios()
}

pub(crate) fn milestone_one_out_of_class_range_scenarios() -> Vec<MilestoneOnePrimitiveScenario> {
    milestone_one_admitted_range_sweep_out_of_class_scenarios()
}




