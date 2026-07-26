use super::super::{WorthQueryActiveDirectGraphExecution, WorthQueryActiveWorkflowGraphExecution};
use super::WorthQueryReadmissionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryReadmittedAttemptEvidence {
    counters: WorthQueryReadmissionCounters,
}

#[must_use = "a readmitted direct execution must enter the managed-run lifecycle"]
pub struct WorthQueryReadmittedDirectGraphExecution {
    active: WorthQueryActiveDirectGraphExecution,
    evidence: WorthQueryReadmittedAttemptEvidence,
}

#[must_use = "a readmitted workflow execution must enter the managed-run lifecycle"]
pub struct WorthQueryReadmittedWorkflowGraphExecution {
    active: WorthQueryActiveWorkflowGraphExecution,
    evidence: WorthQueryReadmittedAttemptEvidence,
}

impl WorthQueryReadmittedAttemptEvidence {
    const fn committed(counters: WorthQueryReadmissionCounters) -> Self {
        Self { counters }
    }

    pub const fn counters(self) -> WorthQueryReadmissionCounters {
        self.counters
    }
}

impl WorthQueryReadmittedDirectGraphExecution {
    pub(super) fn new(
        active: WorthQueryActiveDirectGraphExecution,
        counters: WorthQueryReadmissionCounters,
    ) -> Self {
        Self {
            active,
            evidence: WorthQueryReadmittedAttemptEvidence::committed(counters),
        }
    }

    pub const fn readmission_evidence(&self) -> WorthQueryReadmittedAttemptEvidence {
        self.evidence
    }

    pub const fn active(&self) -> &WorthQueryActiveDirectGraphExecution {
        &self.active
    }

    pub fn into_active(self) -> WorthQueryActiveDirectGraphExecution {
        self.active
    }
}

impl WorthQueryReadmittedWorkflowGraphExecution {
    pub(super) fn new(
        active: WorthQueryActiveWorkflowGraphExecution,
        counters: WorthQueryReadmissionCounters,
    ) -> Self {
        Self {
            active,
            evidence: WorthQueryReadmittedAttemptEvidence::committed(counters),
        }
    }

    pub const fn readmission_evidence(&self) -> WorthQueryReadmittedAttemptEvidence {
        self.evidence
    }

    pub const fn active(&self) -> &WorthQueryActiveWorkflowGraphExecution {
        &self.active
    }

    pub fn into_active(self) -> WorthQueryActiveWorkflowGraphExecution {
        self.active
    }
}
