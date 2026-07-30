use super::AdmittedDirtyFrame;
use worth_store_physical_format::RecordFrameCoordinate;

use crate::physical_runtime::{
    LifecycleGeneration, PhysicalEffectIdentity, PhysicalSignalSettlementOutcome,
    PhysicalWorkEffectFate, PhysicalWorkIdentity, PhysicalWorkPreEffectDenial,
    PhysicalWorkRecoveryDisposition,
};

#[derive(Debug)]
#[must_use = "failed writeback transition retains dirty residency ownership"]
pub struct PhysicalWritebackTransitionFailure {
    cause: PhysicalWritebackFailureCause,
    dirty: AdmittedDirtyFrame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWritebackFailureCause {
    RuntimeReleased,
    SubmissionRejected,
    DependencyBlocked,
    StaleOrForeignDirtyFrame,
    PreEffect(PhysicalWorkPreEffectDenial),
    SchedulerReservation(
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundAdmissionDenial,
    ),
    Scheduler(crate::physical_runtime::PhysicalSchedulerDenial),
    SecureIo(worth_store_io_scheduler::SecureIoPreservationDenial),
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
    WritebackAdmission(
        super::super::scheduled_writeback::PhysicalScheduledWritebackAdmissionDenial,
    ),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecordWritebackFailureCause {
    Transition(PhysicalWritebackFailureCause),
    RetryableNoEffect,
    InspectionRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordWritebackFailureEvidence {
    identity: Option<PhysicalWorkIdentity>,
    cause: PhysicalRecordWritebackFailureCause,
    frame_coordinate: Option<RecordFrameCoordinate>,
    effect: Option<PhysicalEffectIdentity>,
    effect_fate: PhysicalWorkEffectFate,
    recovery: Option<PhysicalWorkRecoveryDisposition>,
    signal: Option<PhysicalSignalSettlementOutcome>,
}

impl PhysicalWritebackTransitionFailure {
    pub(super) const fn new(
        cause: PhysicalWritebackFailureCause,
        dirty: AdmittedDirtyFrame,
    ) -> Self {
        Self { cause, dirty }
    }

    pub const fn cause(&self) -> PhysicalWritebackFailureCause {
        self.cause
    }

    pub fn into_dirty(self) -> AdmittedDirtyFrame {
        self.dirty
    }
}

impl PhysicalRecordWritebackFailureEvidence {
    pub(in crate::physical_runtime::record_serving) const fn transition(
        identity: Option<PhysicalWorkIdentity>,
        cause: PhysicalWritebackFailureCause,
        frame_coordinate: RecordFrameCoordinate,
    ) -> Self {
        Self {
            identity,
            cause: PhysicalRecordWritebackFailureCause::Transition(cause),
            frame_coordinate: Some(frame_coordinate),
            effect: None,
            effect_fate: PhysicalWorkEffectFate::ProvenNoEffect,
            recovery: None,
            signal: None,
        }
    }

    pub(in crate::physical_runtime::record_serving) const fn settled(
        cause: PhysicalRecordWritebackFailureCause,
        settlement: super::PhysicalWritebackSettlement,
    ) -> Self {
        Self {
            identity: Some(settlement.identity()),
            cause,
            frame_coordinate: None,
            effect: settlement.effect(),
            effect_fate: settlement.effect_fate(),
            recovery: Some(settlement.recovery()),
            signal: Some(settlement.signal()),
        }
    }

    pub const fn identity(self) -> Option<PhysicalWorkIdentity> {
        self.identity
    }

    pub const fn cause(self) -> PhysicalRecordWritebackFailureCause {
        self.cause
    }

    pub const fn frame_coordinate(self) -> Option<RecordFrameCoordinate> {
        self.frame_coordinate
    }

    pub const fn effect(self) -> Option<PhysicalEffectIdentity> {
        self.effect
    }

    pub const fn effect_fate(self) -> PhysicalWorkEffectFate {
        self.effect_fate
    }

    pub const fn recovery(self) -> Option<PhysicalWorkRecoveryDisposition> {
        self.recovery
    }

    pub const fn signal(self) -> Option<PhysicalSignalSettlementOutcome> {
        self.signal
    }

    pub(in crate::physical_runtime::record_serving) fn pressure(
        self,
        generation: LifecycleGeneration,
    ) -> Option<super::super::PhysicalRecordPressureEvidence> {
        let PhysicalRecordWritebackFailureCause::Transition(
            PhysicalWritebackFailureCause::Residency(
                worth_store_buffer_pool::PhysicalResidencyDenial::Pressure(pressure),
            ),
        ) = self.cause
        else {
            return None;
        };
        let basis = super::super::PhysicalRecordPressureBasis::for_store(pressure.store())
            .with_frame_coordinate(self.frame_coordinate?);
        super::super::PhysicalRecordPressureEvidence::from_failure(
            super::super::PhysicalRecordResidencyFailure::from(
                worth_store_buffer_pool::PhysicalResidencyDenial::Pressure(pressure),
            ),
            generation,
            basis,
        )
    }
}
