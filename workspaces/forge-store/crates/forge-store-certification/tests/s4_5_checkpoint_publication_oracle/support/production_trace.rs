use forge_store_physical_certification::{
    PhysicalInterleavingSchedule, PhysicalScenarioActorRole, ProductionBoundaryDriverTrace,
};
use forge_store_test_support::admitted_developer_smoke_driver_contracts;

pub(super) fn developer_smoke_production_trace() -> ProductionBoundaryDriverTrace {
    admitted_developer_smoke_driver_contracts()
        .unwrap()
        .iter()
        .find_map(|driver| driver.production_boundary_trace())
        .unwrap()
}

pub(crate) fn actor_step_index(
    schedule: &PhysicalInterleavingSchedule,
    role: PhysicalScenarioActorRole,
) -> usize {
    schedule
        .actor_steps()
        .iter()
        .position(|step| step.actor_role() == role)
        .unwrap()
}
