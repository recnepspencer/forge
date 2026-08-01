use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};

use worth_ui::facade::intent::{
    UiIntentExecutionAttempt, UiIntentExecutionCancellationContext, UiIntentExecutionPollContext,
    UiIntentExecutionProvider, UiIntentExecutionRecovery, UiIntentExecutionRequest,
    UiIntentProviderPoll, UiIntentProviderRecoveryPoll, UiIntentProviderSettlement,
    UiIntentProviderStart, UiIntentProviderStop, UiIntentProviderVersion,
};

use super::{PlatformPulseAction, PlatformPulseActionPayload};

mod executor_gate;
mod product_port;

pub use executor_gate::{PlatformPulseExecutorGate, PlatformPulseExecutorGateRevisionDenial};
pub use product_port::{
    PlatformPulseActionAttemptReference, PlatformPulseActionPort, PlatformPulseActionPortCensus,
    PlatformPulseActionPortOwner, PlatformPulseActionPortRequest,
};
use product_port::{PlatformPulseActionPortSubmission, PlatformPulseProductSettlement};

#[cfg(test)]
mod tests;

const PORT_CLOSED: UiIntentProviderStop = UiIntentProviderStop::stable("pulse-action-port-closed");
const PORT_FULL: UiIntentProviderStop = UiIntentProviderStop::stable("pulse-action-port-full");
const PRODUCT_REJECTED: UiIntentProviderStop =
    UiIntentProviderStop::stable("pulse-action-product-rejected");
const PRODUCT_FAILED: UiIntentProviderStop =
    UiIntentProviderStop::stable("pulse-action-product-failed");
const CANCELLED: UiIntentProviderStop = UiIntentProviderStop::stable("pulse-action-cancelled");
const PRODUCT_INDETERMINATE: UiIntentProviderStop =
    UiIntentProviderStop::stable("pulse-action-product-indeterminate");

#[derive(Clone)]
pub struct PlatformPulseActionProvider {
    port: PlatformPulseActionPort,
    gate: PlatformPulseExecutorGate,
}

enum PlatformPulseActionAttemptState {
    AwaitingGate(PlatformPulseActionPayload),
    AwaitingProduct(mpsc::Receiver<PlatformPulseProductSettlement>),
    Terminal,
}

struct PlatformPulseActionAttempt {
    reference: PlatformPulseActionAttemptReference,
    state: PlatformPulseActionAttemptState,
    port: PlatformPulseActionPort,
    gate: PlatformPulseExecutorGate,
    cancellation: Arc<AtomicBool>,
}

struct PlatformPulseActionRecovery;

impl PlatformPulseActionProvider {
    pub fn new(port: PlatformPulseActionPort, gate: PlatformPulseExecutorGate) -> Self {
        Self { port, gate }
    }
}

impl UiIntentExecutionProvider<PlatformPulseAction> for PlatformPulseActionProvider {
    const VERSION: UiIntentProviderVersion = UiIntentProviderVersion::stable(1);

    fn begin(
        &self,
        request: UiIntentExecutionRequest<PlatformPulseAction>,
    ) -> UiIntentProviderStart<PlatformPulseAction> {
        let reference = PlatformPulseActionAttemptReference::from_execution(
            request.attempt(),
            request.idempotency(),
        );
        UiIntentProviderStart::Started(Box::new(PlatformPulseActionAttempt {
            reference,
            state: PlatformPulseActionAttemptState::AwaitingGate(request.into_payload()),
            port: self.port.clone(),
            gate: self.gate.clone(),
            cancellation: Arc::new(AtomicBool::new(false)),
        }))
    }
}

impl UiIntentExecutionAttempt<PlatformPulseAction> for PlatformPulseActionAttempt {
    fn poll(
        &mut self,
        _context: UiIntentExecutionPollContext,
    ) -> UiIntentProviderPoll<PlatformPulseAction> {
        match &mut self.state {
            PlatformPulseActionAttemptState::AwaitingGate(_) if self.gate.is_held() => {
                UiIntentProviderPoll::PendingBeforeEffect
            }
            PlatformPulseActionAttemptState::AwaitingGate(_) => self.submit(),
            PlatformPulseActionAttemptState::AwaitingProduct(receiver) => {
                match receiver.try_recv() {
                    Ok(settlement) => {
                        self.state = PlatformPulseActionAttemptState::Terminal;
                        settled_poll(settlement)
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        UiIntentProviderPoll::PendingEffectMayHaveBegun
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.state = PlatformPulseActionAttemptState::Terminal;
                        failed_poll(PORT_CLOSED)
                    }
                }
            }
            PlatformPulseActionAttemptState::Terminal => failed_poll(PORT_CLOSED),
        }
    }

