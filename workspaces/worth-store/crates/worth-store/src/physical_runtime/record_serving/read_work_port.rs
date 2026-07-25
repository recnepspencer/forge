use std::sync::{Arc, Weak};

use worth_proof::TransitionOutcome;
use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::ArtifactTreeFailure;
use worth_store_physical_format::RecordFrameCoordinate;

use crate::physical_runtime::{
    instance::{
        PhysicalSchedulerAdmissionOwner, PhysicalStoreWorkRuntime, RecordSchedulerReservationDenial,
    },
    work::PhysicalWorkAdmissionAuthority,
    PhysicalExecutorCommand, PhysicalExecutorCommandDenial, PhysicalMetadataReadWorkRequest,
    PhysicalReadSubmission, PhysicalReadWorkRequest, PhysicalSchedulerDemand,
    PhysicalSchedulerDenial, PhysicalWorkAdmission, PhysicalWorkExecution, PhysicalWorkIdentity,
    PhysicalWorkPreEffectDenial, PhysicalWorkReadiness, PhysicalWorkScope,
    PhysicalWorkSettlementEvidence,
};

use super::{RecordReadPartition, RecordWorkAdmission};

#[derive(Clone)]
pub(in crate::physical_runtime) struct CanonicalRecordReadPort {
    runtime: Weak<PhysicalStoreWorkRuntime>,
    execution: PhysicalWorkExecution,
    submission: PhysicalReadSubmission,
    physical: PhysicalWorkAdmissionAuthority,
    scheduler: PhysicalSchedulerAdmissionOwner,
    record: Arc<RecordWorkAdmission>,
}

pub(in crate::physical_runtime) struct PreparedCanonicalRecordRead {
    execution: PhysicalWorkExecution,
    command: PhysicalExecutorCommand,
    identity: PhysicalWorkIdentity,
}

struct PreparedCanonicalMetadataRead {
    execution: PhysicalWorkExecution,
    command: PhysicalExecutorCommand,
    identity: PhysicalWorkIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) enum CanonicalRecordReadFailure {
    RuntimeReleased,
    InvalidCoordinate,
    SubmissionRejected,
    PreEffect(PhysicalWorkPreEffectDenial),
    DependencyBlocked,
    SchedulerReservation(RecordSchedulerReservationDenial),
    Scheduler(PhysicalSchedulerDenial),
    Command(PhysicalExecutorCommandDenial),
    Backend(ArtifactTreeFailure),
    SchedulerSettlementRejected,
    SettlementMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct CanonicalMetadataReadFailure {
    failure: CanonicalRecordReadFailure,
    identity: Option<PhysicalWorkIdentity>,
}

impl CanonicalMetadataReadFailure {
    const fn before_work(failure: CanonicalRecordReadFailure) -> Self {
        Self {
            failure,
            identity: None,
        }
    }

    const fn during_work(
        failure: CanonicalRecordReadFailure,
        identity: PhysicalWorkIdentity,
    ) -> Self {
        Self {
            failure,
            identity: Some(identity),
        }
    }

    pub(in crate::physical_runtime) const fn failure(self) -> CanonicalRecordReadFailure {
        self.failure
    }

    pub(in crate::physical_runtime) const fn identity(self) -> Option<PhysicalWorkIdentity> {
        self.identity
    }
}

impl CanonicalRecordReadPort {
    pub(in crate::physical_runtime) fn new(
        runtime: &Arc<PhysicalStoreWorkRuntime>,
        generation: crate::physical_runtime::LifecycleGeneration,
        physical: PhysicalWorkAdmissionAuthority,
        scheduler: PhysicalSchedulerAdmissionOwner,
        record: Arc<RecordWorkAdmission>,
    ) -> Self {
        Self {
            runtime: Arc::downgrade(runtime),
            execution: PhysicalStoreWorkRuntime::execution(runtime, generation),
            submission: runtime.submission.read_submission(),
            physical,
            scheduler,
            record,
        }
    }

    pub(in crate::physical_runtime) fn prepare(
        &self,
        coordinate: RecordFrameCoordinate,
        partition: RecordReadPartition,
    ) -> Result<PreparedCanonicalRecordRead, CanonicalRecordReadFailure> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or(CanonicalRecordReadFailure::RuntimeReleased)?;
        let request = PhysicalReadWorkRequest::new(
            PhysicalWorkScope::one(coordinate),
            self.record.read_basis(partition),
            self.record.security(),
        )
        .expect("record coordinates, read basis, and security are admitted together");
        let receipt = match self.submission.submit(request).into_raw() {
            TransitionOutcome::Success(receipt) => receipt,
            _ => return Err(CanonicalRecordReadFailure::SubmissionRejected),
        };
        let admitted = PhysicalWorkAdmission::admit(
            &runtime.submission,
            receipt,
            &self.physical,
            &runtime.health,
        )
        .map_err(CanonicalRecordReadFailure::PreEffect)?;
        let ready = match runtime
            .signal
            .request(admitted)
            .map_err(CanonicalRecordReadFailure::PreEffect)?
        {
            PhysicalWorkReadiness::Ready(ready) => ready,
            PhysicalWorkReadiness::Blocked(_) => {
                return Err(CanonicalRecordReadFailure::DependencyBlocked)
            }
        };
        let (reservation, backend) = self
            .scheduler
            .record_read(
                self.record.scheduler_security(),
                u64::from(coordinate.length()),
            )
            .map_err(CanonicalRecordReadFailure::SchedulerReservation)?;
        let demand = PhysicalSchedulerDemand::foreground(ready, reservation, None)
            .map_err(CanonicalRecordReadFailure::Scheduler)?;
        PhysicalWorkAdmission::require_current(
            &runtime.submission,
            demand.intent(),
            &runtime.health,
        )
        .map_err(CanonicalRecordReadFailure::PreEffect)?;
        let policy = super::record_queue_policy::admit_record_queue_policy(&demand.queue_work());
        let work = crate::physical_runtime::PhysicalWorkScheduler::admit(demand, &backend, policy)
            .map_err(CanonicalRecordReadFailure::Scheduler)?;
        let identity = work.intent().identity();
        let command =
            PhysicalExecutorCommand::read(work).map_err(CanonicalRecordReadFailure::Command)?;
        drop(runtime);
        Ok(PreparedCanonicalRecordRead {
            execution: self.execution.clone(),
            command,
            identity,
        })
    }

    pub(in crate::physical_runtime) fn file_length(
        &self,
        artifact: worth_store_physical_format::RecordArtifactFile,
        partition: RecordReadPartition,
    ) -> Result<(u64, PhysicalWorkIdentity), CanonicalMetadataReadFailure> {
        self.prepare_metadata(artifact, partition)
            .map_err(CanonicalMetadataReadFailure::before_work)?
            .execute()
    }

    fn prepare_metadata(
        &self,
        artifact: worth_store_physical_format::RecordArtifactFile,
        partition: RecordReadPartition,
    ) -> Result<PreparedCanonicalMetadataRead, CanonicalRecordReadFailure> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or(CanonicalRecordReadFailure::RuntimeReleased)?;
        let request = PhysicalMetadataReadWorkRequest::new(
            artifact,
            self.record.read_basis(partition),
            self.record.security(),
        )
        .expect("record artifact metadata, read basis, and security are admitted together");
        let receipt = match self.submission.submit_metadata(request).into_raw() {
            TransitionOutcome::Success(receipt) => receipt,
            _ => return Err(CanonicalRecordReadFailure::SubmissionRejected),
        };
        let admitted = PhysicalWorkAdmission::admit(
            &runtime.submission,
            receipt,
            &self.physical,
            &runtime.health,
        )
        .map_err(CanonicalRecordReadFailure::PreEffect)?;
        let ready = match runtime
            .signal
            .request(admitted)
            .map_err(CanonicalRecordReadFailure::PreEffect)?
        {
            PhysicalWorkReadiness::Ready(ready) => ready,
            PhysicalWorkReadiness::Blocked(_) => {
                return Err(CanonicalRecordReadFailure::DependencyBlocked)
            }
        };
        let (reservation, backend) = self
            .scheduler
            .record_metadata(self.record.scheduler_security())
            .map_err(CanonicalRecordReadFailure::SchedulerReservation)?;
        let demand = PhysicalSchedulerDemand::foreground(ready, reservation, None)
            .map_err(CanonicalRecordReadFailure::Scheduler)?;
        PhysicalWorkAdmission::require_current(
            &runtime.submission,
            demand.intent(),
            &runtime.health,
        )
        .map_err(CanonicalRecordReadFailure::PreEffect)?;
        let policy = super::record_queue_policy::admit_record_queue_policy(&demand.queue_work());
        let work = crate::physical_runtime::PhysicalWorkScheduler::admit(demand, &backend, policy)
            .map_err(CanonicalRecordReadFailure::Scheduler)?;
        let identity = work.intent().identity();
        let command =
            PhysicalExecutorCommand::metadata(work).map_err(CanonicalRecordReadFailure::Command)?;
        Ok(PreparedCanonicalMetadataRead {
            execution: self.execution.clone(),
            command,
            identity,
        })
    }
}

