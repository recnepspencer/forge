use worth_store::physical_runtime::PhysicalEffectObligation;

use super::super::cancellation::{
    BoundedCancellationEvidence, CancellationCaseEvidence, CancellationDispatchOutcome,
    CancellationRecoveryOutcome, CancellationSignalOutcome, CancellationTerminalFate,
};

pub(super) fn emit(evidence: &BoundedCancellationEvidence) {
    emit_case("pre-dispatch", &evidence.pre_dispatch);
    emit_case("post-dispatch", &evidence.post_dispatch);
}

fn emit_case(seam: &str, evidence: &CancellationCaseEvidence) {
    let identity = evidence.identity;
    println!(
        "BOUNDED_RESIDENCY_CANCELLATION {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        seam,
        store_hex(&identity.store().bytes()),
        identity.runtime().get(),
        identity.generation().lifecycle().get(),
        identity.operation().get(),
        obligation(evidence.obligation),
        signal(evidence.signal),
        dispatch(evidence.dispatch),
        recovery(evidence.recovery),
        terminal(evidence.terminal),
        evidence.media_before_cancellation,
        evidence.cancellation_media_effects,
        evidence.terminal_media_effects,
        evidence.backend_receipt.unwrap_or(0),
    );
}

const fn obligation(obligation: PhysicalEffectObligation) -> &'static str {
    match obligation {
        PhysicalEffectObligation::NotDispatched => "not-dispatched",
        PhysicalEffectObligation::SettlementContinues => "settlement-continues",
    }
}

const fn signal(signal: CancellationSignalOutcome) -> &'static str {
    match signal {
        CancellationSignalOutcome::RequestCancelled => "request-cancelled",
        CancellationSignalOutcome::ReconciledFromPhysicalTruth => "reconciled-from-physical-truth",
    }
}

const fn dispatch(dispatch: CancellationDispatchOutcome) -> &'static str {
    match dispatch {
        CancellationDispatchOutcome::DeniedConsumerCancelled => "denied-consumer-cancelled",
        CancellationDispatchOutcome::WriteCompleted => "write-completed",
    }
}

const fn recovery(recovery: CancellationRecoveryOutcome) -> &'static str {
    match recovery {
        CancellationRecoveryOutcome::NoSettlement => "no-settlement",
        CancellationRecoveryOutcome::ContinueSettlement => "continue-settlement",
    }
}

const fn terminal(terminal: CancellationTerminalFate) -> &'static str {
    match terminal {
        CancellationTerminalFate::CancelledBeforeDispatch => "cancelled-before-dispatch",
        CancellationTerminalFate::ContinuedAfterConsumerCancellation => {
            "continued-after-consumer-cancellation"
        }
    }
}

fn store_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
