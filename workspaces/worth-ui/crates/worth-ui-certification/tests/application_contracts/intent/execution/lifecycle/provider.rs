use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use worth_ui::facade::intent::{
    UiIntentExecutionAttempt, UiIntentExecutionCancellationContext, UiIntentExecutionPollContext,
    UiIntentExecutionProvider, UiIntentExecutionRecovery, UiIntentExecutionRequest,
    UiIntentPartialEffect, UiIntentProviderPoll, UiIntentProviderRecoveryPoll,
    UiIntentProviderSettlement, UiIntentProviderStart, UiIntentProviderStop,
    UiIntentProviderVersion,
};

use crate::intent::operability::{EmptyOutcome, PrimaryIntent};

mod script;

use script::ScriptedStart;
pub(in crate::intent) use script::{AttemptStep, ExecutionScript, RecoveryStep};

pub(in crate::intent) struct ScriptedProvider {
    scripts: Arc<Mutex<VecDeque<ExecutionScript>>>,
    observation: Arc<ScriptedProviderObservationState>,
}

#[derive(Clone)]
pub(in crate::intent) struct ScriptedProviderObservation {
    state: Arc<ScriptedProviderObservationState>,
}

#[derive(Clone, Copy)]
pub(in crate::intent) struct RequestObservation {
    pub(in crate::intent) attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
    pub(in crate::intent) idempotency:
        worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
    pub(in crate::intent) deadline: worth_ui::facade::intent::UiIntentExecutionDeadline,
}

struct ScriptedProviderObservationState {
    requests: Mutex<Vec<RequestObservation>>,
    cancellations: Mutex<Vec<UiIntentExecutionCancellationContext>>,
    begin_calls: AtomicUsize,
    poll_calls: AtomicUsize,
    cancellation_calls: AtomicUsize,
    recovery_poll_calls: AtomicUsize,
    attempt_drops: AtomicUsize,
    recovery_drops: AtomicUsize,
    provider_drops: AtomicUsize,
}

struct ScriptedAttempt {
    polls: VecDeque<AttemptStep>,
    cancellations: VecDeque<AttemptStep>,
    recovery: VecDeque<RecoveryStep>,
    observation: Arc<ScriptedProviderObservationState>,
}

struct ScriptedRecovery {
    steps: VecDeque<RecoveryStep>,
    observation: Arc<ScriptedProviderObservationState>,
}

impl ScriptedProvider {
    pub(in crate::intent) fn new(
        scripts: impl IntoIterator<Item = ExecutionScript>,
    ) -> (Self, ScriptedProviderObservation) {
        let observation = Arc::new(ScriptedProviderObservationState::new());
        (
            Self {
                scripts: Arc::new(Mutex::new(scripts.into_iter().collect())),
                observation: Arc::clone(&observation),
            },
            ScriptedProviderObservation { state: observation },
        )
    }
}

impl UiIntentExecutionProvider<PrimaryIntent> for ScriptedProvider {
    const VERSION: UiIntentProviderVersion = UiIntentProviderVersion::stable(91);

    fn begin(
        &self,
        request: UiIntentExecutionRequest<PrimaryIntent>,
    ) -> UiIntentProviderStart<PrimaryIntent> {
        self.observation.begin_calls.fetch_add(1, Ordering::Relaxed);
        self.observation
            .requests
            .lock()
            .expect("scripted provider request observation lock")
            .push(RequestObservation {
                attempt: request.attempt(),
                idempotency: request.idempotency(),
                deadline: request.deadline(),
            });
        let _payload = request.into_payload();
        let script = self
            .scripts
            .lock()
            .expect("scripted provider queue lock")
            .pop_front()
            .expect("each provider begin has one declared script");
        match script.start {
            ScriptedStart::RejectedBeforeEffect => UiIntentProviderStart::RejectedBeforeEffect(
                UiIntentProviderStop::stable("certification.start_rejected"),
            ),
            ScriptedStart::Started => UiIntentProviderStart::Started(Box::new(ScriptedAttempt {
                polls: script.polls,
                cancellations: script.cancellations,
                recovery: script.recovery,
                observation: Arc::clone(&self.observation),
            })),
        }
    }
}

impl Drop for ScriptedProvider {
    fn drop(&mut self) {
        self.observation
            .provider_drops
            .fetch_add(1, Ordering::Relaxed);
    }
}

impl UiIntentExecutionAttempt<PrimaryIntent> for ScriptedAttempt {
    fn poll(
        &mut self,
        _context: UiIntentExecutionPollContext,
    ) -> UiIntentProviderPoll<PrimaryIntent> {
        self.observation.poll_calls.fetch_add(1, Ordering::Relaxed);
        attempt_poll(
            self.polls
                .pop_front()
                .unwrap_or(AttemptStep::PendingBeforeEffect),
            &mut self.recovery,
            &self.observation,
        )
    }

    fn cancel(
        &mut self,
        context: UiIntentExecutionCancellationContext,
    ) -> UiIntentProviderPoll<PrimaryIntent> {
        self.observation
            .cancellation_calls
            .fetch_add(1, Ordering::Relaxed);
        self.observation
            .cancellations
            .lock()
            .expect("scripted provider cancellation observation lock")
            .push(context);
        attempt_poll(
            self.cancellations
                .pop_front()
                .unwrap_or(AttemptStep::PendingBeforeEffect),
            &mut self.recovery,
            &self.observation,
        )
    }
}

impl Drop for ScriptedAttempt {
    fn drop(&mut self) {
        self.observation
            .attempt_drops
            .fetch_add(1, Ordering::Relaxed);
    }
}

