use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

#[derive(Debug, Clone)]
pub(super) struct MaintenanceDebtProjection {
    pub(super) coalesced_work_count: u64,
    pub(super) cancelled_superseded_work_count: u64,
    pub(super) store_global_scope_declaration_count: u64,
    pub(super) starved_lane_count: u64,
    pub(super) debt_bearing_lane_count: u64,
}

pub(super) fn project_maintenance_debt<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> MaintenanceDebtProjection {
    MaintenanceDebtProjection {
        coalesced_work_count: backend
            .state()
            .maintenance_queue_summary_records
            .values()
            .map(|record| record.summary.coalesced_count())
            .sum(),
        cancelled_superseded_work_count: backend
            .state()
            .maintenance_queue_summary_records
            .values()
            .map(|record| record.summary.cancelled_superseded_count())
            .sum(),
        store_global_scope_declaration_count: backend
            .state()
            .maintenance_execution_records
            .values()
            .filter(|record| record.explicit_global_scope_debt)
            .count() as u64,
        starved_lane_count: backend
            .state()
            .maintenance_debt_summary_records
            .values()
            .filter(|record| {
                matches!(
                    record.summary.starvation_status(),
                    crate::MaintenanceStarvationStatus::DeferredLanePressure
                )
            })
            .count() as u64,
        debt_bearing_lane_count: backend
            .state()
            .maintenance_debt_summary_records
            .values()
            .filter(|record| {
                !matches!(
                    record.summary.pressure_class(),
                    crate::MaintenanceDebtPressureClass::None
                )
            })
            .count() as u64,
    }
}
