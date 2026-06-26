use crate::runtime::{
    WorthUiExecutionPlanDigest, WorthUiExecutionPlanEquivalenceCounters,
    WorthUiPlanReuseClassification,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiExecutionPlanEquivalence {
    previous_digest: WorthUiExecutionPlanDigest,
    next_digest: WorthUiExecutionPlanDigest,
    reuse_classification: WorthUiPlanReuseClassification,
    counters: WorthUiExecutionPlanEquivalenceCounters,
}

impl WorthUiExecutionPlanEquivalence {
    pub(crate) fn new(
        previous_digest: WorthUiExecutionPlanDigest,
        next_digest: WorthUiExecutionPlanDigest,
        reuse_classification: WorthUiPlanReuseClassification,
        counters: WorthUiExecutionPlanEquivalenceCounters,
    ) -> Self {
        Self {
            previous_digest,
            next_digest,
            reuse_classification,
            counters,
        }
    }

    pub fn previous_digest(self) -> WorthUiExecutionPlanDigest {
        self.previous_digest
    }

    pub fn next_digest(self) -> WorthUiExecutionPlanDigest {
        self.next_digest
    }

    pub fn reuse_classification(self) -> WorthUiPlanReuseClassification {
        self.reuse_classification
    }

    pub fn is_reusable(self) -> bool {
        self.reuse_classification == WorthUiPlanReuseClassification::Reusable
    }

    pub fn counters(self) -> WorthUiExecutionPlanEquivalenceCounters {
        self.counters
    }
}
