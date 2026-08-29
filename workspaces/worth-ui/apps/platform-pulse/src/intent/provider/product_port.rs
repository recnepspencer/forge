use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};

use worth_ui::facade::query_binding::UiProjectionObservation;

use crate::intent::{PlatformPulseActionInputRevision, PlatformPulseActionOutcome};

const PRODUCT_PORT_CAPACITY: usize = 16;

#[derive(Clone)]
pub struct PlatformPulseActionPort {
    sender: mpsc::SyncSender<PlatformPulseActionPortRequest>,
    census: Arc<PlatformPulseActionPortCensusState>,
}

pub struct PlatformPulseActionPortOwner {
    receiver: mpsc::Receiver<PlatformPulseActionPortRequest>,
    census: Arc<PlatformPulseActionPortCensusState>,
}

pub struct PlatformPulseActionPortRequest {
    reference: PlatformPulseActionAttemptReference,
    action_input_revision: PlatformPulseActionInputRevision,
    query_denial_requested: bool,
    cancellation: Arc<AtomicBool>,
    effect_started: Arc<AtomicBool>,
    completion: Option<mpsc::SyncSender<PlatformPulseProductSettlement>>,
    census: Arc<PlatformPulseActionPortCensusState>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlatformPulseActionAttemptReference {
    attempt_slot: u8,
    attempt_generation: u64,
    idempotency_session: u64,
    idempotency_lineage: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseActionPortCensus {
    submitted: usize,
    received: usize,
    settled: usize,
    retained: usize,
}

#[derive(Default)]
struct PlatformPulseActionPortCensusState {
    submitted: AtomicUsize,
    received: AtomicUsize,
    settled: AtomicUsize,
    retained: AtomicUsize,
}

pub(super) enum PlatformPulseProductSettlement {
    Completed(PlatformPulseActionOutcome),
    Rejected,
    Failed,
    Cancelled,
    Indeterminate,
}

pub(super) enum PlatformPulseActionPortSubmission {
    Accepted {
        receiver: mpsc::Receiver<PlatformPulseProductSettlement>,
        effect_started: Arc<AtomicBool>,
    },
    Full,
    Closed,
}

impl PlatformPulseActionPort {
    pub fn bounded() -> (Self, PlatformPulseActionPortOwner) {
        let (sender, receiver) = mpsc::sync_channel(PRODUCT_PORT_CAPACITY);
        let census = Arc::new(PlatformPulseActionPortCensusState::default());
        (
            Self {
                sender,
                census: Arc::clone(&census),
            },
            PlatformPulseActionPortOwner { receiver, census },
        )
    }

    pub fn census(&self) -> PlatformPulseActionPortCensus {
        census(&self.census)
    }

    pub(super) fn try_submit(
        &self,
        reference: PlatformPulseActionAttemptReference,
        action_input_revision: PlatformPulseActionInputRevision,
        query_denial_requested: bool,
        cancellation: Arc<AtomicBool>,
    ) -> PlatformPulseActionPortSubmission {
        let (completion, receiver) = mpsc::sync_channel(1);
        let effect_started = Arc::new(AtomicBool::new(false));
        let request = PlatformPulseActionPortRequest {
            reference,
            action_input_revision,
            query_denial_requested,
            cancellation,
            effect_started: Arc::clone(&effect_started),
            completion: Some(completion),
            census: Arc::clone(&self.census),
        };
        self.census.retained.fetch_add(1, Ordering::Relaxed);
        match self.sender.try_send(request) {
            Ok(()) => {
                self.census.submitted.fetch_add(1, Ordering::Relaxed);
                PlatformPulseActionPortSubmission::Accepted {
                    receiver,
                    effect_started,
                }
            }
            Err(mpsc::TrySendError::Full(_)) => PlatformPulseActionPortSubmission::Full,
            Err(mpsc::TrySendError::Disconnected(_)) => PlatformPulseActionPortSubmission::Closed,
        }
    }
}

impl PlatformPulseActionPortOwner {
    pub fn try_next(&self) -> Option<PlatformPulseActionPortRequest> {
        let request = self.receiver.try_recv().ok()?;
        self.census.received.fetch_add(1, Ordering::Relaxed);
        Some(request)
    }

    pub fn census(&self) -> PlatformPulseActionPortCensus {
        census(&self.census)
    }
}

impl PlatformPulseActionPortRequest {
    pub const fn reference(&self) -> PlatformPulseActionAttemptReference {
        self.reference
    }

    pub const fn action_input_revision(&self) -> PlatformPulseActionInputRevision {
        self.action_input_revision
    }

    pub const fn query_denial_requested(&self) -> bool {
        self.query_denial_requested
    }

    pub fn cancellation_requested(&self) -> bool {
        self.cancellation.load(Ordering::Acquire)
    }

    pub fn begin_effect(&self) -> bool {
        !self.cancellation_requested()
            && self
                .effect_started
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
    }

    pub fn complete(self, consequence: UiProjectionObservation) -> bool {
        if !self.effect_started.load(Ordering::Acquire) {
            let _ = self.settle(PlatformPulseProductSettlement::Indeterminate);
            return false;
        }
        self.settle(PlatformPulseProductSettlement::Completed(
            PlatformPulseActionOutcome::query(consequence),
        ))
    }

    pub fn reject_before_effect(self) -> bool {
        self.settle_before_effect(PlatformPulseProductSettlement::Rejected)
    }

    pub fn fail_before_effect(self) -> bool {
        self.settle_before_effect(PlatformPulseProductSettlement::Failed)
    }

    pub fn cancel_before_effect(self) -> bool {
        self.settle_before_effect(PlatformPulseProductSettlement::Cancelled)
    }

    pub fn settle_indeterminate(self) -> bool {
        self.settle(PlatformPulseProductSettlement::Indeterminate)
    }

    fn settle_before_effect(self, settlement: PlatformPulseProductSettlement) -> bool {
        if self.effect_started.load(Ordering::Acquire) {
            let _ = self.settle(PlatformPulseProductSettlement::Indeterminate);
            return false;
        }
        self.settle(settlement)
    }

    fn settle(mut self, settlement: PlatformPulseProductSettlement) -> bool {
        let sent = self
            .completion
            .take()
            .is_some_and(|completion| completion.send(settlement).is_ok());
        self.census.settled.fetch_add(1, Ordering::Relaxed);
        self.census.retained.fetch_sub(1, Ordering::Relaxed);
        sent
    }
}

impl PlatformPulseActionAttemptReference {
    pub(super) const fn from_execution(
        attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
        idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
    ) -> Self {
        Self {
            attempt_slot: attempt.slot(),
            attempt_generation: attempt.generation(),
            idempotency_session: idempotency.session(),
            idempotency_lineage: idempotency.lineage(),
        }
    }

    #[cfg(test)]
    pub(super) const fn for_test(lineage: u64) -> Self {
        Self {
            attempt_slot: 0,
            attempt_generation: lineage,
            idempotency_session: 1,
            idempotency_lineage: lineage,
        }
    }

    pub const fn attempt_slot(self) -> u8 {
        self.attempt_slot
    }

    pub const fn attempt_generation(self) -> u64 {
        self.attempt_generation
    }

    pub const fn idempotency_session(self) -> u64 {
        self.idempotency_session
    }

    pub const fn idempotency_lineage(self) -> u64 {
        self.idempotency_lineage
    }

    pub fn matches_execution(
        self,
        attempt: worth_ui::facade::intent::UiIntentExecutionAttemptIdentity,
        idempotency: worth_ui::facade::intent::UiIntentExecutionIdempotencyIdentity,
    ) -> bool {
        self.attempt_slot == attempt.slot()
            && self.attempt_generation == attempt.generation()
            && self.idempotency_session == idempotency.session()
            && self.idempotency_lineage == idempotency.lineage()
    }
}

impl Drop for PlatformPulseActionPortRequest {
    fn drop(&mut self) {
        if self.completion.take().is_some() {
            self.census.retained.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

impl PlatformPulseActionPortCensus {
    pub const fn submitted(self) -> usize {
        self.submitted
    }

    pub const fn received(self) -> usize {
        self.received
    }

    pub const fn settled(self) -> usize {
        self.settled
    }

    pub const fn retained(self) -> usize {
        self.retained
    }
}

fn census(state: &PlatformPulseActionPortCensusState) -> PlatformPulseActionPortCensus {
    PlatformPulseActionPortCensus {
        submitted: state.submitted.load(Ordering::Relaxed),
        received: state.received.load(Ordering::Relaxed),
        settled: state.settled.load(Ordering::Relaxed),
        retained: state.retained.load(Ordering::Relaxed),
    }
}
