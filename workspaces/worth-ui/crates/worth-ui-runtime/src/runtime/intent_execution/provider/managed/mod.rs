mod execution;
mod material;
mod recovery;

use super::{
    UiIntentExecutionAttemptIdentity, UiIntentExecutionCancellationContext,
    UiIntentExecutionDeadline, UiIntentExecutionIdempotencyIdentity, UiIntentExecutionPollContext,
    UiIntentProviderStop,
};

pub(super) use execution::UiTypedManagedIntentExecution;
pub(super) use material::outcome_material;

#[derive(Clone, Copy)]
pub(crate) struct UiManagedIntentExecutionStartContext {
    attempt: UiIntentExecutionAttemptIdentity,
    idempotency: UiIntentExecutionIdempotencyIdentity,
    deadline: UiIntentExecutionDeadline,
}

pub(crate) enum UiManagedIntentExecutionStart {
    Running(Box<dyn UiManagedIntentExecution>),
    Settled(UiManagedIntentSettlement),
}

pub(crate) trait UiManagedIntentExecution: Send {
    fn poll(self: Box<Self>, context: UiIntentExecutionPollContext)
        -> UiManagedIntentExecutionPoll;

    fn cancel(
        self: Box<Self>,
        context: UiIntentExecutionCancellationContext,
    ) -> UiManagedIntentExecutionPoll;
}

pub(crate) enum UiManagedIntentExecutionPoll {
    PendingBeforeEffect(Box<dyn UiManagedIntentExecution>),
    PendingEffectMayHaveBegun(Box<dyn UiManagedIntentExecution>),
    Settled(UiManagedIntentSettlement),
}

pub(crate) enum UiManagedIntentSettlement {
    Completed(Box<dyn UiManagedIntentOutcomeMaterial>),
    RejectedBeforeEffect(UiIntentProviderStop),
    FailedBeforeEffect(UiIntentProviderStop),
    CancelledBeforeEffect(UiIntentProviderStop),
    TimedOutBeforeEffect(UiIntentProviderStop),
    Partial {
        effect: UiManagedIntentPartialEffect,
        recovery: Box<dyn UiManagedIntentRecovery>,
    },
    Indeterminate {
        detail: Option<UiIntentProviderStop>,
        recovery: Box<dyn UiManagedIntentRecovery>,
    },
}

pub(crate) trait UiManagedIntentOutcomeMaterial: Send {
    fn schema(&self) -> crate::capability::UiIntentSchema;

    fn into_consequences(self: Box<Self>) -> crate::capability::UiIntentProductConsequences;
}

pub(crate) struct UiManagedIntentPartialEffect {
    outcome: Option<Box<dyn UiManagedIntentOutcomeMaterial>>,
    detail: UiIntentProviderStop,
}

pub(crate) trait UiManagedIntentRecovery: Send {
    fn poll(self: Box<Self>, context: UiIntentExecutionPollContext) -> UiManagedIntentRecoveryPoll;
}

pub(crate) enum UiManagedIntentRecoveryPoll {
    Pending(Box<dyn UiManagedIntentRecovery>),
    Completed(Box<dyn UiManagedIntentOutcomeMaterial>),
    Partial {
        effect: UiManagedIntentPartialEffect,
        recovery: Box<dyn UiManagedIntentRecovery>,
    },
    Indeterminate {
        detail: UiIntentProviderStop,
        recovery: Box<dyn UiManagedIntentRecovery>,
    },
    Failed {
        detail: UiIntentProviderStop,
        recovery: Box<dyn UiManagedIntentRecovery>,
    },
}

impl UiManagedIntentExecutionStartContext {
    pub(crate) const fn new(
        attempt: UiIntentExecutionAttemptIdentity,
        idempotency: UiIntentExecutionIdempotencyIdentity,
        deadline: UiIntentExecutionDeadline,
    ) -> Self {
        Self {
            attempt,
            idempotency,
            deadline,
        }
    }

    pub(crate) const fn attempt(self) -> UiIntentExecutionAttemptIdentity {
        self.attempt
    }

    pub(crate) const fn idempotency(self) -> UiIntentExecutionIdempotencyIdentity {
        self.idempotency
    }

    pub(crate) const fn deadline(self) -> UiIntentExecutionDeadline {
        self.deadline
    }
}

impl UiManagedIntentPartialEffect {
    pub(super) fn new(
        outcome: Option<Box<dyn UiManagedIntentOutcomeMaterial>>,
        detail: UiIntentProviderStop,
    ) -> Self {
        Self { outcome, detail }
    }

    pub(crate) fn outcome_schema(&self) -> Option<crate::capability::UiIntentSchema> {
        self.outcome.as_ref().map(|outcome| outcome.schema())
    }

    pub(crate) const fn detail(&self) -> UiIntentProviderStop {
        self.detail
    }
}
