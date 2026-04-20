use std::sync::atomic::{AtomicU64, Ordering};

use super::{StoreCounterSnapshot, StoreCounters};

#[derive(Debug, Default)]
pub(super) struct RetentionCounters {
    retention_policy_evaluation_count: AtomicU64,
    retained_authoritative_range_count: AtomicU64,
    expired_authoritative_range_count: AtomicU64,
    compaction_plan_count: AtomicU64,
    compacted_delta_layer_count: AtomicU64,
    compacted_snapshot_family_count: AtomicU64,
    compacted_layout_family_count: AtomicU64,
    compaction_cutover_count: AtomicU64,
    compaction_cutover_rejection_count: AtomicU64,
    reclaim_candidate_count: AtomicU64,
    reclaimed_authoritative_artifact_count: AtomicU64,
    reclaimed_derived_artifact_count: AtomicU64,
    reclaim_rejected_live_basis_count: AtomicU64,
    retention_closure_ancestor_count: AtomicU64,
    retention_closure_failure_count: AtomicU64,
    retained_range_rebuild_count: AtomicU64,
    rebuild_debt_count: AtomicU64,
    compaction_debt_count: AtomicU64,
    retention_truth_parity_failure_count: AtomicU64,
    retention_restore_parity_failure_count: AtomicU64,
    retention_artifact_rebuild_failure_count: AtomicU64,
    maintenance_declaration_count: AtomicU64,
    maintenance_admission_count: AtomicU64,
    maintenance_rejection_count: AtomicU64,
    maintenance_resume_count: AtomicU64,
    maintenance_checkpoint_count: AtomicU64,
    maintenance_completion_count: AtomicU64,
    maintenance_failure_count: AtomicU64,
    maintenance_debt_link_count: AtomicU64,
}

impl StoreCounters {
    pub fn record_retention_policy_evaluation(&self) { self.retention.retention_policy_evaluation_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_retained_authoritative_ranges(&self, count: u64) { self.retention.retained_authoritative_range_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_expired_authoritative_ranges(&self, count: u64) { self.retention.expired_authoritative_range_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_compaction_plan(&self) { self.retention.compaction_plan_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_compacted_delta_layers(&self, count: u64) { self.retention.compacted_delta_layer_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_compacted_snapshot_families(&self, count: u64) { self.retention.compacted_snapshot_family_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_compacted_layout_families(&self, count: u64) { self.retention.compacted_layout_family_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_compaction_cutover(&self) { self.retention.compaction_cutover_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_compaction_cutover_rejection(&self) { self.retention.compaction_cutover_rejection_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_reclaim_candidates(&self, count: u64) { self.retention.reclaim_candidate_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_reclaimed_authoritative_artifacts(&self, count: u64) { self.retention.reclaimed_authoritative_artifact_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_reclaimed_derived_artifacts(&self, count: u64) { self.retention.reclaimed_derived_artifact_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_reclaim_rejected_live_basis(&self) { self.retention.reclaim_rejected_live_basis_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_retention_closure(&self, ancestor_count: u64) { self.retention.retention_closure_ancestor_count.fetch_add(ancestor_count, Ordering::Relaxed); }
    pub fn record_retention_closure_failure(&self) { self.retention.retention_closure_failure_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_retained_range_rebuild(&self) { self.retention.retained_range_rebuild_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_rebuild_debt(&self, count: u64) { self.retention.rebuild_debt_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_compaction_debt(&self, count: u64) { self.retention.compaction_debt_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_retention_truth_parity_failure(&self) { self.retention.retention_truth_parity_failure_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_retention_restore_parity_failure(&self) { self.retention.retention_restore_parity_failure_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_retention_artifact_rebuild_failure(&self) { self.retention.retention_artifact_rebuild_failure_count.fetch_add(1, Ordering::Relaxed); }
    pub fn record_maintenance_declarations(&self, count: u64) { self.retention.maintenance_declaration_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_maintenance_admissions(&self, count: u64) { self.retention.maintenance_admission_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_maintenance_rejections(&self, count: u64) { self.retention.maintenance_rejection_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_maintenance_resumes(&self, count: u64) { self.retention.maintenance_resume_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_maintenance_checkpoints(&self, count: u64) { self.retention.maintenance_checkpoint_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_maintenance_completions(&self, count: u64) { self.retention.maintenance_completion_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_maintenance_failures(&self, count: u64) { self.retention.maintenance_failure_count.fetch_add(count, Ordering::Relaxed); }
    pub fn record_maintenance_debt_links(&self, count: u64) { self.retention.maintenance_debt_link_count.fetch_add(count, Ordering::Relaxed); }
}

pub(super) fn write_snapshot(counters: &RetentionCounters, snapshot: &mut StoreCounterSnapshot) {
    macro_rules! load { ($field:ident) => { snapshot.$field = counters.$field.load(Ordering::Relaxed); }; }
    load!(retention_policy_evaluation_count); load!(retained_authoritative_range_count);
    load!(expired_authoritative_range_count); load!(compaction_plan_count);
    load!(compacted_delta_layer_count); load!(compacted_snapshot_family_count);
    load!(compacted_layout_family_count); load!(compaction_cutover_count);
    load!(compaction_cutover_rejection_count); load!(reclaim_candidate_count);
    load!(reclaimed_authoritative_artifact_count); load!(reclaimed_derived_artifact_count);
    load!(reclaim_rejected_live_basis_count); load!(retention_closure_ancestor_count);
    load!(retention_closure_failure_count); load!(retained_range_rebuild_count);
    load!(rebuild_debt_count); load!(compaction_debt_count);
    load!(retention_truth_parity_failure_count); load!(retention_restore_parity_failure_count);
    load!(retention_artifact_rebuild_failure_count); load!(maintenance_declaration_count);
    load!(maintenance_admission_count); load!(maintenance_rejection_count);
    load!(maintenance_resume_count); load!(maintenance_checkpoint_count);
    load!(maintenance_completion_count); load!(maintenance_failure_count);
    load!(maintenance_debt_link_count);
}
