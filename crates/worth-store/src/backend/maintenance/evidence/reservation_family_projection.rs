use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

pub(super) fn project_reservation_family_counts<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Vec<crate::Milestone11ReservationFamilyCount> {
    backend
        .state()
        .maintenance_declaration_records
        .values()
        .fold(
            std::collections::BTreeMap::<crate::MaintenanceReservationFamily, u64>::new(),
            |mut counts, record| {
                *counts
                    .entry(record.work_descriptor.reservation_family())
                    .or_default() += 1;
                counts
            },
        )
        .into_iter()
        .map(|(reservation_family, declaration_count)| {
            let (reserved_count, deferred_count) = backend
                .state()
                .maintenance_reservation_summary_records
                .values()
                .filter(|record| record.summary.reservation_family() == reservation_family)
                .fold((0, 0), |(reserved, deferred), record| {
                    (
                        reserved + record.summary.reserved_count(),
                        deferred + record.summary.deferred_count(),
                    )
                });
            crate::Milestone11ReservationFamilyCount {
                reservation_family,
                declaration_count,
                reserved_count,
                deferred_count,
            }
        })
        .collect()
}
