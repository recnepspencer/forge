use std::sync::Arc;

/// Move-only continuation from one completed product effect into its declared
/// consequence batch. It carries no provider or effect-invocation authority.
#[must_use]
pub struct UiIntentConsequenceHandle {
    attempt: super::UiIntentExecutionAttemptIdentity,
    idempotency: super::UiIntentExecutionIdempotencyIdentity,
    lease: Arc<UiIntentConsequenceLease>,
}

pub(crate) struct UiIntentConsequenceLease;

/// Move-only authority to retry only the consequence handoff of an already
/// completed effect.
#[must_use]
pub struct UiIntentConsequenceRecovery {
    attempt: super::UiIntentExecutionAttemptIdentity,
    idempotency: super::UiIntentExecutionIdempotencyIdentity,
    lease: Arc<UiIntentConsequenceLease>,
}

#[derive(Debug)]
pub enum UiIntentConsequenceStopReason {
    StaleOrForeign,
    ApplicationGenerationChanged,
    TargetChanged(crate::runtime::interaction::UiInteractionTargetingDenial),
    ProductRouteChanged,
    MultipleQueryConsequences,
    UndeclaredQueryConsequence {
        observed: worth_ui_query_binding::WorthUiQueryViewIdentity,
    },
    MissingDeclaredQueryConsequence {
        expected: worth_ui_query_binding::WorthUiQueryViewIdentity,
    },
    QueryConsequenceIdentityMismatch {
        expected: worth_ui_query_binding::WorthUiQueryViewIdentity,
        observed: worth_ui_query_binding::WorthUiQueryViewIdentity,
    },
    ConsequenceFactCapacityExceeded {
        limit: usize,
        observed: usize,
    },
    ObservationTurn(crate::runtime::observation::UiObservationTurnDenial),
    ObservationAdmission(crate::runtime::observation::UiObservationAdmissionDenial),
    QueryHandoff(worth_ui_query_binding::WorthUiCollectionChangeHandoffRetryDenial),
    QueryAdmission(worth_ui_query_binding::WorthUiCollectionChangeAdmissionDenial),
    RebindAdmission(crate::runtime::rebind::UiRebindReservationDenial),
    MountedRetention(crate::mounting::UiMountedFrameRetentionDenial),
    MountedPresentation(crate::mounting::UiMountedPresentationAdmissionDenial),
    HostRejectedBeforeEffects {
        rejection_count: usize,
    },
    AffectedScope(Box<crate::runtime::rebind::UiAffectedScopeDenial>),
    IdentityLifecycle(Box<crate::runtime::rebind::UiIdentityLifecycleDenial>),
    Planning(Box<crate::runtime::rebind::UiRebindPlanningDenial>),
    Preparation(Box<crate::runtime::rebind::UiRebindPreparationDenial>),
    IntentPostureIdentityExhausted,
}

#[must_use]
pub struct UiIntentConsequenceStop {
    reason: UiIntentConsequenceStopReason,
    recovery: UiIntentConsequenceRecovery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentConsequenceCompletionReceipt {
    attempt: super::UiIntentExecutionAttemptIdentity,
    idempotency: super::UiIntentExecutionIdempotencyIdentity,
}

impl UiIntentConsequenceHandle {
    pub(crate) fn new(
        attempt: super::UiIntentExecutionAttemptIdentity,
        idempotency: super::UiIntentExecutionIdempotencyIdentity,
    ) -> (Self, Arc<UiIntentConsequenceLease>) {
        let lease = Arc::new(UiIntentConsequenceLease);
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
        Arc<UiIntentConsequenceLease>,
    ) {
        (self.attempt, self.idempotency, self.lease)
    }

    pub(crate) const fn lease(&self) -> &Arc<UiIntentConsequenceLease> {
        &self.lease
    }
}

impl UiIntentConsequenceRecovery {
    pub const fn attempt(&self) -> super::UiIntentExecutionAttemptIdentity {
        self.attempt
    }

    pub const fn idempotency(&self) -> super::UiIntentExecutionIdempotencyIdentity {
        self.idempotency
    }

    pub(crate) fn from_parts(
        attempt: super::UiIntentExecutionAttemptIdentity,
        idempotency: super::UiIntentExecutionIdempotencyIdentity,
        lease: Arc<UiIntentConsequenceLease>,
    ) -> Self {
        Self {
            attempt,
            idempotency,
            lease,
        }
    }

    pub(crate) fn into_handle(self) -> UiIntentConsequenceHandle {
        UiIntentConsequenceHandle {
            attempt: self.attempt,
            idempotency: self.idempotency,
            lease: self.lease,
        }
    }
}

impl UiIntentConsequenceStop {
    pub(crate) const fn new(
        reason: UiIntentConsequenceStopReason,
        recovery: UiIntentConsequenceRecovery,
    ) -> Self {
        Self { reason, recovery }
    }

    pub const fn reason(&self) -> &UiIntentConsequenceStopReason {
        &self.reason
    }

    pub fn into_recovery(self) -> UiIntentConsequenceRecovery {
        self.recovery
    }
}

impl UiIntentConsequenceCompletionReceipt {
    pub(crate) const fn new(
        attempt: super::UiIntentExecutionAttemptIdentity,
        idempotency: super::UiIntentExecutionIdempotencyIdentity,
    ) -> Self {
        Self {
            attempt,
            idempotency,
        }
    }

    pub const fn attempt(self) -> super::UiIntentExecutionAttemptIdentity {
        self.attempt
    }

    pub const fn idempotency(self) -> super::UiIntentExecutionIdempotencyIdentity {
        self.idempotency
    }
}
