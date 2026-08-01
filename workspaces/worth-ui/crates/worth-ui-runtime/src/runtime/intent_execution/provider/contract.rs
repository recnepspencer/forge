use super::{
    UiIntentExecutionAttemptIdentity, UiIntentExecutionIdempotencyIdentity, UiIntentProviderVersion,
};

pub trait UiIntentExecutionProvider<I: crate::capability::UiIntent>: Send + Sync + 'static {
    const VERSION: UiIntentProviderVersion;

    fn begin(&self, request: UiIntentExecutionRequest<I>) -> UiIntentProviderStart<I>;
}

pub trait UiIntentExecutionAttempt<I: crate::capability::UiIntent>: Send + 'static {
    fn poll(&mut self, context: UiIntentExecutionPollContext) -> UiIntentProviderPoll<I>;

    fn cancel(&mut self, context: UiIntentExecutionCancellationContext) -> UiIntentProviderPoll<I>;
}

pub trait UiIntentExecutionRecovery<I: crate::capability::UiIntent>: Send + 'static {
    fn poll_recovery(
        &mut self,
        context: UiIntentExecutionPollContext,
    ) -> UiIntentProviderRecoveryPoll<I>;
}

#[must_use]
pub struct UiIntentExecutionRequest<I: crate::capability::UiIntent> {
    attempt: UiIntentExecutionAttemptIdentity,
    idempotency: UiIntentExecutionIdempotencyIdentity,
    payload: I::Payload,
    deadline: UiIntentExecutionDeadline,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentExecutionDeadline {
    tick: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentExecutionPollContext {
    tick: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentExecutionCancellationContext {
    tick: u64,
    reason: UiIntentExecutionCancellationReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentExecutionCancellationReason {
    Requested,
    DeadlineExpired,
    MountedInstanceRemoved,
    SurfaceRebound,
    ApplicationRebound,
    Shutdown,
}

#[must_use]
pub enum UiIntentProviderStart<I: crate::capability::UiIntent> {
    Started(Box<dyn UiIntentExecutionAttempt<I>>),
    RejectedBeforeEffect(UiIntentProviderStop),
}

#[must_use]
pub enum UiIntentProviderPoll<I: crate::capability::UiIntent> {
    PendingBeforeEffect,
    PendingEffectMayHaveBegun,
    Settled(UiIntentProviderSettlement<I>),
}

#[must_use]
pub enum UiIntentProviderSettlement<I: crate::capability::UiIntent> {
    Completed(I::ProductOutcome),
    RejectedBeforeEffect(UiIntentProviderStop),
    FailedBeforeEffect(UiIntentProviderStop),
    CancelledBeforeEffect(UiIntentProviderStop),
    TimedOutBeforeEffect(UiIntentProviderStop),
    Partial(
        UiIntentPartialEffect<I::ProductOutcome>,
        Box<dyn UiIntentExecutionRecovery<I>>,
    ),
    Indeterminate(Box<dyn UiIntentExecutionRecovery<I>>),
}

#[must_use]
pub enum UiIntentProviderRecoveryPoll<I: crate::capability::UiIntent> {
    Pending,
    Completed(I::ProductOutcome),
    Partial(UiIntentPartialEffect<I::ProductOutcome>),
    Indeterminate(UiIntentProviderStop),
    Failed(UiIntentProviderStop),
}

pub struct UiIntentPartialEffect<O> {
    outcome: Option<O>,
    detail: UiIntentProviderStop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentProviderStop {
    code: &'static str,
}

impl<I: crate::capability::UiIntent> UiIntentExecutionRequest<I> {
    pub(crate) const fn new(
        attempt: UiIntentExecutionAttemptIdentity,
        idempotency: UiIntentExecutionIdempotencyIdentity,
        payload: I::Payload,
        deadline: UiIntentExecutionDeadline,
    ) -> Self {
        Self {
            attempt,
            idempotency,
            payload,
            deadline,
        }
    }

    pub const fn attempt(&self) -> UiIntentExecutionAttemptIdentity {
        self.attempt
    }

    pub const fn idempotency(&self) -> UiIntentExecutionIdempotencyIdentity {
        self.idempotency
    }

    pub const fn deadline(&self) -> UiIntentExecutionDeadline {
        self.deadline
    }

    pub fn into_payload(self) -> I::Payload {
        self.payload
    }
}

impl UiIntentExecutionDeadline {
    pub(crate) const fn at_tick(tick: u64) -> Self {
        Self { tick }
    }

    pub const fn tick(self) -> u64 {
        self.tick
    }
}

impl UiIntentExecutionPollContext {
    pub(crate) const fn at_tick(tick: u64) -> Self {
        Self { tick }
    }

    pub const fn tick(self) -> u64 {
        self.tick
    }
}

impl UiIntentExecutionCancellationContext {
    pub(crate) const fn new(tick: u64, reason: UiIntentExecutionCancellationReason) -> Self {
        Self { tick, reason }
    }

    pub const fn tick(self) -> u64 {
        self.tick
    }

    pub const fn reason(self) -> UiIntentExecutionCancellationReason {
        self.reason
    }
}

impl<O> UiIntentPartialEffect<O> {
    pub fn with_outcome(outcome: O, detail: UiIntentProviderStop) -> Self {
        Self {
            outcome: Some(outcome),
            detail,
        }
    }

    pub const fn without_outcome(detail: UiIntentProviderStop) -> Self {
        Self {
            outcome: None,
            detail,
        }
    }

    pub const fn detail(&self) -> UiIntentProviderStop {
        self.detail
    }

    pub fn into_outcome(self) -> Option<O> {
        self.outcome
    }
}

impl UiIntentProviderStop {
    pub const fn stable(code: &'static str) -> Self {
        assert!(
            !code.is_empty(),
            "intent provider stop code cannot be empty"
        );
        Self { code }
    }

    pub const fn code(self) -> &'static str {
        self.code
    }
}
