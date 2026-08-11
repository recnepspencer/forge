use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

#[derive(Debug, Clone)]
pub(super) struct MaintenanceSchedulerDebtFacts {
    pub(super) queue_depth: u64,
    pub(super) queue_locality_scope_count: u64,
    pub(super) compaction_debt_units: u64,
    pub(super) rebuild_debt_units: u64,
    pub(super) snapshot_debt_units: u64,
    pub(super) replication_preparation_debt_units: u64,
    pub(super) tiering_debt_units: u64,
}

pub(super) fn observe_scheduler_debt_facts<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> MaintenanceSchedulerDebtFacts {
    let queue_depth = backend
        .state()
        .maintenance_queue_summary_records
        .values()
        .map(|record| record.summary.admitted_count() + record.summary.deferred_count())
        .sum();
    let queue_locality_scope_count = backend
        .state()
        .maintenance_queue_summary_records
        .values()
        .map(|record| record.summary.lane_key().locality_scope().clone())
        .collect::<std::collections::BTreeSet<_>>()
        .len() as u64;

    MaintenanceSchedulerDebtFacts {
        queue_depth,
        queue_locality_scope_count,
        compaction_debt_units: debt_units_by_family(
            backend,
            crate::MaintenanceDebtFamily::CompactionDebt,
        ),
        rebuild_debt_units: debt_units_by_family(
            backend,
            crate::MaintenanceDebtFamily::RebuildDebt,
        ),
        snapshot_debt_units: debt_units_by_family(
            backend,
            crate::MaintenanceDebtFamily::SnapshotDebt,
        ),
        replication_preparation_debt_units: debt_units_by_family(
            backend,
            crate::MaintenanceDebtFamily::ReplicationPreparationDebt,
        ),
        tiering_debt_units: debt_units_by_family(
            backend,
            crate::MaintenanceDebtFamily::TierPlacementDebt,
        ),
    }
}

fn debt_units_by_family<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    family: crate::MaintenanceDebtFamily,
) -> u64 {
    backend
        .state()
        .maintenance_declaration_records
        .values()
        .filter(|record| record.work_descriptor.debt_family() == Some(family))
        .count() as u64
}
