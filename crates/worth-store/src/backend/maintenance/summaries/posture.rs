use crate::backend::records::StoreState;

#[derive(Debug, Clone, Copy)]
pub(super) struct SchedulerSummaryPosture {
    pub(super) has_maintenance_state: bool,
}

pub(super) fn clear_scheduler_summary_records(state: &mut StoreState) -> SchedulerSummaryPosture {
    let posture = SchedulerSummaryPosture {
        has_maintenance_state: !state.maintenance_declaration_records.is_empty()
            || !state.maintenance_execution_records.is_empty(),
    };
    state.maintenance_queue_summary_records.clear();
    state.maintenance_locality_summary_records.clear();
    state.maintenance_reservation_summary_records.clear();
    state.maintenance_resource_budget_summary_records.clear();
    state.maintenance_debt_summary_records.clear();
    posture
}
