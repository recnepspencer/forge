use crate::backend::records::StoreState;

pub(super) fn project_cold_start_boot(state: &StoreState) -> crate::MaintenanceColdStartBootReport {
    crate::MaintenanceColdStartBootReport::new(
        state.maintenance_loaded_persisted_summaries_on_boot,
        state.maintenance_used_legacy_summary_backfill_on_boot,
        state.maintenance_recovered_backlog_on_boot,
        state.maintenance_boot_integrity_reject_count,
    )
}
