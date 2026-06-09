use forge_query::facade::ForgeQueryWorkspace;
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

use crate::construction::tests::support::compound_runtime::compound_parity_registry;

pub(crate) fn compound_workspace(name: &str) -> ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        name.to_string(),
    )
    .expect("workspace")
}

pub(crate) fn sorted_ids(ids: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort();
    ids
}

pub(crate) fn expected_required_scenario_ids() -> Vec<String> {
    sorted_ids(
        compound_parity_registry()
            .required_scenario_inventory()
            .scenario_ids()
            .iter()
            .cloned(),
    )
}

pub(crate) fn expected_motion_scenario_ids() -> Vec<String> {
    sorted_ids(
        compound_parity_registry()
            .motion_inventory()
            .keys()
            .cloned(),
    )
}

pub(crate) fn expected_grazing_scenario_ids() -> Vec<String> {
    sorted_ids(
        compound_parity_registry()
            .grazing_inventory()
            .keys()
            .cloned(),
    )
}

pub(crate) fn expected_exhaustion_scenario_ids() -> Vec<String> {
    sorted_ids(
        compound_parity_registry()
            .exhaustion_inventory()
            .keys()
            .cloned(),
    )
}