    fn cancel(
        &mut self,
        _context: UiIntentExecutionCancellationContext,
    ) -> UiIntentProviderPoll<PlatformPulseAction> {
        self.cancellation.store(true, Ordering::Release);
        match self.state {
            PlatformPulseActionAttemptState::AwaitingGate(_) => {
                self.state = PlatformPulseActionAttemptState::Terminal;
                cancelled_poll(CANCELLED)
            }
            PlatformPulseActionAttemptState::AwaitingProduct(_) => self.poll_product_after_cancel(),
            PlatformPulseActionAttemptState::Terminal => cancelled_poll(CANCELLED),
        }
    }
}

impl PlatformPulseActionAttempt {
    fn submit(&mut self) -> UiIntentProviderPoll<PlatformPulseAction> {
        let PlatformPulseActionAttemptState::AwaitingGate(payload) =
            std::mem::replace(&mut self.state, PlatformPulseActionAttemptState::Terminal)
        else {
            return failed_poll(PORT_CLOSED);
        };
        match self.port.try_submit(
            self.reference,
            payload.action_input_revision(),
            Arc::clone(&self.cancellation),
        ) {
            PlatformPulseActionPortSubmission::Accepted(receiver) => {
                self.state = PlatformPulseActionAttemptState::AwaitingProduct(receiver);
                UiIntentProviderPoll::PendingEffectMayHaveBegun
            }
            PlatformPulseActionPortSubmission::Full => failed_poll(PORT_FULL),
            PlatformPulseActionPortSubmission::Closed => failed_poll(PORT_CLOSED),
        }
    }

    fn poll_product_after_cancel(&mut self) -> UiIntentProviderPoll<PlatformPulseAction> {
        let PlatformPulseActionAttemptState::AwaitingProduct(receiver) = &mut self.state else {
            return cancelled_poll(CANCELLED);
        };
        match receiver.try_recv() {
            Ok(settlement) => {
                self.state = PlatformPulseActionAttemptState::Terminal;
                settled_poll(settlement)
            }
            Err(mpsc::TryRecvError::Empty) => UiIntentProviderPoll::PendingEffectMayHaveBegun,
            Err(mpsc::TryRecvError::Disconnected) => {
                self.state = PlatformPulseActionAttemptState::Terminal;
                failed_poll(PORT_CLOSED)
            }
        }
    }
}

fn settled_poll(
    settlement: PlatformPulseProductSettlement,
) -> UiIntentProviderPoll<PlatformPulseAction> {
    let settlement = match settlement {
        PlatformPulseProductSettlement::Completed(outcome) => {
            UiIntentProviderSettlement::Completed(outcome)
        }
        PlatformPulseProductSettlement::Rejected => {
            UiIntentProviderSettlement::RejectedBeforeEffect(PRODUCT_REJECTED)
        }
        PlatformPulseProductSettlement::Failed => {
            UiIntentProviderSettlement::FailedBeforeEffect(PRODUCT_FAILED)
        }
        PlatformPulseProductSettlement::Cancelled => {
            UiIntentProviderSettlement::CancelledBeforeEffect(CANCELLED)
        }
        PlatformPulseProductSettlement::Indeterminate => {
            UiIntentProviderSettlement::Indeterminate(Box::new(PlatformPulseActionRecovery))
        }
    };
    UiIntentProviderPoll::Settled(settlement)
}

impl UiIntentExecutionRecovery<PlatformPulseAction> for PlatformPulseActionRecovery {
    fn poll_recovery(
        &mut self,
        _context: UiIntentExecutionPollContext,
    ) -> UiIntentProviderRecoveryPoll<PlatformPulseAction> {
        UiIntentProviderRecoveryPoll::Indeterminate(PRODUCT_INDETERMINATE)
    }
}

fn failed_poll(stop: UiIntentProviderStop) -> UiIntentProviderPoll<PlatformPulseAction> {
    UiIntentProviderPoll::Settled(UiIntentProviderSettlement::FailedBeforeEffect(stop))
}

fn cancelled_poll(stop: UiIntentProviderStop) -> UiIntentProviderPoll<PlatformPulseAction> {
    UiIntentProviderPoll::Settled(UiIntentProviderSettlement::CancelledBeforeEffect(stop))
}
