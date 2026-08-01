#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentExecutionTransitionPosture {
    Started,
    PendingBeforeEffect,
    PendingEffectMayHaveBegun,
    Completed {
        outcome: crate::capability::UiIntentSchema,
    },
    RejectedBeforeEffect {
        detail: super::UiIntentProviderStop,
    },
    FailedBeforeEffect {
        detail: super::UiIntentProviderStop,
    },
    CancelledBeforeEffect {
        detail: super::UiIntentProviderStop,
    },
    TimedOutBeforeEffect {
        detail: super::UiIntentProviderStop,
    },
    Partial {
        outcome: Option<crate::capability::UiIntentSchema>,
        detail: super::UiIntentProviderStop,
    },
    Indeterminate {
        detail: Option<super::UiIntentProviderStop>,
    },
}

#[must_use]
pub struct UiIntentExecutionTransition {
    attempt: super::UiIntentExecutionAttemptIdentity,
    idempotency: super::UiIntentExecutionIdempotencyIdentity,
    posture: UiIntentExecutionTransitionPosture,
    recovery: Option<super::UiIntentRecoveryHandle>,
    consequence: Option<super::UiIntentConsequenceHandle>,
    posture_basis: Option<UiIntentExecutionPostureBasis>,
}

#[derive(Clone, Copy)]
pub(crate) struct UiIntentExecutionPostureBasis {
    pub(crate) graph_node: crate::graph::UiGraphNodeIdentity,
    pub(crate) target: crate::runtime::interaction::UiPresentedInteractionTargetView,
}

#[must_use]
pub struct UiIntentExecutionAdvanceReport {
    transitions: Box<[UiIntentExecutionTransition]>,
    active_slots_visited: usize,
    provider_calls: usize,
    provider_polls: usize,
    cancellation_calls: usize,
    settlements: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentExecutionAdvanceStop {
    MonotonicTimeRegressed { previous: u64, observed: u64 },
}

#[must_use]
pub enum UiIntentExecutionAdvanceOutcome {
    Advanced(UiIntentExecutionAdvanceReport),
    Stopped(UiIntentExecutionAdvanceStop),
}

#[derive(Default)]
pub(crate) struct UiIntentExecutionAdvanceMetrics {
    pub(crate) active_slots_visited: usize,
    pub(crate) provider_calls: usize,
    pub(crate) provider_polls: usize,
    pub(crate) cancellation_calls: usize,
    pub(crate) settlements: usize,
}

impl UiIntentExecutionTransition {
    pub(crate) const fn new(
        attempt: super::UiIntentExecutionAttemptIdentity,
        idempotency: super::UiIntentExecutionIdempotencyIdentity,
        posture: UiIntentExecutionTransitionPosture,
        recovery: Option<super::UiIntentRecoveryHandle>,
    ) -> Self {
        Self {
            attempt,
            idempotency,
            posture,
            recovery,
            consequence: None,
            posture_basis: None,
        }
    }

    pub(crate) const fn completed(
        attempt: super::UiIntentExecutionAttemptIdentity,
        idempotency: super::UiIntentExecutionIdempotencyIdentity,
        outcome: crate::capability::UiIntentSchema,
        consequence: super::UiIntentConsequenceHandle,
    ) -> Self {
        Self {
            attempt,
            idempotency,
            posture: UiIntentExecutionTransitionPosture::Completed { outcome },
            recovery: None,
            consequence: Some(consequence),
            posture_basis: None,
        }
    }

    pub const fn attempt(&self) -> super::UiIntentExecutionAttemptIdentity {
        self.attempt
    }

    pub const fn idempotency(&self) -> super::UiIntentExecutionIdempotencyIdentity {
        self.idempotency
    }

    pub const fn posture(&self) -> UiIntentExecutionTransitionPosture {
        self.posture
    }

    pub fn into_recovery(self) -> Option<super::UiIntentRecoveryHandle> {
        self.recovery
    }

    pub fn into_consequence(self) -> Option<super::UiIntentConsequenceHandle> {
        self.consequence
    }

    pub(crate) const fn with_posture_basis(
        mut self,
        posture_basis: UiIntentExecutionPostureBasis,
    ) -> Self {
        self.posture_basis = Some(posture_basis);
        self
    }

    pub(crate) const fn posture_basis(&self) -> Option<UiIntentExecutionPostureBasis> {
        self.posture_basis
    }
}

impl UiIntentExecutionAdvanceReport {
    pub(crate) fn new(
        transitions: Vec<UiIntentExecutionTransition>,
        metrics: UiIntentExecutionAdvanceMetrics,
    ) -> Self {
        Self {
            transitions: transitions.into_boxed_slice(),
            active_slots_visited: metrics.active_slots_visited,
            provider_calls: metrics.provider_calls,
            provider_polls: metrics.provider_polls,
            cancellation_calls: metrics.cancellation_calls,
            settlements: metrics.settlements,
        }
    }

    pub fn transitions(&self) -> &[UiIntentExecutionTransition] {
        &self.transitions
    }

    pub fn into_transitions(self) -> Box<[UiIntentExecutionTransition]> {
        self.transitions
    }

    pub const fn active_slots_visited(&self) -> usize {
        self.active_slots_visited
    }

    pub const fn provider_calls(&self) -> usize {
        self.provider_calls
    }

    pub const fn provider_polls(&self) -> usize {
        self.provider_polls
    }

    pub const fn cancellation_calls(&self) -> usize {
        self.cancellation_calls
    }

    pub const fn settlements(&self) -> usize {
        self.settlements
    }
}
