use super::AdmittedDirtyFrame;
use crate::physical_runtime::{
    PhysicalEffectIdentity, PhysicalSignalSettlementOutcome, PhysicalWorkEffectFate,
    PhysicalWorkIdentity, PhysicalWorkPreEffectDenial, PhysicalWorkRecoveryDisposition,
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
    ) -> Self {
        Self {
            identity,
            cause: PhysicalRecordWritebackFailureCause::Transition(cause),
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
}
