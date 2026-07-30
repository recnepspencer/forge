use worth_store::physical_runtime::{
    AdmittedDirtyFrame, PhysicalEffectObligation, PhysicalResidencyCertification,
    PhysicalWorkDrainObservation, PhysicalWorkIdentity,
};
use worth_store_physical_backend::MediaOperationRole;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

mod post_dispatch;
mod pre_dispatch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CancellationSignalOutcome {
    RequestCancelled,
    ReconciledFromPhysicalTruth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CancellationDispatchOutcome {
    DeniedConsumerCancelled,
    WriteCompleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CancellationRecoveryOutcome {
    NoSettlement,
    ContinueSettlement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CancellationTerminalFate {
    CancelledBeforeDispatch,
    ContinuedAfterConsumerCancellation,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct CancellationCaseEvidence {
    pub(super) identity: PhysicalWorkIdentity,
    pub(super) obligation: PhysicalEffectObligation,
    pub(super) signal: CancellationSignalOutcome,
    pub(super) dispatch: CancellationDispatchOutcome,
    pub(super) recovery: CancellationRecoveryOutcome,
    pub(super) terminal: CancellationTerminalFate,
    pub(super) media_before_cancellation: u64,
    pub(super) cancellation_media_effects: u64,
    pub(super) terminal_media_effects: u64,
    pub(super) backend_receipt: Option<u64>,
}

pub(super) struct BoundedCancellationEvidence {
    pub(super) pre_dispatch: CancellationCaseEvidence,
    pub(super) post_dispatch: CancellationCaseEvidence,
}

pub(super) struct PendingBoundedCancellationEvidence {
    pre_dispatch: PendingCancellationCase,
    post_dispatch: PendingCancellationCase,
}

pub(super) struct PendingCancellationCase {
    pub(super) identity: PhysicalWorkIdentity,
    pub(super) obligation: PhysicalEffectObligation,
    pub(super) signal: CancellationSignalOutcome,
    pub(super) dispatch: CancellationDispatchOutcome,
    pub(super) recovery: CancellationRecoveryOutcome,
    pub(super) media_before_cancellation: u64,
    pub(super) cancellation_media_effects: u64,
    pub(super) terminal_media_effects: u64,
    pub(super) backend_receipt: Option<u64>,
}

pub(super) fn exercise(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
) -> Result<PendingBoundedCancellationEvidence, String> {
    let residency = serving.certification_physical_residency();
    let pre_dispatch = pre_dispatch::exercise(serving, &residency)?;
    let post_dispatch = post_dispatch::exercise(serving, &residency)?;
    Ok(PendingBoundedCancellationEvidence {
        pre_dispatch,
        post_dispatch,
    })
}

impl PendingBoundedCancellationEvidence {
    pub(super) fn finalize(
        self,
        drain: &PhysicalWorkDrainObservation,
    ) -> Result<BoundedCancellationEvidence, String> {
        let pre_dispatch = finalize_case(self.pre_dispatch, drain)?;
        let post_dispatch = finalize_case(self.post_dispatch, drain)?;
        if pre_dispatch.terminal != CancellationTerminalFate::CancelledBeforeDispatch
            || post_dispatch.terminal
                != CancellationTerminalFate::ContinuedAfterConsumerCancellation
        {
            return Err(format!(
                "bounded cancellation terminal fates crossed seams: \
                 pre={:?}, post={:?}",
                pre_dispatch.terminal, post_dispatch.terminal
            ));
        }
        Ok(BoundedCancellationEvidence {
            pre_dispatch,
            post_dispatch,
        })
    }
}

fn finalize_case(
    pending: PendingCancellationCase,
    drain: &PhysicalWorkDrainObservation,
) -> Result<CancellationCaseEvidence, String> {
    let cancelled = occurrences(drain.cancelled_before_dispatch(), pending.identity);
    let continued = occurrences(
        drain.continued_after_consumer_cancellation(),
        pending.identity,
    );
    let terminal = match (cancelled, continued) {
        (1, 0) => CancellationTerminalFate::CancelledBeforeDispatch,
        (0, 1) => CancellationTerminalFate::ContinuedAfterConsumerCancellation,
        _ => {
            return Err(format!(
                "bounded cancellation identity {:?} did not reach one terminal fate: \
                 cancelled={cancelled}, continued={continued}",
                pending.identity
            ))
        }
    };
    Ok(CancellationCaseEvidence {
        identity: pending.identity,
        obligation: pending.obligation,
        signal: pending.signal,
        dispatch: pending.dispatch,
        recovery: pending.recovery,
        terminal,
        media_before_cancellation: pending.media_before_cancellation,
        cancellation_media_effects: pending.cancellation_media_effects,
        terminal_media_effects: pending.terminal_media_effects,
        backend_receipt: pending.backend_receipt,
    })
}

fn occurrences(identities: &[PhysicalWorkIdentity], expected: PhysicalWorkIdentity) -> usize {
    identities
        .iter()
        .filter(|identity| **identity == expected)
        .count()
}

pub(super) fn same_bytes_dirty_frame(
    residency: &PhysicalResidencyCertification,
) -> Result<AdmittedDirtyFrame, String> {
    let coordinate = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8)
        .ok_or_else(|| "bounded cancellation coordinate was invalid".to_owned())?;
    let lease = residency
        .pin_exact(coordinate)
        .map_err(|failure| format!("bounded cancellation frame pin failed: {failure:?}"))?;
    residency
        .admit_dirty_frame(lease, |source, target| target.copy_from_slice(source))
        .map_err(|failure| format!("bounded cancellation dirty admission failed: {failure:?}"))
}

pub(super) fn positioned_writes(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
) -> u64 {
    serving
        .media_counters()
        .attempts_for(MediaOperationRole::PositionedWrite)
}
