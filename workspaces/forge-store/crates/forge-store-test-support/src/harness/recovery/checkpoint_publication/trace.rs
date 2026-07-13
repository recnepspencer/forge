use crate::admitted_developer_smoke_driver_contracts;
use forge_store_physical_certification::{
    PhysicalInterleavingSchedule, PhysicalScenarioActorRole, ProductionBoundaryDriverTrace,
};

pub(super) fn developer_smoke_production_trace() -> ProductionBoundaryDriverTrace {
    admitted_developer_smoke_driver_contracts()
        .unwrap()
        .iter()
        .find_map(|driver| driver.production_boundary_trace())
        .unwrap()
}

pub fn actor_step_index(
    schedule: &PhysicalInterleavingSchedule,
    role: PhysicalScenarioActorRole,
) -> usize {
    schedule
        .actor_steps()
        .iter()
        .position(|step| step.actor_role() == role)
        .unwrap()
}
