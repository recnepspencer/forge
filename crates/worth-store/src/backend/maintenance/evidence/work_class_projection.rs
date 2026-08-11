use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

pub(super) fn project_work_class_counts<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Vec<crate::Milestone11WorkClassCount> {
    backend
        .state()
        .maintenance_declaration_records
        .values()
        .fold(
            std::collections::BTreeMap::<crate::MaintenanceWorkClass, u64>::new(),
            |mut counts, record| {
                *counts
                    .entry(record.work_descriptor.work_class())
                    .or_default() += 1;
                counts
            },
        )
        .into_iter()
        .map(
            |(work_class, declaration_count)| crate::Milestone11WorkClassCount {
                work_class,
                declaration_count,
            },
        )
        .collect()
}
