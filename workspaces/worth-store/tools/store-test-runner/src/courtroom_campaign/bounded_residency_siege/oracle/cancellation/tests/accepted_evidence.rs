use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    exact_route_fixture, BoundedCancellationCaseObservation, BoundedCancellationDispatch,
    BoundedCancellationObligation, BoundedCancellationObservation, BoundedCancellationRecovery,
    BoundedCancellationSeam, BoundedCancellationSignal, BoundedCancellationTerminal,
    BoundedResidencyMediaRole, BoundedResidencySignalAspectRole,
    BoundedResidencySignalBindingObservation, BoundedResidencySignalFamilySet,
    BoundedResidencyWorkEffectFate, BoundedResidencyWorkFamily,
    BoundedResidencyWorkReconciliationObservation, BoundedResidencyWorkRecordObservation,
    BoundedResidencyWorkRecovery, BoundedResidencyWorkTerminalFate,
};

pub(super) const STORE: [u8; 16] = [9; 16];
pub(super) const RUNTIME: u64 = 11;
pub(super) const GENERATION: u64 = 13;

pub(super) fn cancellation() -> BoundedCancellationObservation {
    BoundedCancellationObservation {
        pre_dispatch: BoundedCancellationCaseObservation {
            seam: BoundedCancellationSeam::PreDispatch,
            store: STORE,
            runtime: RUNTIME,
            generation: GENERATION,
            operation: 70,
            obligation: BoundedCancellationObligation::NotDispatched,
            signal: BoundedCancellationSignal::RequestCancelled,
            dispatch: BoundedCancellationDispatch::DeniedConsumerCancelled,
            recovery: BoundedCancellationRecovery::NoSettlement,
            terminal: BoundedCancellationTerminal::CancelledBeforeDispatch,
            media_before_cancellation: 0,
            cancellation_media_effects: 0,
            terminal_media_effects: 0,
            backend_receipt: None,
        },
        post_dispatch: BoundedCancellationCaseObservation {
            seam: BoundedCancellationSeam::PostDispatch,
            store: STORE,
            runtime: RUNTIME,
            generation: GENERATION,
            operation: 71,
            obligation: BoundedCancellationObligation::SettlementContinues,
            signal: BoundedCancellationSignal::ReconciledFromPhysicalTruth,
            dispatch: BoundedCancellationDispatch::WriteCompleted,
            recovery: BoundedCancellationRecovery::ContinueSettlement,
            terminal: BoundedCancellationTerminal::ContinuedAfterConsumerCancellation,
            media_before_cancellation: 1,
            cancellation_media_effects: 0,
            terminal_media_effects: 1,
            backend_receipt: Some(501),
        },
    }
}

pub(super) fn work() -> BoundedResidencyWorkReconciliationObservation {
    BoundedResidencyWorkReconciliationObservation {
        causal_overflow: 0,
        terminal_overflow: 0,
        safe_evidence_elided: 0,
        faults: 0,
        source_loads: 0,
        exact_writebacks: 1,
        identified_metadata_reads: 0,
        identified_positioned_reads: 0,
        identified_positioned_writes: 1,
        settled_terminal_fates: 0,
        continued_terminal_fates: 1,
        signal_bindings: Box::new([BoundedResidencySignalBindingObservation {
            digest: [7; 32],
            aspect_key: "store.physical.record.frame-writeback-basis".to_owned(),
            role: BoundedResidencySignalAspectRole::DependencyAndOutput,
            families: BoundedResidencySignalFamilySet {
                read_fault: false,
                exact_writeback: true,
                publication: false,
                lifecycle: false,
            },
            partition: None,
        }]),
        records: Box::new([BoundedResidencyWorkRecordObservation {
            store: STORE,
            runtime: RUNTIME,
            generation: GENERATION,
            operation: 71,
            family: BoundedResidencyWorkFamily::ArtifactRangeWrite,
            backend_operation: 501,
            backend_role: BoundedResidencyMediaRole::PositionedWrite,
            effect_fate: BoundedResidencyWorkEffectFate::WriteCompleted,
            recovery: BoundedResidencyWorkRecovery::ContinueSettlement,
            route: exact_route_fixture(71, BoundedResidencyWorkFamily::ArtifactRangeWrite, [7; 32]),
            terminal: BoundedResidencyWorkTerminalFate::ContinuedAfterConsumerCancellation,
        }]),
    }
}
