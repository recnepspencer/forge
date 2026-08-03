#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedCancellationSeam {
    PreDispatch,
    PostDispatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedCancellationObligation {
    NotDispatched,
    SettlementContinues,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedCancellationSignal {
    RequestCancelled,
    ReconciledFromPhysicalTruth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedCancellationDispatch {
    DeniedConsumerCancelled,
    WriteCompleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedCancellationRecovery {
    NoSettlement,
    ContinueSettlement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedCancellationTerminal {
    CancelledBeforeDispatch,
    ContinuedAfterConsumerCancellation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedCancellationCaseObservation
{
    pub(in crate::courtroom_campaign::bounded_residency_siege) seam: BoundedCancellationSeam,
    pub(in crate::courtroom_campaign::bounded_residency_siege) store: [u8; 16],
    pub(in crate::courtroom_campaign::bounded_residency_siege) runtime: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) generation: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) operation: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) obligation:
        BoundedCancellationObligation,
    pub(in crate::courtroom_campaign::bounded_residency_siege) signal: BoundedCancellationSignal,
    pub(in crate::courtroom_campaign::bounded_residency_siege) dispatch:
        BoundedCancellationDispatch,
    pub(in crate::courtroom_campaign::bounded_residency_siege) recovery:
        BoundedCancellationRecovery,
    pub(in crate::courtroom_campaign::bounded_residency_siege) terminal:
        BoundedCancellationTerminal,
    pub(in crate::courtroom_campaign::bounded_residency_siege) media_before_cancellation: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) cancellation_media_effects: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) terminal_media_effects: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) backend_receipt: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedCancellationObservation {
    pub(in crate::courtroom_campaign::bounded_residency_siege) pre_dispatch:
        BoundedCancellationCaseObservation,
    pub(in crate::courtroom_campaign::bounded_residency_siege) post_dispatch:
        BoundedCancellationCaseObservation,
}
