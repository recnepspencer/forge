use super::core::WorthQueryConvergenceEpochCore;
use super::{
    WorthQueryBoundConvergenceReport, WorthQueryConvergenceEpochCounters,
    WorthQueryRetainedConvergenceCandidateEvidence, WorthQueryYieldedWorkflowConvergenceIteration,
};
use crate::domain_computation::{
    WorthQueryWorkflowYieldCleanupOutcome, WorthQueryWorkflowYieldCleanupPending,
    WorthQueryWorkflowYieldCleanupReceipt,
};

#[must_use = "convergence yield cleanup carries complete, pending, or recovery authority"]
pub enum WorthQueryWorkflowConvergenceYieldCleanupOutcome {
    Complete(WorthQueryWorkflowConvergenceYieldCleanupReceipt),
    Pending(WorthQueryWorkflowConvergenceYieldCleanupPending),
    RecoveryRequired(WorthQueryWorkflowConvergenceYieldCleanupReceipt),
}

#[must_use = "pending convergence yield cleanup must be retried after artifact owners close"]
pub struct WorthQueryWorkflowConvergenceYieldCleanupPending {
    core: WorthQueryConvergenceEpochCore,
    managed: WorthQueryWorkflowYieldCleanupPending,
}

pub struct WorthQueryWorkflowConvergenceYieldCleanupReceipt {
    core: WorthQueryConvergenceEpochCore,
    managed: WorthQueryWorkflowYieldCleanupReceipt,
}

impl WorthQueryWorkflowConvergenceYieldCleanupPending {
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

    pub fn managed_pending(&self) -> &WorthQueryWorkflowYieldCleanupPending {
        &self.managed
    }

    pub fn retry(self) -> WorthQueryWorkflowConvergenceYieldCleanupOutcome {
        let Self { mut core, managed } = self;
        core.counters_mut().cleaned_up();
        admit_cleanup_outcome(core, managed.retry())
    }
}

impl WorthQueryWorkflowConvergenceYieldCleanupReceipt {
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

    pub fn managed_receipt(&self) -> &WorthQueryWorkflowYieldCleanupReceipt {
        &self.managed
    }
}

impl WorthQueryYieldedWorkflowConvergenceIteration {
    pub fn cleanup(self) -> WorthQueryWorkflowConvergenceYieldCleanupOutcome {
        let Self {
            pending, yielded, ..
        } = self;
        let mut core = pending.core;
        core.counters_mut().cleaned_up();
        admit_cleanup_outcome(core, yielded.cleanup())
    }
}

fn admit_cleanup_outcome(
    core: WorthQueryConvergenceEpochCore,
    outcome: WorthQueryWorkflowYieldCleanupOutcome,
) -> WorthQueryWorkflowConvergenceYieldCleanupOutcome {
    match outcome {
        WorthQueryWorkflowYieldCleanupOutcome::Complete(managed) => {
            WorthQueryWorkflowConvergenceYieldCleanupOutcome::Complete(
                WorthQueryWorkflowConvergenceYieldCleanupReceipt { core, managed },
            )
        }
        WorthQueryWorkflowYieldCleanupOutcome::Pending(managed) => {
            WorthQueryWorkflowConvergenceYieldCleanupOutcome::Pending(
                WorthQueryWorkflowConvergenceYieldCleanupPending { core, managed },
            )
        }
        WorthQueryWorkflowYieldCleanupOutcome::RecoveryRequired(managed) => {
            WorthQueryWorkflowConvergenceYieldCleanupOutcome::RecoveryRequired(
                WorthQueryWorkflowConvergenceYieldCleanupReceipt { core, managed },
            )
        }
    }
}
