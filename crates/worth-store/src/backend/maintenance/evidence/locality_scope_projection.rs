use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

pub(super) fn project_locality_scope_counts<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Vec<crate::Milestone11LocalityScopeCount> {
    backend
        .state()
        .maintenance_declaration_records
        .values()
        .fold(
            std::collections::BTreeMap::<crate::MaintenanceLocalityScope, u64>::new(),
            |mut counts, record| {
                *counts
                    .entry(record.work_descriptor.locality_scope().clone())
                    .or_default() += 1;
                counts
            },
        )
        .into_iter()
        .map(|(locality_scope, declaration_count)| {
            let (deferred_count, active_count) = backend
                .state()
                .maintenance_locality_summary_records
                .values()
                .filter(|record| record.summary.locality_scope() == &locality_scope)
                .fold((0, 0), |(deferred, active), record| {
                    (
                        deferred + record.summary.deferred_count(),
                        active + record.summary.active_count(),
                    )
                });
            crate::Milestone11LocalityScopeCount {
                locality_scope,
                declaration_count,
                deferred_count,
                active_count,
            }
        })
        .collect()
}
