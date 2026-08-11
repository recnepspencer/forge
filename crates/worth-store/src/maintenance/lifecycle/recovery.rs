use serde::{Deserialize, Serialize};

use super::super::MaintenanceLaneKey;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveredMaintenanceLaneIntake {
    lane_key: MaintenanceLaneKey,
    pending_recovered_count: u64,
    readmitted_recovered_count: u64,
    rejected_recovered_count: u64,
    stale_recovered_count: u64,
    coalesced_recovered_count: u64,
    debt_bearing: bool,
}

impl RecoveredMaintenanceLaneIntake {
    pub(crate) fn new(
        lane_key: MaintenanceLaneKey,
        pending_recovered_count: u64,
        readmitted_recovered_count: u64,
        rejected_recovered_count: u64,
        stale_recovered_count: u64,
        coalesced_recovered_count: u64,
        debt_bearing: bool,
    ) -> Self {
        Self {
            lane_key,
            pending_recovered_count,
            readmitted_recovered_count,
            rejected_recovered_count,
            stale_recovered_count,
            coalesced_recovered_count,
            debt_bearing,
        }
    }

    pub fn lane_key(&self) -> &MaintenanceLaneKey {
        &self.lane_key
    }

    pub fn pending_recovered_count(&self) -> u64 {
        self.pending_recovered_count
    }

    pub fn readmitted_recovered_count(&self) -> u64 {
        self.readmitted_recovered_count
    }

    pub fn rejected_recovered_count(&self) -> u64 {
        self.rejected_recovered_count
    }

    pub fn stale_recovered_count(&self) -> u64 {
        self.stale_recovered_count
    }

    pub fn coalesced_recovered_count(&self) -> u64 {
        self.coalesced_recovered_count
    }

    pub fn debt_bearing(&self) -> bool {
        self.debt_bearing
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveredMaintenanceIntakeReport {
    pending_recovered_count: u64,
    readmitted_recovered_count: u64,
    rejected_recovered_count: u64,
    stale_recovered_count: u64,
    coalesced_recovered_count: u64,
    lane_intake: Vec<RecoveredMaintenanceLaneIntake>,
}

impl RecoveredMaintenanceIntakeReport {
    pub(crate) fn new(
        pending_recovered_count: u64,
        readmitted_recovered_count: u64,
        rejected_recovered_count: u64,
        stale_recovered_count: u64,
        coalesced_recovered_count: u64,
        lane_intake: Vec<RecoveredMaintenanceLaneIntake>,
    ) -> Self {
        Self {
            pending_recovered_count,
            readmitted_recovered_count,
            rejected_recovered_count,
            stale_recovered_count,
            coalesced_recovered_count,
            lane_intake,
        }
    }

    pub fn pending_recovered_count(&self) -> u64 {
        self.pending_recovered_count
    }

    pub fn readmitted_recovered_count(&self) -> u64 {
        self.readmitted_recovered_count
    }

    pub fn rejected_recovered_count(&self) -> u64 {
        self.rejected_recovered_count
    }

    pub fn stale_recovered_count(&self) -> u64 {
        self.stale_recovered_count
    }

    pub fn coalesced_recovered_count(&self) -> u64 {
        self.coalesced_recovered_count
    }

    pub fn lane_intake(&self) -> &[RecoveredMaintenanceLaneIntake] {
        &self.lane_intake
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceColdStartBootReport {
    loaded_persisted_summaries: bool,
    used_legacy_summary_backfill: bool,
    recovered_backlog_count: u64,
    integrity_reject_count: u64,
}

impl MaintenanceColdStartBootReport {
    pub(crate) fn new(
        loaded_persisted_summaries: bool,
        used_legacy_summary_backfill: bool,
        recovered_backlog_count: u64,
        integrity_reject_count: u64,
    ) -> Self {
        Self {
            loaded_persisted_summaries,
            used_legacy_summary_backfill,
            recovered_backlog_count,
            integrity_reject_count,
        }
    }

    pub fn loaded_persisted_summaries(&self) -> bool {
        self.loaded_persisted_summaries
    }

    pub fn used_legacy_summary_backfill(&self) -> bool {
        self.used_legacy_summary_backfill
    }

    pub fn recovered_backlog_count(&self) -> u64 {
        self.recovered_backlog_count
    }

    pub fn integrity_reject_count(&self) -> u64 {
        self.integrity_reject_count
    }
}
