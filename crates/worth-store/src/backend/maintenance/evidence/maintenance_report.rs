use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

use super::cold_start_projection::project_cold_start_boot;
use super::debt_projection::project_maintenance_debt;
use super::execution_status_projection::project_execution_status;
use super::foreground_impact_projection::project_foreground_impact;
use super::locality_scope_projection::project_locality_scope_counts;
use super::recovered_intake::{recovered_declaration_count, recovered_maintenance_intake_report};
use super::reservation_family_projection::project_reservation_family_counts;
use super::topology_projection::project_scheduler_topology;
use super::work_class_projection::project_work_class_counts;

pub(crate) fn milestone_11_maintenance_report<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone11MaintenanceReport {
    let work_class_counts = project_work_class_counts(backend);
    let reservation_family_counts = project_reservation_family_counts(backend);
    let locality_scope_counts = project_locality_scope_counts(backend);
    let execution_status = project_execution_status(backend);
    let foreground_impact = project_foreground_impact(backend);
    let debt = project_maintenance_debt(backend);
    let recovered_intake = recovered_maintenance_intake_report(backend.state());
    let scheduler_topology = project_scheduler_topology(
        work_class_counts.len() as u64,
        locality_scope_counts.len() as u64,
    );
    let cold_start_boot = project_cold_start_boot(backend.state());
    let counters = backend.counters().snapshot();

    crate::Milestone11MaintenanceReport {
        declared_batch_count: backend.state().maintenance_batch_records.len() as u64,
        persisted_declaration_count: backend.state().maintenance_declaration_records.len() as u64,
        active_declaration_count: execution_status.active_declaration_count,
        reserved_declaration_count: execution_status.reserved_declaration_count,
        deferred_declaration_count: execution_status.deferred_declaration_count,
        escalated_declaration_count: execution_status.escalated_declaration_count,
        cancelled_declaration_count: execution_status.cancelled_declaration_count,
        readmitted_recovered_declaration_count: execution_status
            .readmitted_recovered_declaration_count,
        rejected_recovered_declaration_count: execution_status.rejected_recovered_declaration_count,
        completed_declaration_count: execution_status.completed_declaration_count,
        failed_declaration_count: execution_status.failed_declaration_count,
        checkpoint_count: backend.state().maintenance_checkpoint_records.len() as u64,
        recovered_declaration_count: recovered_declaration_count(backend.state()),
        foreground_borrowed_declaration_count: foreground_impact.borrowed_declaration_count,
        foreground_waited_declaration_count: foreground_impact.waited_declaration_count,
        cutover_dependency_declaration_count: foreground_impact
            .cutover_dependency_declaration_count,
        coalesced_work_count: debt.coalesced_work_count,
        cancelled_superseded_work_count: debt.cancelled_superseded_work_count,
        store_global_scope_declaration_count: debt.store_global_scope_declaration_count,
        starved_lane_count: debt.starved_lane_count,
        debt_bearing_lane_count: debt.debt_bearing_lane_count,
        foreground_interference_count: counters.maintenance_foreground_interference_count,
        foreground_broadened_count: counters.maintenance_foreground_broadened_count,
        reservation_violation_count: counters.maintenance_reservation_violation_count,
        recovered_intake,
        cold_start_boot,
        scheduler_topology,
        work_class_counts,
        reservation_family_counts,
        locality_scope_counts,
    }
}
