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
    cancellation: Arc<AtomicBool>,
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
    Accepted(mpsc::Receiver<PlatformPulseProductSettlement>),
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
        cancellation: Arc<AtomicBool>,
    ) -> PlatformPulseActionPortSubmission {
        let (completion, receiver) = mpsc::sync_channel(1);
        let request = PlatformPulseActionPortRequest {
            reference,
            action_input_revision,
            cancellation,
            completion: Some(completion),
            census: Arc::clone(&self.census),
        };
        self.census.retained.fetch_add(1, Ordering::Relaxed);
        match self.sender.try_send(request) {
            Ok(()) => {
                self.census.submitted.fetch_add(1, Ordering::Relaxed);
                PlatformPulseActionPortSubmission::Accepted(receiver)
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

    pub fn cancellation_requested(&self) -> bool {
        self.cancellation.load(Ordering::Acquire)
    }

    pub fn complete(self, consequence: UiProjectionObservation) -> bool {
        self.settle(PlatformPulseProductSettlement::Completed(
            PlatformPulseActionOutcome::query(consequence),
        ))
    }

    pub fn reject_before_effect(self) -> bool {
        self.settle(PlatformPulseProductSettlement::Rejected)
    }

    pub fn fail_before_effect(self) -> bool {
        self.settle(PlatformPulseProductSettlement::Failed)
    }

    pub fn cancel_before_effect(self) -> bool {
        self.settle(PlatformPulseProductSettlement::Cancelled)
    }

    pub fn settle_indeterminate(self) -> bool {
        self.settle(PlatformPulseProductSettlement::Indeterminate)
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
