mod failure;
mod identity;
mod residency;

pub use failure::C6PhysicalWorkHandoffFailure;
pub use identity::C6PhysicalWorkHandoffIdentity;
pub use residency::{
    C6AdmittedDirtyFrame, C6AdmittedPhysicalWriteback, C6PhysicalFrameLease,
    C6PhysicalFrameReadFailure, C6PhysicalFrameWorkFailure, C6PhysicalResidencyWork,
    C6PhysicalWorkSettlement, C6PhysicalWritebackExecution, C6PhysicalWritebackReservation,
    C6PhysicalWritebackTransitionFailure, C6PreparedPhysicalWriteback,
    C6RetryablePhysicalWriteback,
};

use crate::physical_runtime::{
    instance::{PhysicalStoreInstanceParts, PhysicalWorkLifecycle},
    AdmittedPhysicalWork, BlockedPhysicalWork, PhysicalMutationSubmission, PhysicalReadSubmission,
    PhysicalWorkCancellationFailure, PhysicalWorkCancellationJoin, PhysicalWorkConsumerHandle,
    PhysicalWorkObservation, PhysicalWorkPreEffectDenial, PhysicalWorkReadiness,
    PhysicalWorkRecoveryLocator, PhysicalWorkRetryAdmission, PhysicalWorkRetryFailure,
    PhysicalWorkRetrySchedule, PhysicalWorkRetryScheduleOutcome, PhysicalWorkSubmissionReceipt,
    PhysicalWorkTimeoutJoin, ReadyPhysicalWork, SettledPhysicalWork,
};

use super::{PhysicalRecordReader, PhysicalRecordSubmission, RecordPublicationDirector};

/// Sealed future-integration surface for C.6 residency and frame lifecycle.
///
/// Construction is owned by the serving physical Store. The handoff exposes
/// existing record and physical-work capabilities without exporting raw
/// Signal state, scheduler plans, residency claims, executor commands, or
/// backend receipts.
pub struct C6PhysicalWorkHandoff {
    identity: C6PhysicalWorkHandoffIdentity,
    records: PhysicalRecordReader,
    record_submission: PhysicalRecordSubmission,
    work: PhysicalWorkLifecycle,
    residency: C6PhysicalResidencyWork,
}

impl C6PhysicalWorkHandoff {
    pub(in crate::physical_runtime) fn from_parts(
        parts: &PhysicalStoreInstanceParts,
        records: PhysicalRecordReader,
    ) -> Self {
        let identity = C6PhysicalWorkHandoffIdentity::new(
            records.store_identity(),
            parts.core.runtime_identity(),
            parts.core.lifecycle_generation(),
        );
        Self {
            identity,
            records,
            record_submission: RecordPublicationDirector::submission(&parts.publication),
            work: PhysicalWorkLifecycle::new(&parts.work_runtime, parts.work_admission),
            residency: C6PhysicalResidencyWork::from_parts(parts, identity),
        }
    }

    pub const fn identity(&self) -> C6PhysicalWorkHandoffIdentity {
        self.identity
    }

    pub const fn record_reads(&self) -> &PhysicalRecordReader {
        &self.records
    }

    pub fn record_submissions(&self) -> PhysicalRecordSubmission {
        self.record_submission.clone()
    }

    pub fn read_submission(&self) -> PhysicalReadSubmission {
        self.work.read_submission()
    }

    pub fn mutation_submission(&self) -> PhysicalMutationSubmission {
        self.work.mutation_submission()
    }

    pub fn observation(&self) -> PhysicalWorkObservation {
        self.work.observation()
    }

    pub fn admit_submitted_work(
        &self,
        receipt: PhysicalWorkSubmissionReceipt,
    ) -> Result<AdmittedPhysicalWork, PhysicalWorkPreEffectDenial> {
        self.work.admit(receipt)
    }

    pub fn request_work(
        &self,
        admitted: AdmittedPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        self.work.request(admitted)
    }

    pub fn revalidate_ready_work(
        &self,
        ready: ReadyPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        self.work.revalidate_ready(ready)
    }

    pub fn revalidate_blocked_work(
        &self,
        blocked: BlockedPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        self.work.revalidate_blocked(blocked)
    }

    pub fn cancel_work(
        &self,
        consumer: PhysicalWorkConsumerHandle,
    ) -> Result<PhysicalWorkCancellationJoin, PhysicalWorkCancellationFailure> {
        self.work.cancel(consumer)
    }

    pub fn schedule_work_retry(
        &self,
        settled: &SettledPhysicalWork,
    ) -> Result<PhysicalWorkRetryScheduleOutcome, PhysicalWorkRetryFailure> {
        self.work.schedule_retry(settled)
    }

    pub fn advance_signal_clock(
        &self,
        consumer: PhysicalWorkConsumerHandle,
        request: worth_signal::facade::ClockAdvanceRequest,
    ) -> Result<worth_signal::facade::ValidatedClockAdvance, PhysicalWorkRetryFailure> {
        self.work.advance_clock(consumer, request)
    }

    pub fn admit_work_retry(
        &self,
        retry: &PhysicalWorkRetrySchedule,
        settled: SettledPhysicalWork,
    ) -> Result<PhysicalWorkRetryAdmission, PhysicalWorkRetryFailure> {
        self.work.admit_retry(retry, settled)
    }

    pub fn timeout_work(
        &self,
        consumer: PhysicalWorkConsumerHandle,
    ) -> Result<PhysicalWorkTimeoutJoin, PhysicalWorkCancellationFailure> {
        self.work.timeout(consumer)
    }

    pub fn residency_work(&self) -> C6PhysicalResidencyWork {
        self.residency.clone()
    }

    pub fn recovery_obligations(
        &self,
    ) -> Result<Box<[PhysicalWorkRecoveryLocator]>, C6PhysicalWorkHandoffFailure> {
        self.work
            .recovery_obligations()
            .ok_or(C6PhysicalWorkHandoffFailure::RuntimeReleased)
    }

    pub fn recovery_evidence_damaged(&self) -> Result<bool, C6PhysicalWorkHandoffFailure> {
        self.work
            .recovery_evidence_damaged()
            .ok_or(C6PhysicalWorkHandoffFailure::RuntimeReleased)
    }
}
