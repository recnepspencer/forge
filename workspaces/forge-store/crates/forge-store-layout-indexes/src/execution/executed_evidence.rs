use crate::execution::counter_snapshot::S8AccessPathCounterSnapshot;
use crate::execution::ready_plan::S8ExecutionReadyAccessPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S8ExecutedAccessEvidence {
    ready: S8ExecutionReadyAccessPlan,
    observed: S8AccessPathCounterSnapshot,
}

impl S8ExecutedAccessEvidence {
    pub(crate) const fn new(
        ready: S8ExecutionReadyAccessPlan,
        observed: S8AccessPathCounterSnapshot,
    ) -> Self {
        Self { ready, observed }
    }

    pub(crate) const fn observe_as_planned(ready: S8ExecutionReadyAccessPlan) -> Self {
        Self::new(ready, ready.lowered().planned())
    }

    pub(crate) const fn ready(&self) -> S8ExecutionReadyAccessPlan {
        self.ready
    }

    pub const fn observed(&self) -> S8AccessPathCounterSnapshot {
        self.observed
    }
}
