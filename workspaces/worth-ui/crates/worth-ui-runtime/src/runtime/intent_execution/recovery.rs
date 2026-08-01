use std::sync::Arc;

#[must_use]
pub struct UiIntentRecoveryHandle {
    attempt: super::UiIntentExecutionAttemptIdentity,
    idempotency: super::UiIntentExecutionIdempotencyIdentity,
    lease: Arc<UiIntentRecoveryLease>,
}

pub(crate) struct UiIntentRecoveryLease;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentRecoveryProgressPosture {
    Pending,
    Completed {
        outcome: crate::capability::UiIntentSchema,
    },
    Partial {
        outcome: Option<crate::capability::UiIntentSchema>,
        detail: super::UiIntentProviderStop,
    },
    Indeterminate {
        detail: super::UiIntentProviderStop,
    },
    Failed {
        detail: super::UiIntentProviderStop,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentRecoveryProgressStop {
    StaleOrForeign,
    MonotonicTimeRegressed { previous: u64, observed: u64 },
}

#[must_use]
pub struct UiIntentRecoveryProgressReceipt {
    attempt: super::UiIntentExecutionAttemptIdentity,
    idempotency: super::UiIntentExecutionIdempotencyIdentity,
    posture: UiIntentRecoveryProgressPosture,
    continuation: Option<UiIntentRecoveryHandle>,
    consequence: Option<super::UiIntentConsequenceHandle>,
}

#[must_use]
pub enum UiIntentRecoveryProgressOutcome {
    Progressed(UiIntentRecoveryProgressReceipt),
    Stopped {
        reason: UiIntentRecoveryProgressStop,
        recovery: UiIntentRecoveryHandle,
    },
}

impl UiIntentRecoveryHandle {
    pub(crate) fn new(
        attempt: super::UiIntentExecutionAttemptIdentity,
        idempotency: super::UiIntentExecutionIdempotencyIdentity,
    ) -> (Self, Arc<UiIntentRecoveryLease>) {
        let lease = Arc::new(UiIntentRecoveryLease);
        (
            Self {
                attempt,
                idempotency,
                lease: Arc::clone(&lease),
            },
            lease,
        )
    }

    pub const fn attempt(&self) -> super::UiIntentExecutionAttemptIdentity {
        self.attempt
    }

    pub const fn idempotency(&self) -> super::UiIntentExecutionIdempotencyIdentity {
        self.idempotency
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        super::UiIntentExecutionAttemptIdentity,
        super::UiIntentExecutionIdempotencyIdentity,
        Arc<UiIntentRecoveryLease>,
    ) {
        (self.attempt, self.idempotency, self.lease)
    }

    pub(crate) fn from_parts(
        attempt: super::UiIntentExecutionAttemptIdentity,
        idempotency: super::UiIntentExecutionIdempotencyIdentity,
        lease: Arc<UiIntentRecoveryLease>,
    ) -> Self {
        Self {
            attempt,
            idempotency,
            lease,
        }
    }

    pub(crate) const fn lease(&self) -> &Arc<UiIntentRecoveryLease> {
        &self.lease
    }
}

impl UiIntentRecoveryProgressReceipt {
    pub(crate) const fn new(
        attempt: super::UiIntentExecutionAttemptIdentity,
        idempotency: super::UiIntentExecutionIdempotencyIdentity,
        posture: UiIntentRecoveryProgressPosture,
        continuation: Option<UiIntentRecoveryHandle>,
        consequence: Option<super::UiIntentConsequenceHandle>,
    ) -> Self {
        Self {
            attempt,
            idempotency,
            posture,
            continuation,
            consequence,
        }
    }

    pub const fn attempt(&self) -> super::UiIntentExecutionAttemptIdentity {
        self.attempt
    }

    pub const fn idempotency(&self) -> super::UiIntentExecutionIdempotencyIdentity {
        self.idempotency
    }

    pub const fn posture(&self) -> UiIntentRecoveryProgressPosture {
        self.posture
    }

    pub fn into_continuation(self) -> Option<UiIntentRecoveryHandle> {
        self.continuation
    }

    pub fn into_consequence(self) -> Option<super::UiIntentConsequenceHandle> {
        self.consequence
    }
}