impl PreparedCanonicalMetadataRead {
    fn execute(self) -> Result<(u64, PhysicalWorkIdentity), CanonicalMetadataReadFailure> {
        let identity = self.identity;
        let outcome = self
            .execution
            .execute_physical_work(self.command)
            .map_err(CanonicalRecordReadFailure::PreEffect)
            .map_err(|failure| CanonicalMetadataReadFailure::during_work(failure, identity))?;
        let result = match outcome.into_settled().into_evidence() {
            PhysicalWorkSettlementEvidence::Metadata {
                physical,
                scheduler: QueueExecutionOutcome::Executed(_),
            } => Ok((physical.file_length(), identity)),
            PhysicalWorkSettlementEvidence::Metadata { .. } => {
                Err(CanonicalRecordReadFailure::SchedulerSettlementRejected)
            }
            PhysicalWorkSettlementEvidence::NoEffect(evidence) => {
                Err(CanonicalRecordReadFailure::Backend(evidence.failure()))
            }
            _ => Err(CanonicalRecordReadFailure::SettlementMismatch),
        };
        result.map_err(|failure| CanonicalMetadataReadFailure::during_work(failure, identity))
    }
}

impl PreparedCanonicalRecordRead {
    pub(in crate::physical_runtime) const fn identity(&self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub(in crate::physical_runtime) fn execute(
        self,
    ) -> Result<Box<[u8]>, CanonicalRecordReadFailure> {
        let outcome = self
            .execution
            .execute_physical_work(self.command)
            .map_err(CanonicalRecordReadFailure::PreEffect)?;
        match outcome.into_settled().into_evidence() {
            PhysicalWorkSettlementEvidence::Read {
                bytes,
                scheduler: QueueExecutionOutcome::Executed(_),
                ..
            } => Ok(bytes),
            PhysicalWorkSettlementEvidence::Read { .. } => {
                Err(CanonicalRecordReadFailure::SchedulerSettlementRejected)
            }
            PhysicalWorkSettlementEvidence::NoEffect(evidence) => {
                Err(CanonicalRecordReadFailure::Backend(evidence.failure()))
            }
            _ => Err(CanonicalRecordReadFailure::SettlementMismatch),
        }
    }
}
