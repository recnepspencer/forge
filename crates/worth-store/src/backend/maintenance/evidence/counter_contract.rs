use crate::backend::engine::{StateBackedStoreBackend, StatePersistence};

use super::counter_snapshot_facts::MaintenanceCounterSnapshotFacts;
use super::declaration_facts::observe_declaration_facts;
use super::execution_facts::observe_execution_facts;
use super::scheduler_debt_facts::observe_scheduler_debt_facts;

pub(crate) fn milestone_11_counter_contract<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone11CounterContract {
    let declaration_facts = observe_declaration_facts(backend);
    let execution_facts = observe_execution_facts(backend);
    let scheduler_debt_facts = observe_scheduler_debt_facts(backend);
    let counter_snapshot_facts = MaintenanceCounterSnapshotFacts::observe(backend);
    let counters = counter_snapshot_facts.snapshot();

    crate::Milestone11CounterContract {
        maintenance_work_descriptor_count: declaration_facts.work_descriptor_count,
        maintenance_declaration_count: counters.maintenance_declaration_count,
        maintenance_admission_count: counters.maintenance_admission_count,
        maintenance_rejection_count: counters.maintenance_rejection_count,
        maintenance_admitted_plan_count: execution_facts.admitted_plan_count,
        maintenance_deferred_plan_count: execution_facts.deferred_plan_count,
        maintenance_escalated_plan_count: execution_facts.escalated_plan_count,
        maintenance_rejected_plan_count: execution_facts.rejected_plan_count,
        maintenance_resume_count: counters.maintenance_resume_count,
        maintenance_restart_readmission_count: counters.maintenance_restart_readmission_count,
        maintenance_restart_rejection_count: counters.maintenance_restart_rejection_count,
        maintenance_restart_recovered_count: declaration_facts.restart_recovered_descriptor_count,
        maintenance_checkpoint_count: counters.maintenance_checkpoint_count,
        maintenance_completion_count: counters.maintenance_completion_count,
        maintenance_failure_count: counters.maintenance_failure_count,
        maintenance_debt_link_count: counters.maintenance_debt_link_count,
        maintenance_compaction_debt_units: scheduler_debt_facts.compaction_debt_units,
        maintenance_rebuild_debt_units: scheduler_debt_facts.rebuild_debt_units,
        maintenance_snapshot_debt_units: scheduler_debt_facts.snapshot_debt_units,
        maintenance_replication_prep_debt_units: scheduler_debt_facts
            .replication_preparation_debt_units,
        maintenance_tiering_debt_units: scheduler_debt_facts.tiering_debt_units,
        maintenance_foreground_borrow_count: counters.maintenance_foreground_borrow_count,
        maintenance_foreground_wait_count: counters.maintenance_foreground_wait_count,
        maintenance_cutover_dependency_count: counters.maintenance_cutover_dependency_count,
        maintenance_coalesced_work_count: counters.maintenance_coalesced_work_count,
        maintenance_cancelled_superseded_work_count: counters
            .maintenance_cancelled_superseded_work_count,
        maintenance_store_global_scope_count: counters.maintenance_store_global_scope_count,
        maintenance_starvation_trigger_count: counters.maintenance_starvation_trigger_count,
        maintenance_debt_escalation_count: counters.maintenance_debt_escalation_count,
        maintenance_io_budget_units_reserved: counters.maintenance_io_budget_units_reserved,
        maintenance_cpu_budget_units_reserved: counters.maintenance_cpu_budget_units_reserved,
        maintenance_memory_budget_units_reserved: counters.maintenance_memory_budget_units_reserved,
        maintenance_publication_slot_budget_reserved: counters
            .maintenance_publication_slot_budget_reserved,
        maintenance_queue_depth: scheduler_debt_facts.queue_depth,
        maintenance_queue_locality_scope_count: scheduler_debt_facts.queue_locality_scope_count,
        maintenance_quantum_grant_count: counters.maintenance_quantum_grant_count,
        maintenance_quantum_exhaustion_count: counters.maintenance_quantum_exhaustion_count,
        maintenance_background_unit_execute_count: counters
            .maintenance_background_unit_execute_count,
        maintenance_tier_work_execute_count: execution_facts.tier_work_execute_count,
        maintenance_foreground_interference_count: counters
            .maintenance_foreground_interference_count,
        maintenance_foreground_wait_on_cutover_count: counters
            .maintenance_foreground_wait_on_cutover_count,
        maintenance_foreground_broadened_count: counters.maintenance_foreground_broadened_count,
        maintenance_reservation_violation_count: counters.maintenance_reservation_violation_count,
        maintenance_cross_locality_escalation_count: counters
            .maintenance_cross_locality_escalation_count,
        maintenance_freshness_rejection_count: counters.maintenance_freshness_rejection_count,
        maintenance_locality_touch_count: counters.maintenance_locality_touch_count,
        maintenance_global_scope_fallback_count: counters.maintenance_global_scope_fallback_count,
        maintenance_cold_start_boot_count: counters.maintenance_cold_start_boot_count,
        maintenance_cold_start_summary_load_count: counters
            .maintenance_cold_start_summary_load_count,
        maintenance_cold_start_legacy_backfill_count: counters
            .maintenance_cold_start_legacy_backfill_count,
        maintenance_cold_start_recovery_backlog_count: counters
            .maintenance_cold_start_recovery_backlog_count,
        maintenance_cold_start_integrity_reject_count: counters
            .maintenance_cold_start_integrity_reject_count,
        maintenance_cold_start_global_scan_count: counters.maintenance_cold_start_global_scan_count,
        maintenance_plan_execute_without_descriptor_count: counters
            .maintenance_plan_execute_without_descriptor_count,
        maintenance_illegal_escalation_count: counters.maintenance_illegal_escalation_count,
        maintenance_truth_visibility_violation_count: counters
            .maintenance_truth_visibility_violation_count,
        scheduler_work_class_lane_count: declaration_facts.scheduler_work_class_lane_count,
        scheduler_locality_bucket_count: declaration_facts.scheduler_locality_bucket_count,
        explicit_foreground_reservation_count: declaration_facts
            .explicit_foreground_reservation_count,
        explicit_background_reservation_count: declaration_facts
            .explicit_background_reservation_count,
        restart_recovered_descriptor_count: declaration_facts.restart_recovered_descriptor_count,
    }
}
