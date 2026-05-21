use forge_query::facade::ForgeQueryWorkspace;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};

use super::super::parity::compound_parity_registry;
use super::super::PrimitiveConstructionCompoundAdversarialSiegeReport;
use super::super::PrimitiveConstructionCompoundParityCanonicalTruth;

pub(super) fn compound_workspace(name: &str) -> ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        name.to_string(),
    )
    .expect("workspace")
}

pub(super) fn sorted_ids(ids: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort();
    ids
}

pub(super) fn expected_required_scenario_ids() -> Vec<String> {
    sorted_ids(
        compound_parity_registry()
            .required_scenario_inventory()
            .scenario_ids()
            .iter()
            .cloned(),
    )
}

pub(super) fn expected_motion_scenario_ids(
    truth: &PrimitiveConstructionCompoundParityCanonicalTruth,
) -> Vec<String> {
    sorted_ids(
        truth
            .expected_motion()
            .expect("expected motion truth")
            .rows()
            .iter()
            .map(|row| row.scenario_id().to_string()),
    )
}

pub(super) fn expected_grazing_scenario_ids(
    truth: &PrimitiveConstructionCompoundParityCanonicalTruth,
) -> Vec<String> {
    sorted_ids(
        truth
            .expected_grazing()
            .expect("expected grazing truth")
            .rows()
            .iter()
            .map(|row| row.scenario_id().to_string()),
    )
}

pub(super) fn expected_exhaustion_scenario_ids(
    truth: &PrimitiveConstructionCompoundParityCanonicalTruth,
) -> Vec<String> {
    sorted_ids(
        truth
            .expected_exhaustion()
            .expect("expected exhaustion truth")
            .rows()
            .iter()
            .map(|row| row.scenario_id().to_string()),
    )
}

pub(super) fn parity_truth_from_siege(
    siege: &PrimitiveConstructionCompoundAdversarialSiegeReport,
) -> PrimitiveConstructionCompoundParityCanonicalTruth {
    PrimitiveConstructionCompoundParityCanonicalTruth::from_siege(siege)
}
