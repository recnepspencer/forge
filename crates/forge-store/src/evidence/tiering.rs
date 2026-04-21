#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};

use super::{StoreCounterSnapshot, StoreCounters};

#[derive(Debug, Default)]
pub(super) struct TieringCounters {
    placement_state_manifest_load_count: AtomicU64,
    placement_state_recovery_count: AtomicU64,
    working_set_observation_window_count: AtomicU64,
    working_set_reclassification_count: AtomicU64,
    hot_tier_resident_read_count: AtomicU64,
    warm_tier_resident_read_count: AtomicU64,
    cold_tier_recall_count: AtomicU64,
    foreground_cold_recall_count: AtomicU64,
    background_tier_move_count: AtomicU64,
    restart_recall_count: AtomicU64,
    tier_move_plan_count: AtomicU64,
    tier_move_cutover_count: AtomicU64,
    tier_move_cutover_rejection_count: AtomicU64,
    authoritative_tier_move_count: AtomicU64,
    derived_tier_move_count: AtomicU64,
    tier_move_rejection_count: AtomicU64,
    tier_miss_count: AtomicU64,
    broadened_recall_plan_count: AtomicU64,
    recall_coalesced_request_count: AtomicU64,
    recall_duplicate_suppression_count: AtomicU64,
    tier_interleaved_read_count: AtomicU64,
    tier_interleaved_continuation_count: AtomicU64,
    tier_interleaving_recall_count: AtomicU64,
    tier_interleaving_parity_failure_count: AtomicU64,
    placement_debt_count: AtomicU64,
    working_set_debt_count: AtomicU64,
    tier_truth_parity_failure_count: AtomicU64,
    tier_restore_parity_failure_count: AtomicU64,
    tier_recall_failure_count: AtomicU64,
}

impl StoreCounters {
    pub fn record_placement_state_manifest_loads(&self, count: u64) {
        self.tiering
            .placement_state_manifest_load_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_placement_state_recovery(&self, count: u64) {
        self.tiering
            .placement_state_recovery_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_working_set_observation_windows(&self, count: u64) {
        self.tiering
            .working_set_observation_window_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_working_set_reclassifications(&self, count: u64) {
        self.tiering
            .working_set_reclassification_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_hot_tier_resident_reads(&self, count: u64) {
        self.tiering
            .hot_tier_resident_read_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_warm_tier_resident_reads(&self, count: u64) {
        self.tiering
            .warm_tier_resident_read_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_cold_tier_recalls(&self, count: u64) {
        self.tiering
            .cold_tier_recall_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_foreground_cold_recalls(&self, count: u64) {
        self.tiering
            .foreground_cold_recall_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_background_tier_moves(&self, count: u64) {
        self.tiering
            .background_tier_move_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_restart_recalls(&self, count: u64) {
        self.tiering
            .restart_recall_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_move_plans(&self, count: u64) {
        self.tiering
            .tier_move_plan_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_move_cutovers(&self, count: u64) {
        self.tiering
            .tier_move_cutover_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_move_cutover_rejections(&self, count: u64) {
        self.tiering
            .tier_move_cutover_rejection_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_authoritative_tier_moves(&self, count: u64) {
        self.tiering
            .authoritative_tier_move_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_derived_tier_moves(&self, count: u64) {
        self.tiering
            .derived_tier_move_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_move_rejections(&self, count: u64) {
        self.tiering
            .tier_move_rejection_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_misses(&self, count: u64) {
        self.tiering
            .tier_miss_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_broadened_recall_plans(&self, count: u64) {
        self.tiering
            .broadened_recall_plan_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_recall_coalesced_requests(&self, count: u64) {
        self.tiering
            .recall_coalesced_request_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_recall_duplicate_suppression(&self, count: u64) {
        self.tiering
            .recall_duplicate_suppression_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_interleaved_reads(&self, count: u64) {
        self.tiering
            .tier_interleaved_read_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_interleaved_continuations(&self, count: u64) {
        self.tiering
            .tier_interleaved_continuation_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_interleaving_recalls(&self, count: u64) {
        self.tiering
            .tier_interleaving_recall_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_interleaving_parity_failures(&self, count: u64) {
        self.tiering
            .tier_interleaving_parity_failure_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_placement_debt(&self, count: u64) {
        self.tiering
            .placement_debt_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_working_set_debt(&self, count: u64) {
        self.tiering
            .working_set_debt_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_truth_parity_failures(&self, count: u64) {
        self.tiering
            .tier_truth_parity_failure_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_restore_parity_failures(&self, count: u64) {
        self.tiering
            .tier_restore_parity_failure_count
            .fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_tier_recall_failures(&self, count: u64) {
        self.tiering
            .tier_recall_failure_count
            .fetch_add(count, Ordering::Relaxed);
    }
}

pub(super) fn write_snapshot(counters: &TieringCounters, snapshot: &mut StoreCounterSnapshot) {
    macro_rules! load {
        ($field:ident) => {
            snapshot.$field = counters.$field.load(Ordering::Relaxed);
        };
    }

    load!(placement_state_manifest_load_count);
    load!(placement_state_recovery_count);
    load!(working_set_observation_window_count);
    load!(working_set_reclassification_count);
    load!(hot_tier_resident_read_count);
    load!(warm_tier_resident_read_count);
    load!(cold_tier_recall_count);
    load!(foreground_cold_recall_count);
    load!(background_tier_move_count);
    load!(restart_recall_count);
    load!(tier_move_plan_count);
    load!(tier_move_cutover_count);
    load!(tier_move_cutover_rejection_count);
    load!(authoritative_tier_move_count);
    load!(derived_tier_move_count);
    load!(tier_move_rejection_count);
    load!(tier_miss_count);
    load!(broadened_recall_plan_count);
    load!(recall_coalesced_request_count);
    load!(recall_duplicate_suppression_count);
    load!(tier_interleaved_read_count);
    load!(tier_interleaved_continuation_count);
    load!(tier_interleaving_recall_count);
    load!(tier_interleaving_parity_failure_count);
    load!(placement_debt_count);
    load!(working_set_debt_count);
    load!(tier_truth_parity_failure_count);
    load!(tier_restore_parity_failure_count);
    load!(tier_recall_failure_count);
}
