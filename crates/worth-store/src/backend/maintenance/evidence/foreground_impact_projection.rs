use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

#[derive(Debug, Clone)]
pub(super) struct MaintenanceForegroundImpactProjection {
    pub(super) borrowed_declaration_count: u64,
    pub(super) waited_declaration_count: u64,
    pub(super) cutover_dependency_declaration_count: u64,
}

pub(super) fn project_foreground_impact<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> MaintenanceForegroundImpactProjection {
    let executions = backend.state().maintenance_execution_records.values();
    MaintenanceForegroundImpactProjection {
        borrowed_declaration_count: executions
            .clone()
            .filter(|record| record.foreground_impact.borrowed_foreground_reservation())
            .count() as u64,
        waited_declaration_count: executions
            .clone()
            .filter(|record| record.foreground_impact.foreground_wait_required())
            .count() as u64,
        cutover_dependency_declaration_count: executions
            .filter(|record| record.foreground_impact.cutover_dependency_required())
            .count() as u64,
    }
}
