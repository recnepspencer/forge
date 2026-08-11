use std::sync::atomic::Ordering;

use super::super::StoreCounterSnapshot;
use super::RetentionCounters;

pub(in crate::evidence) fn write_snapshot(
    counters: &RetentionCounters,
    snapshot: &mut StoreCounterSnapshot,
) {
    macro_rules! load {
        ($field:ident) => {
            snapshot.$field = counters.$field.load(Ordering::Relaxed);
        };
    }
    load!(retention_policy_evaluation_count);
    load!(retained_authoritative_range_count);
    load!(expired_authoritative_range_count);
    load!(compaction_plan_count);
    load!(compacted_delta_layer_count);
    load!(compacted_snapshot_family_count);
    load!(compacted_layout_family_count);
    load!(compaction_cutover_count);
    load!(compaction_cutover_rejection_count);
    load!(reclaim_candidate_count);
    load!(reclaimed_authoritative_artifact_count);
    load!(reclaimed_derived_artifact_count);
    load!(reclaim_rejected_live_basis_count);
    load!(retention_closure_ancestor_count);
    load!(retention_closure_failure_count);
    load!(retained_range_rebuild_count);
    load!(rebuild_debt_count);
    load!(compaction_debt_count);
    load!(retention_truth_parity_failure_count);
    load!(retention_restore_parity_failure_count);
    load!(retention_artifact_rebuild_failure_count);
    load!(maintenance_declaration_count);
    load!(maintenance_admission_count);
    load!(maintenance_rejection_count);
    load!(maintenance_resume_count);
    load!(maintenance_restart_readmission_count);
    load!(maintenance_restart_rejection_count);
    load!(maintenance_checkpoint_count);
    load!(maintenance_completion_count);
    load!(maintenance_failure_count);
    load!(maintenance_debt_link_count);
    load!(maintenance_foreground_borrow_count);
    load!(maintenance_foreground_wait_count);
    load!(maintenance_cutover_dependency_count);
    load!(maintenance_coalesced_work_count);
    load!(maintenance_cancelled_superseded_work_count);
    load!(maintenance_store_global_scope_count);
    load!(maintenance_starvation_trigger_count);
    load!(maintenance_debt_escalation_count);
    load!(maintenance_io_budget_units_reserved);
    load!(maintenance_cpu_budget_units_reserved);
    load!(maintenance_memory_budget_units_reserved);
    load!(maintenance_publication_slot_budget_reserved);
    load!(maintenance_quantum_grant_count);
    load!(maintenance_quantum_exhaustion_count);
    load!(maintenance_background_unit_execute_count);
    load!(maintenance_foreground_interference_count);
    load!(maintenance_foreground_wait_on_cutover_count);
    load!(maintenance_foreground_broadened_count);
    load!(maintenance_reservation_violation_count);
    load!(maintenance_cross_locality_escalation_count);
    load!(maintenance_freshness_rejection_count);
    load!(maintenance_locality_touch_count);
    load!(maintenance_global_scope_fallback_count);
    load!(maintenance_cold_start_boot_count);
    load!(maintenance_cold_start_summary_load_count);
    load!(maintenance_cold_start_legacy_backfill_count);
    load!(maintenance_cold_start_recovery_backlog_count);
    load!(maintenance_cold_start_integrity_reject_count);
    load!(maintenance_cold_start_global_scan_count);
    load!(maintenance_plan_execute_without_descriptor_count);
    load!(maintenance_illegal_escalation_count);
    load!(maintenance_truth_visibility_violation_count);
}
