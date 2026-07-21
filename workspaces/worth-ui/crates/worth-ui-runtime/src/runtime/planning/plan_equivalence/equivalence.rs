use crate::runtime::{
    WorthUiExecutablePlanDecisionKind, WorthUiExecutionPlanDigest,
    WorthUiExecutionPlanEquivalenceCounters,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionPlanEquivalence {
    previous_digest: WorthUiExecutionPlanDigest,
    next_digest: WorthUiExecutionPlanDigest,
    decision_kind: WorthUiExecutablePlanDecisionKind,
    counters: WorthUiExecutionPlanEquivalenceCounters,
}

impl WorthUiExecutionPlanEquivalence {
    pub(crate) fn new(
        previous_digest: WorthUiExecutionPlanDigest,
        next_digest: WorthUiExecutionPlanDigest,
        decision_kind: WorthUiExecutablePlanDecisionKind,
        counters: WorthUiExecutionPlanEquivalenceCounters,
    ) -> Self {
        Self {
            previous_digest,
            next_digest,
            decision_kind,
            counters,
        }
    }

    pub fn previous_digest(self) -> WorthUiExecutionPlanDigest {
        self.previous_digest
    }

    pub fn next_digest(self) -> WorthUiExecutionPlanDigest {
        self.next_digest
    }

    pub fn decision_kind(self) -> WorthUiExecutablePlanDecisionKind {
        self.decision_kind
    }

    pub fn counters(self) -> WorthUiExecutionPlanEquivalenceCounters {
        self.counters
    }
}
