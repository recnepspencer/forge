use crate::facade::{ComplexityContract, RuntimeComplexityCounters};

use super::super::fixture::FintechWorld;

pub(crate) fn measure_world_action(
    world: &mut FintechWorld,
    action: impl FnOnce(&mut FintechWorld),
) -> RuntimeComplexityCounters {
    world.runtime.reset_complexity_counters();
    action(world);
    world.runtime.complexity_counters()
}

pub(crate) fn contract_ids(world: &FintechWorld) -> Vec<&'static str> {
    world.runtime
        .complexity_contracts()
        .iter()
        .map(|contract: &ComplexityContract| contract.id)
        .collect()
}
