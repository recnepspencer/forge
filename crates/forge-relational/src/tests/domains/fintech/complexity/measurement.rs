use crate::facade::{ComplexityContract, RuntimeComplexityCounters};

use super::super::fixture::FintechWorld;

pub(crate) fn measure_world_action(
    world: &mut FintechWorld,
    action: impl FnOnce(&mut FintechWorld),
) -> RuntimeComplexityCounters {
    world.runtime.performance_access().reset_counters();
    action(world);
    world.runtime.performance_access().counters()
}

pub(crate) fn contract_ids(world: &FintechWorld) -> Vec<&'static str> {
    world
        .runtime
        .performance_access()
        .contracts()
        .iter()
        .map(|contract: &ComplexityContract| contract.id)
        .collect()
}
