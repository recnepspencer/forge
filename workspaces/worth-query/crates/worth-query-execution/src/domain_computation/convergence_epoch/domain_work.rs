use std::sync::Arc;

use super::WorthQueryConvergenceDomainDecision;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryConvergenceDomainWorkEvidence {
    comparator_call_count: usize,
    progress_check_count: usize,
    repeated_state_probe_count: usize,
}

impl WorthQueryConvergenceDomainWorkEvidence {
    pub const fn new(
        comparator_call_count: usize,
        progress_check_count: usize,
        repeated_state_probe_count: usize,
    ) -> Self {
        Self {
            comparator_call_count,
            progress_check_count,
            repeated_state_probe_count,
        }
    }

    pub const fn one_complete_assessment() -> Self {
        Self::new(1, 1, 1)
    }

    pub const fn comparator_failure() -> Self {
        Self::new(1, 0, 0)
    }

    pub const fn comparator_call_count(&self) -> usize {
        self.comparator_call_count
    }

    pub const fn progress_check_count(&self) -> usize {
        self.progress_check_count
    }

    pub const fn repeated_state_probe_count(&self) -> usize {
        self.repeated_state_probe_count
    }
}

pub struct WorthQueryConvergenceDomainAssessmentOutcome {
    decision: WorthQueryConvergenceDomainDecision,
    work: WorthQueryConvergenceDomainWorkEvidence,
}

impl WorthQueryConvergenceDomainAssessmentOutcome {
    pub fn new(
        decision: WorthQueryConvergenceDomainDecision,
        work: WorthQueryConvergenceDomainWorkEvidence,
    ) -> Self {
        Self { decision, work }
    }

    pub fn work(&self) -> &WorthQueryConvergenceDomainWorkEvidence {
        &self.work
    }

    pub fn decision(&self) -> &WorthQueryConvergenceDomainDecision {
        &self.decision
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryConvergenceDomainDecision,
        WorthQueryConvergenceDomainWorkEvidence,
    ) {
        (self.decision, self.work)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConvergenceDomainFailure {
    detail: Arc<str>,
    work: WorthQueryConvergenceDomainWorkEvidence,
}

impl WorthQueryConvergenceDomainFailure {
    pub fn new(detail: impl Into<Arc<str>>) -> Self {
        Self {
            detail: detail.into(),
            work: WorthQueryConvergenceDomainWorkEvidence::new(0, 0, 0),
        }
    }

    pub fn with_work(
        detail: impl Into<Arc<str>>,
        work: WorthQueryConvergenceDomainWorkEvidence,
    ) -> Self {
        Self {
            detail: detail.into(),
            work,
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn work(&self) -> &WorthQueryConvergenceDomainWorkEvidence {
        &self.work
    }
}
