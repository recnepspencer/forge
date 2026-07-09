use crate::{PageLsn, RecoveryCounterSnapshot};

use super::{
    RecoveryPhysicsCloseoutSuiteLane, RecoveryPhysicsCloseoutSuiteRequirement, RecoveryWorkBound,
    S4RecoveryCrashSeam, SyntheticRecoveryShortcutRejectionReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryPhysicsCloseoutSuiteStatus {
    required_lanes: usize,
    completed_lanes: usize,
}

impl RecoveryPhysicsCloseoutSuiteStatus {
    pub(crate) const fn new(required_lanes: usize, completed_lanes: usize) -> Self {
        Self {
            required_lanes,
            completed_lanes,
        }
    }

    pub const fn required_lanes(self) -> usize {
        self.required_lanes
    }

    pub const fn completed_lanes(self) -> usize {
        self.completed_lanes
    }

    pub const fn is_complete(self) -> bool {
        self.required_lanes == self.completed_lanes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsCloseoutReport {
    recovered_root: String,
    admitted_page_lsn_frontier: Option<PageLsn>,
    source_decision_digest: String,
    counters: RecoveryCounterSnapshot,
    work_bound: RecoveryWorkBound,
    suite_status: RecoveryPhysicsCloseoutSuiteStatus,
    required_suites: Vec<RecoveryPhysicsCloseoutSuiteRequirement>,
    crash_seams: Vec<S4RecoveryCrashSeam>,
    synthetic_shortcuts: SyntheticRecoveryShortcutRejectionReport,
    foundational_exact_counter_assertions: usize,
}

impl RecoveryPhysicsCloseoutReport {
    pub(crate) fn new(
        evidence: &super::RecoveryPhysicsCloseoutEvidence,
        work_bound: RecoveryWorkBound,
        required_suites: Vec<RecoveryPhysicsCloseoutSuiteRequirement>,
        crash_seams: Vec<S4RecoveryCrashSeam>,
    ) -> Self {
        let state = evidence.receipt().execution().recovered_state();
        Self {
            recovered_root: state.recovered_physical_root().to_string(),
            admitted_page_lsn_frontier: state.page_lsn_frontier(),
            source_decision_digest: state.source_decision_digest().to_string(),
            counters: evidence.receipt().counters(),
            work_bound,
            suite_status: RecoveryPhysicsCloseoutSuiteStatus::new(
                required_suites.len(),
                required_suites
                    .iter()
                    .filter(|requirement| requirement.is_complete())
                    .count(),
            ),
            required_suites,
            crash_seams,
            synthetic_shortcuts: evidence.shortcut_rejections().clone(),
            foundational_exact_counter_assertions: evidence
                .foundational_evidence()
                .performance()
                .exact_counter_assertions(),
        }
    }

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn admitted_page_lsn_frontier(&self) -> Option<PageLsn> {
        self.admitted_page_lsn_frontier
    }

    pub fn source_decision_digest(&self) -> &str {
        &self.source_decision_digest
    }

    pub const fn counters(&self) -> RecoveryCounterSnapshot {
        self.counters
    }

    pub const fn work_bound(&self) -> RecoveryWorkBound {
        self.work_bound
    }

    pub const fn suite_status(&self) -> RecoveryPhysicsCloseoutSuiteStatus {
        self.suite_status
    }

    pub fn required_suites(&self) -> &[RecoveryPhysicsCloseoutSuiteRequirement] {
        &self.required_suites
    }

    pub fn crash_seams(&self) -> &[S4RecoveryCrashSeam] {
        &self.crash_seams
    }

    pub const fn synthetic_shortcut_rejections(&self) -> &SyntheticRecoveryShortcutRejectionReport {
        &self.synthetic_shortcuts
    }

    pub const fn foundational_exact_counter_assertions(&self) -> usize {
        self.foundational_exact_counter_assertions
    }

    pub fn covers_suite(&self, lane: RecoveryPhysicsCloseoutSuiteLane) -> bool {
        self.required_suites
            .iter()
            .any(|requirement| requirement.lane() == lane && requirement.is_complete())
    }
}
