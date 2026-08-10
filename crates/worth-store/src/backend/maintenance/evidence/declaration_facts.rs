use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

#[derive(Debug, Clone)]
pub(super) struct MaintenanceDeclarationFacts {
    pub(super) work_descriptor_count: u64,
    pub(super) restart_recovered_descriptor_count: u64,
    pub(super) scheduler_work_class_lane_count: u64,
    pub(super) scheduler_locality_bucket_count: u64,
    pub(super) explicit_foreground_reservation_count: u64,
    pub(super) explicit_background_reservation_count: u64,
}

pub(super) fn observe_declaration_facts<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> MaintenanceDeclarationFacts {
    let declarations = backend.state().maintenance_declaration_records.values();
    let scheduler_work_class_lane_count = declarations
        .clone()
        .map(|record| record.work_descriptor.work_class())
        .collect::<std::collections::BTreeSet<_>>()
        .len() as u64;
    let scheduler_locality_bucket_count = declarations
        .clone()
        .map(|record| record.work_descriptor.locality_scope().clone())
        .collect::<std::collections::BTreeSet<_>>()
        .len() as u64;
    let explicit_foreground_reservation_count = declarations
        .clone()
        .filter(|record| {
            matches!(
                record.work_descriptor.reservation_family(),
                crate::MaintenanceReservationFamily::Foreground(_)
            )
        })
        .count() as u64;
    let explicit_background_reservation_count = declarations
        .clone()
        .filter(|record| {
            matches!(
                record.work_descriptor.reservation_family(),
                crate::MaintenanceReservationFamily::Background(_)
            )
        })
        .count() as u64;
    let restart_recovered_descriptor_count = declarations
        .filter(|record| record.work_descriptor.recovered_from_restart())
        .count() as u64;

    MaintenanceDeclarationFacts {
        work_descriptor_count: backend.state().maintenance_declaration_records.len() as u64,
        restart_recovered_descriptor_count,
        scheduler_work_class_lane_count,
        scheduler_locality_bucket_count,
        explicit_foreground_reservation_count,
        explicit_background_reservation_count,
    }
}
