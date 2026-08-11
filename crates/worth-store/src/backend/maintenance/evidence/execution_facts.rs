use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

#[derive(Debug, Clone)]
pub(super) struct MaintenanceExecutionFacts {
    pub(super) admitted_plan_count: u64,
    pub(super) deferred_plan_count: u64,
    pub(super) escalated_plan_count: u64,
    pub(super) rejected_plan_count: u64,
    pub(super) tier_work_execute_count: u64,
}

pub(super) fn observe_execution_facts<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> MaintenanceExecutionFacts {
    let executions = backend.state().maintenance_execution_records.values();
    let admitted_plan_count = executions
        .clone()
        .filter(|record| {
            matches!(
                record.plan_family,
                Some(crate::MaintenancePlanFamily::BackgroundPaced)
                    | Some(crate::MaintenancePlanFamily::ForegroundReserved)
            )
        })
        .count() as u64;
    let deferred_plan_count = executions
        .clone()
        .filter(|record| {
            matches!(
                record.plan_family,
                Some(crate::MaintenancePlanFamily::Deferred)
            )
        })
        .count() as u64;
    let escalated_plan_count = executions
        .clone()
        .filter(|record| {
            matches!(
                record.plan_family,
                Some(crate::MaintenancePlanFamily::Escalated)
            )
        })
        .count() as u64;
    let rejected_plan_count = executions
        .clone()
        .filter(|record| {
            matches!(
                record.plan_family,
                Some(crate::MaintenancePlanFamily::Cancelled)
            )
        })
        .count() as u64;
    let tier_work_execute_count = executions
        .filter(|record| {
            matches!(
                record.execution_status,
                crate::MaintenanceExecutionStatus::Completed
            ) && record.lane_key.as_ref().is_some_and(|lane_key| {
                matches!(
                    lane_key.work_class(),
                    crate::MaintenanceWorkClass::TierPlacementProposal
                        | crate::MaintenanceWorkClass::TierMoveExecution
                )
            })
        })
        .count() as u64;

    MaintenanceExecutionFacts {
        admitted_plan_count,
        deferred_plan_count,
        escalated_plan_count,
        rejected_plan_count,
        tier_work_execute_count,
    }
}
