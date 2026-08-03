use serde_json::{json, Value};

use super::super::protocol::{
    BoundedCancellationCaseObservation, BoundedCancellationDispatch, BoundedCancellationObligation,
    BoundedCancellationObservation, BoundedCancellationRecovery, BoundedCancellationSeam,
    BoundedCancellationSignal, BoundedCancellationTerminal,
};
use crate::physical_work_evidence::hex;

pub(super) fn value(observation: BoundedCancellationObservation) -> Value {
    json!({
        "pre_dispatch": case(observation.pre_dispatch),
        "post_dispatch": case(observation.post_dispatch),
    })
}

fn case(observation: BoundedCancellationCaseObservation) -> Value {
    json!({
        "seam": seam(observation.seam),
        "store": hex(&observation.store),
        "runtime": observation.runtime,
        "generation": observation.generation,
        "operation": observation.operation,
        "obligation": obligation(observation.obligation),
        "signal": signal(observation.signal),
        "dispatch": dispatch(observation.dispatch),
        "recovery": recovery(observation.recovery),
        "terminal": terminal(observation.terminal),
        "media_before_cancellation": observation.media_before_cancellation,
        "cancellation_media_effects": observation.cancellation_media_effects,
        "terminal_media_effects": observation.terminal_media_effects,
        "backend_receipt": observation.backend_receipt,
    })
}

const fn seam(seam: BoundedCancellationSeam) -> &'static str {
    match seam {
        BoundedCancellationSeam::PreDispatch => "pre-dispatch",
        BoundedCancellationSeam::PostDispatch => "post-dispatch",
    }
}

const fn obligation(obligation: BoundedCancellationObligation) -> &'static str {
    match obligation {
        BoundedCancellationObligation::NotDispatched => "not-dispatched",
        BoundedCancellationObligation::SettlementContinues => "settlement-continues",
    }
}

const fn signal(signal: BoundedCancellationSignal) -> &'static str {
    match signal {
        BoundedCancellationSignal::RequestCancelled => "request-cancelled",
        BoundedCancellationSignal::ReconciledFromPhysicalTruth => "reconciled-from-physical-truth",
    }
}

const fn dispatch(dispatch: BoundedCancellationDispatch) -> &'static str {
    match dispatch {
        BoundedCancellationDispatch::DeniedConsumerCancelled => "denied-consumer-cancelled",
        BoundedCancellationDispatch::WriteCompleted => "write-completed",
    }
}

const fn recovery(recovery: BoundedCancellationRecovery) -> &'static str {
    match recovery {
        BoundedCancellationRecovery::NoSettlement => "no-settlement",
        BoundedCancellationRecovery::ContinueSettlement => "continue-settlement",
    }
}

const fn terminal(terminal: BoundedCancellationTerminal) -> &'static str {
    match terminal {
        BoundedCancellationTerminal::CancelledBeforeDispatch => "cancelled-before-dispatch",
        BoundedCancellationTerminal::ContinuedAfterConsumerCancellation => {
            "continued-after-consumer-cancellation"
        }
    }
}
