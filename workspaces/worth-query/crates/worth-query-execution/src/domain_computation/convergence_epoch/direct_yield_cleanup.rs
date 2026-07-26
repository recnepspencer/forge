use super::core::WorthQueryConvergenceEpochCore;
use super::{
    WorthQueryBoundConvergenceReport, WorthQueryConvergenceEpochCounters,
    WorthQueryRetainedConvergenceCandidateEvidence, WorthQueryYieldedDirectConvergenceIteration,
};
use crate::domain_computation::{
    WorthQueryDirectYieldCleanupOutcome, WorthQueryDirectYieldCleanupReceipt,
};

#[must_use = "convergence yield cleanup carries epoch and managed release evidence"]
pub enum WorthQueryDirectConvergenceYieldCleanupOutcome {
    Complete(WorthQueryDirectConvergenceYieldCleanupReceipt),
    RecoveryRequired(WorthQueryDirectConvergenceYieldCleanupReceipt),
}

pub struct WorthQueryDirectConvergenceYieldCleanupReceipt {
    core: WorthQueryConvergenceEpochCore,
    managed: WorthQueryDirectYieldCleanupReceipt,
}

impl WorthQueryDirectConvergenceYieldCleanupReceipt {
    pub fn identity(&self) -> &str {
        self.core.identity()
    }

    pub fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        self.core.counters()
    }

    pub fn incumbents(&self) -> &[WorthQueryRetainedConvergenceCandidateEvidence] {
        self.core.incumbents()
    }

    pub fn latest_report(&self) -> Option<&WorthQueryBoundConvergenceReport> {
        self.core.latest_report()
    }

    pub fn managed_receipt(&self) -> &WorthQueryDirectYieldCleanupReceipt {
        &self.managed
    }
}

impl WorthQueryYieldedDirectConvergenceIteration {
    pub fn cleanup(self) -> WorthQueryDirectConvergenceYieldCleanupOutcome {
        let Self {
            pending, yielded, ..
        } = self;
        let mut core = pending.core;
        core.counters_mut().cleaned_up();
        match yielded.cleanup() {
            WorthQueryDirectYieldCleanupOutcome::Complete(managed) => {
                WorthQueryDirectConvergenceYieldCleanupOutcome::Complete(
                    WorthQueryDirectConvergenceYieldCleanupReceipt { core, managed },
                )
            }
            WorthQueryDirectYieldCleanupOutcome::RecoveryRequired(managed) => {
                WorthQueryDirectConvergenceYieldCleanupOutcome::RecoveryRequired(
                    WorthQueryDirectConvergenceYieldCleanupReceipt { core, managed },
                )
            }
        }
    }
}
