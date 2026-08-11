use crate::backend::records::StoreState;

use super::aggregation::refresh_scheduler_summaries;

pub(crate) fn record_scheduler_boot_state(state: &mut StoreState) {
    let has_maintenance_state = !state.maintenance_declaration_records.is_empty()
        || !state.maintenance_execution_records.is_empty();
    let summaries_present = !state.maintenance_queue_summary_records.is_empty()
        || !state.maintenance_locality_summary_records.is_empty()
        || !state.maintenance_reservation_summary_records.is_empty()
        || !state.maintenance_resource_budget_summary_records.is_empty()
        || !state.maintenance_debt_summary_records.is_empty();
    state.maintenance_loaded_persisted_summaries_on_boot =
        has_maintenance_state && summaries_present;
    state.maintenance_used_legacy_summary_backfill_on_boot = false;
    state.maintenance_recovered_backlog_on_boot = state
        .maintenance_declaration_records
        .values()
        .filter(|record| record.work_descriptor.recovered_from_restart())
        .count() as u64;
}

pub(crate) fn backfill_scheduler_summaries_if_missing(state: &mut StoreState) {
    let has_maintenance_state = !state.maintenance_declaration_records.is_empty()
        || !state.maintenance_execution_records.is_empty();
    let summaries_missing = state.maintenance_queue_summary_records.is_empty()
        && state.maintenance_locality_summary_records.is_empty()
        && state.maintenance_reservation_summary_records.is_empty()
        && state.maintenance_resource_budget_summary_records.is_empty()
        && state.maintenance_debt_summary_records.is_empty();

    if has_maintenance_state && summaries_missing {
        refresh_scheduler_summaries(state);
        state.maintenance_used_legacy_summary_backfill_on_boot = true;
    }
}
