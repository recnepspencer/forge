use super::ForgeServerOperationSchedulerCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerSchedulerConflictDenialCode {
    NonSharedReadPlan,
    UnsupportedSharedReadOperation,
    UnsupportedOrderedOperation,
    ConflictingMutationPlan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerSchedulerConflictDenial {
    code: ForgeServerSchedulerConflictDenialCode,
    detail: String,
    facts: Option<ForgeServerSchedulerConflictDenialFacts>,
    scheduler_counters: ForgeServerOperationSchedulerCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerSchedulerConflictDenialFacts {
    scheduler_lane: String,
    requested_basis_digest: Option<String>,
    left_slot_ordinal: Option<usize>,
    right_slot_ordinal: Option<usize>,
}

impl ForgeServerSchedulerConflictDenial {
    pub(crate) fn non_shared_read_plan(detail: impl Into<String>) -> Self {
        Self {
            code: ForgeServerSchedulerConflictDenialCode::NonSharedReadPlan,
            detail: detail.into(),
            facts: None,
            scheduler_counters: ForgeServerOperationSchedulerCounters::default(),
        }
    }

    pub(crate) fn unsupported_shared_read_operation(detail: impl Into<String>) -> Self {
        Self {
            code: ForgeServerSchedulerConflictDenialCode::UnsupportedSharedReadOperation,
            detail: detail.into(),
            facts: None,
            scheduler_counters: ForgeServerOperationSchedulerCounters::default(),
        }
    }

    pub(crate) fn unsupported_ordered_operation(detail: impl Into<String>) -> Self {
        Self {
            code: ForgeServerSchedulerConflictDenialCode::UnsupportedOrderedOperation,
            detail: detail.into(),
            facts: None,
            scheduler_counters: ForgeServerOperationSchedulerCounters::default(),
        }
    }

    pub(crate) fn conflicting_mutation_plan(detail: impl Into<String>) -> Self {
        Self {
            code: ForgeServerSchedulerConflictDenialCode::ConflictingMutationPlan,
            detail: detail.into(),
            facts: None,
            scheduler_counters: ForgeServerOperationSchedulerCounters::default(),
        }
    }

    pub(crate) fn with_conflict_facts(
        mut self,
        facts: ForgeServerSchedulerConflictDenialFacts,
    ) -> Self {
        self.facts = Some(facts);
        self
    }

    pub(crate) fn attach_batch_scheduler_counters(mut self, planned_batch_width: usize) -> Self {
        let mut scheduler_counters = ForgeServerOperationSchedulerCounters::default();
        scheduler_counters.set_planned_batch_width(planned_batch_width);
        if self.code == ForgeServerSchedulerConflictDenialCode::ConflictingMutationPlan {
            scheduler_counters.increment_conflicting_mutation_plan_denial_count();
        }
        self.scheduler_counters = scheduler_counters;
        self
    }

    pub fn code(&self) -> ForgeServerSchedulerConflictDenialCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn facts(&self) -> Option<&ForgeServerSchedulerConflictDenialFacts> {
        self.facts.as_ref()
    }

    pub fn scheduler_counters(&self) -> &ForgeServerOperationSchedulerCounters {
        &self.scheduler_counters
    }
}

impl ForgeServerSchedulerConflictDenialFacts {
    pub(crate) fn conflicting_mutation_plan(
        scheduler_lane: impl Into<String>,
        requested_basis_digest: Option<impl Into<String>>,
        left_slot_ordinal: usize,
        right_slot_ordinal: usize,
    ) -> Self {
        Self {
            scheduler_lane: scheduler_lane.into(),
            requested_basis_digest: requested_basis_digest.map(Into::into),
            left_slot_ordinal: Some(left_slot_ordinal),
            right_slot_ordinal: Some(right_slot_ordinal),
        }
    }

    pub fn scheduler_lane(&self) -> &str {
        &self.scheduler_lane
    }

    pub fn requested_basis_digest(&self) -> Option<&str> {
        self.requested_basis_digest.as_deref()
    }

    pub fn left_slot_ordinal(&self) -> Option<usize> {
        self.left_slot_ordinal
    }

    pub fn right_slot_ordinal(&self) -> Option<usize> {
        self.right_slot_ordinal
    }
}