impl UiIntentExecutionRecovery<PrimaryIntent> for ScriptedRecovery {
    fn poll_recovery(
        &mut self,
        _context: UiIntentExecutionPollContext,
    ) -> UiIntentProviderRecoveryPoll<PrimaryIntent> {
        self.observation
            .recovery_poll_calls
            .fetch_add(1, Ordering::Relaxed);
        match self.steps.pop_front().unwrap_or(RecoveryStep::Pending) {
            RecoveryStep::Pending => UiIntentProviderRecoveryPoll::Pending,
            RecoveryStep::Completed => UiIntentProviderRecoveryPoll::Completed(EmptyOutcome),
            RecoveryStep::PartialWithOutcome => UiIntentProviderRecoveryPoll::Partial(
                UiIntentPartialEffect::with_outcome(EmptyOutcome, recovery_stop()),
            ),
            RecoveryStep::PartialWithoutOutcome => UiIntentProviderRecoveryPoll::Partial(
                UiIntentPartialEffect::without_outcome(recovery_stop()),
            ),
            RecoveryStep::Indeterminate => {
                UiIntentProviderRecoveryPoll::Indeterminate(recovery_stop())
            }
            RecoveryStep::Failed => UiIntentProviderRecoveryPoll::Failed(recovery_stop()),
        }
    }
}

impl Drop for ScriptedRecovery {
    fn drop(&mut self) {
        self.observation
            .recovery_drops
            .fetch_add(1, Ordering::Relaxed);
    }
}

fn attempt_poll(
    step: AttemptStep,
    recovery: &mut VecDeque<RecoveryStep>,
    observation: &Arc<ScriptedProviderObservationState>,
) -> UiIntentProviderPoll<PrimaryIntent> {
    let settlement = match step {
        AttemptStep::PendingBeforeEffect => return UiIntentProviderPoll::PendingBeforeEffect,
        AttemptStep::PendingEffectMayHaveBegun => {
            return UiIntentProviderPoll::PendingEffectMayHaveBegun
        }
        AttemptStep::Completed => UiIntentProviderSettlement::Completed(EmptyOutcome),
        AttemptStep::RejectedBeforeEffect => {
            UiIntentProviderSettlement::RejectedBeforeEffect(attempt_stop())
        }
        AttemptStep::FailedBeforeEffect => {
            UiIntentProviderSettlement::FailedBeforeEffect(attempt_stop())
        }
        AttemptStep::CancelledBeforeEffect => {
            UiIntentProviderSettlement::CancelledBeforeEffect(attempt_stop())
        }
        AttemptStep::TimedOutBeforeEffect => {
            UiIntentProviderSettlement::TimedOutBeforeEffect(attempt_stop())
        }
        AttemptStep::PartialWithOutcome => UiIntentProviderSettlement::Partial(
            UiIntentPartialEffect::with_outcome(EmptyOutcome, attempt_stop()),
            scripted_recovery(recovery, observation),
        ),
        AttemptStep::PartialWithoutOutcome => UiIntentProviderSettlement::Partial(
            UiIntentPartialEffect::without_outcome(attempt_stop()),
            scripted_recovery(recovery, observation),
        ),
        AttemptStep::Indeterminate => {
            UiIntentProviderSettlement::Indeterminate(scripted_recovery(recovery, observation))
        }
    };
    UiIntentProviderPoll::Settled(settlement)
}

fn scripted_recovery(
    steps: &mut VecDeque<RecoveryStep>,
    observation: &Arc<ScriptedProviderObservationState>,
) -> Box<dyn UiIntentExecutionRecovery<PrimaryIntent>> {
    Box::new(ScriptedRecovery {
        steps: std::mem::take(steps),
        observation: Arc::clone(observation),
    })
}

const fn attempt_stop() -> UiIntentProviderStop {
    UiIntentProviderStop::stable("certification.attempt")
}

const fn recovery_stop() -> UiIntentProviderStop {
    UiIntentProviderStop::stable("certification.recovery")
}

impl ScriptedProviderObservationState {
    fn new() -> Self {
        Self {
            requests: Mutex::new(Vec::new()),
            cancellations: Mutex::new(Vec::new()),
            begin_calls: AtomicUsize::new(0),
            poll_calls: AtomicUsize::new(0),
            cancellation_calls: AtomicUsize::new(0),
            recovery_poll_calls: AtomicUsize::new(0),
            attempt_drops: AtomicUsize::new(0),
            recovery_drops: AtomicUsize::new(0),
            provider_drops: AtomicUsize::new(0),
        }
    }
}

impl ScriptedProviderObservation {
    pub(in crate::intent) fn requests(&self) -> Vec<RequestObservation> {
        self.state
            .requests
            .lock()
            .expect("scripted provider request observation lock")
            .clone()
    }

    pub(in crate::intent) fn counts(&self) -> [usize; 7] {
        [
            self.state.begin_calls.load(Ordering::Relaxed),
            self.state.poll_calls.load(Ordering::Relaxed),
            self.state.cancellation_calls.load(Ordering::Relaxed),
            self.state.recovery_poll_calls.load(Ordering::Relaxed),
            self.state.attempt_drops.load(Ordering::Relaxed),
            self.state.recovery_drops.load(Ordering::Relaxed),
            self.state.provider_drops.load(Ordering::Relaxed),
        ]
    }

    pub(in crate::intent) fn cancellations(&self) -> Vec<UiIntentExecutionCancellationContext> {
        self.state
            .cancellations
            .lock()
            .expect("scripted provider cancellation observation lock")
            .clone()
    }
}
