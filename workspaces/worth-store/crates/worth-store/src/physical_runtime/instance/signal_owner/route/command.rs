use std::sync::mpsc;

use worth_signal::facade::{
    ClockAdvanceRequest, RawCompletionEnvelope, ResourceCancellationReport, ResourceRequestHandle,
    ResourceRetryAdmissionReport, ResourceRetryScheduleReport, ResourceTimeoutReport,
    TemporalWakeId, ValidatedClockAdvance,
};

use crate::physical_runtime::work::{
    AdmittedPhysicalWork, PhysicalWorkAspectDelta, PhysicalWorkIdentity,
    PhysicalWorkPreEffectDenial, PhysicalWorkReadiness, ReadyPhysicalWork,
};

use super::super::PhysicalSignalDeltaApplicationFailure;

#[allow(
    clippy::large_enum_variant,
    reason = "bounded route mailboxes carry owned typestate inline to avoid a heap allocation per physical request"
)]
pub(in crate::physical_runtime::instance::signal_owner) enum PhysicalSignalRouteCommand {
    Apply(
        PhysicalWorkAspectDelta,
        mpsc::SyncSender<Result<(), PhysicalSignalDeltaApplicationFailure>>,
    ),
    Request(
        AdmittedPhysicalWork,
        mpsc::SyncSender<Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial>>,
    ),
    RevalidateReady(
        ReadyPhysicalWork,
        mpsc::SyncSender<Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial>>,
    ),
    RevalidateBlocked(
        AdmittedPhysicalWork,
        ResourceRequestHandle,
        mpsc::SyncSender<Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial>>,
    ),
    RecordSettlement(
        RawCompletionEnvelope,
        mpsc::SyncSender<crate::physical_runtime::PhysicalSignalSettlementOutcome>,
    ),
    RecordSettlementBatch(
        Vec<RawCompletionEnvelope>,
        mpsc::SyncSender<Box<[crate::physical_runtime::PhysicalSignalSettlementOutcome]>>,
    ),
    Cancel(
        ResourceRequestHandle,
        mpsc::SyncSender<Result<ResourceCancellationReport, ()>>,
    ),
    ScheduleRetry(
        ResourceRequestHandle,
        mpsc::SyncSender<Result<ResourceRetryScheduleReport, ()>>,
    ),
    AdmitRetry(
        ResourceRequestHandle,
        TemporalWakeId,
        mpsc::SyncSender<Result<ResourceRetryAdmissionReport, ()>>,
    ),
    AdvanceClock(
        ClockAdvanceRequest,
        mpsc::SyncSender<Result<ValidatedClockAdvance, ()>>,
    ),
    Timeout(
        ResourceRequestHandle,
        mpsc::SyncSender<Result<ResourceTimeoutReport, ()>>,
    ),
    Release(PhysicalWorkIdentity),
    Observation(mpsc::SyncSender<super::super::graph::PhysicalSignalGraphObservation>),
}
