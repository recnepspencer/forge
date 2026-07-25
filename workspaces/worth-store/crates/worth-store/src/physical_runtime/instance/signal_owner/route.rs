use std::sync::{mpsc, Arc};

use worth_signal::facade::ResourceRequestHandle;
use worth_signal::facade::{
    ClockAdvanceRequest, ResourceRetryAdmissionReport, ResourceRetryScheduleReport,
    ResourceTimeoutReport, TemporalWakeId, ValidatedClockAdvance,
};
use worth_signal::facade::{RawCompletionEnvelope, ResourceCancellationReport};

use crate::physical_runtime::work::{
    AdmittedPhysicalWork, BlockedPhysicalWork, PhysicalSignalAspectBindingDigest,
    PhysicalWorkAspectDelta, PhysicalWorkIdentity, PhysicalWorkPreEffectDenial,
    PhysicalWorkReadiness, ReadyPhysicalWork,
};

use super::PhysicalSignalDeltaApplicationFailure;

mod command;
mod mailbox;

pub(in crate::physical_runtime::instance::signal_owner) use command::PhysicalSignalRouteCommand;
pub(in crate::physical_runtime::instance::signal_owner) use mailbox::{
    PhysicalSignalRouteMailbox, ROUTE_COMMAND_CAPACITY,
};

pub(super) struct PhysicalSignalRouteOwner {
    pub(super) route: PhysicalSignalAspectBindingDigest,
    pub(super) mailbox: Arc<PhysicalSignalRouteMailbox>,
}

impl PhysicalSignalRouteOwner {
    pub(super) fn apply_delta(
        &self,
        delta: PhysicalWorkAspectDelta,
    ) -> Result<(), PhysicalSignalDeltaApplicationFailure> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::Apply(delta, reply))
            .map_err(|_| PhysicalSignalDeltaApplicationFailure::OwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalSignalDeltaApplicationFailure::OwnerUnavailable)?
    }

    pub(super) fn request(
        &self,
        admitted: AdmittedPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::Request(admitted, reply))
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?
    }

    pub(super) fn begin_publication_dependency(
        &self,
        admitted: AdmittedPhysicalWork,
    ) -> Result<BlockedPhysicalWork, PhysicalWorkPreEffectDenial> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::BeginPublicationDependency(
                admitted, reply,
            ))
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?
    }

    pub(super) fn advance_publication_dependency(
        &self,
        blocked: BlockedPhysicalWork,
    ) -> Result<ReadyPhysicalWork, PhysicalWorkPreEffectDenial> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::AdvancePublicationDependency(
                blocked, reply,
            ))
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?
    }

    pub(super) fn revalidate_ready(
        &self,
        ready: ReadyPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::RevalidateReady(ready, reply))
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?
    }

    pub(super) fn revalidate_blocked(
        &self,
        admitted: AdmittedPhysicalWork,
        active: ResourceRequestHandle,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::RevalidateBlocked(
                admitted, active, reply,
            ))
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?
    }

    pub(super) fn record_settlement(
        &self,
        envelope: RawCompletionEnvelope,
    ) -> crate::physical_runtime::PhysicalSignalSettlementOutcome {
        let (reply, observed) = mpsc::sync_channel(0);
        if self
            .mailbox
            .enqueue(PhysicalSignalRouteCommand::RecordSettlement(
                envelope, reply,
            ))
            .is_err()
        {
            return crate::physical_runtime::PhysicalSignalSettlementOutcome::DerivedStateUnavailable;
        }
        observed.recv().unwrap_or(
            crate::physical_runtime::PhysicalSignalSettlementOutcome::DerivedStateUnavailable,
        )
    }

    pub(super) fn record_settlement_batch(
        &self,
        envelopes: Vec<RawCompletionEnvelope>,
    ) -> Box<[crate::physical_runtime::PhysicalSignalSettlementOutcome]> {
        let width = envelopes.len();
        let (reply, observed) = mpsc::sync_channel(0);
        if self
            .mailbox
            .enqueue(PhysicalSignalRouteCommand::RecordSettlementBatch(
                envelopes, reply,
            ))
            .is_err()
        {
            return unavailable_settlements(width);
        }
        observed
            .recv()
            .unwrap_or_else(|_| unavailable_settlements(width))
    }

    pub(super) fn cancel(
        &self,
        handle: ResourceRequestHandle,
    ) -> Result<ResourceCancellationReport, ()> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::Cancel(handle, reply))?;
        observed.recv().map_err(|_| ())?
    }

    pub(super) fn schedule_retry(
        &self,
        handle: ResourceRequestHandle,
    ) -> Result<ResourceRetryScheduleReport, ()> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::ScheduleRetry(handle, reply))?;
        observed.recv().map_err(|_| ())?
    }

    pub(super) fn admit_retry(
        &self,
        handle: ResourceRequestHandle,
        wake: TemporalWakeId,
    ) -> Result<ResourceRetryAdmissionReport, ()> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::AdmitRetry(handle, wake, reply))?;
        observed.recv().map_err(|_| ())?
    }

    pub(super) fn advance_clock(
        &self,
        request: ClockAdvanceRequest,
    ) -> Result<ValidatedClockAdvance, ()> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::AdvanceClock(request, reply))?;
        observed.recv().map_err(|_| ())?
    }

    pub(super) fn timeout(
        &self,
        handle: ResourceRequestHandle,
    ) -> Result<ResourceTimeoutReport, ()> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::Timeout(handle, reply))?;
        observed.recv().map_err(|_| ())?
    }

    pub(super) fn observation(&self) -> Result<super::graph::PhysicalSignalGraphObservation, ()> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::Observation(reply))?;
        observed.recv().map_err(|_| ())
    }

    pub(super) fn release(&self, identity: PhysicalWorkIdentity) {
        let _ = self
            .mailbox
            .enqueue(PhysicalSignalRouteCommand::Release(identity));
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) fn publication_dependencies(
        &self,
    ) -> Result<Vec<super::graph::PhysicalPublicationDependencyObservation>, ()> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.mailbox
            .enqueue(PhysicalSignalRouteCommand::PublicationDependencies(reply))
            .map_err(|_| ())?;
        observed.recv().map_err(|_| ())
    }
}

fn unavailable_settlements(
    width: usize,
) -> Box<[crate::physical_runtime::PhysicalSignalSettlementOutcome]> {
    vec![crate::physical_runtime::PhysicalSignalSettlementOutcome::DerivedStateUnavailable; width]
        .into_boxed_slice()
}
