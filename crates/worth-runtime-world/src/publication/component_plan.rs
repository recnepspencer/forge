use worth_relational::facade::mvcc::PreparedRelationalCommitCandidate;

use crate::publication::{CompositeComponentIntent, ResolvedExpectedProductHead};

mod compatibility;
mod lowering;
#[path = "component_plan/relational.rs"]
mod relational;
#[path = "component_plan/signal.rs"]
mod signal;

pub(crate) use lowering::lower_component_plans;
pub use relational::{RelationalComponentPlan, RelationalComponentPlanPosture};
pub use signal::{SignalComponentPlan, SignalComponentPlanPosture};

/// Lowered component plans retain the exact expected composite basis and
/// never infer a sibling plan from currentness.
#[derive(Debug)]
pub struct LoweredOwnerComponentPlan {
    expected: ResolvedExpectedProductHead,
    intent: CompositeComponentIntent,
    relational: RelationalComponentPlan,
    signal: SignalComponentPlan,
}

impl LoweredOwnerComponentPlan {
    pub(crate) fn new(
        expected: ResolvedExpectedProductHead,
        intent: CompositeComponentIntent,
        relational: RelationalComponentPlan,
        signal: SignalComponentPlan,
    ) -> Self {
        Self {
            expected,
            intent,
            relational,
            signal,
        }
    }

    pub const fn expected(&self) -> &ResolvedExpectedProductHead {
        &self.expected
    }

    pub fn component_intent(&self) -> CompositeComponentIntent {
        self.intent.clone()
    }

    pub const fn relational(&self) -> &RelationalComponentPlan {
        &self.relational
    }

    pub const fn signal(&self) -> &SignalComponentPlan {
        &self.signal
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolvedExpectedProductHead,
        CompositeComponentIntent,
        RelationalComponentPlan,
        SignalComponentPlan,
    ) {
        (self.expected, self.intent, self.relational, self.signal)
    }

    /// Recheck the plan against its own admitted head before any bounded
    /// reservation is acquired: postures against the component intent they
    /// were lowered from, and each leg against the head's component basis.
    /// Component basis equality is owner-issued equality, not a digest or
    /// branch-name comparison. Staleness against the live reference cell is a
    /// separate check.
    pub(crate) fn is_internally_consistent(&self) -> bool {
        compatibility::plan_is_internally_consistent(self)
    }

    pub(crate) fn take_relational_candidate(
        &mut self,
    ) -> Option<PreparedRelationalCommitCandidate> {
        self.relational.take_prepared_candidate()
    }
}
