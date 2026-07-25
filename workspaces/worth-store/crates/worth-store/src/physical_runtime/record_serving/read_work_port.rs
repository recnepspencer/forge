use std::sync::{Arc, Weak};

use worth_proof::TransitionOutcome;
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
    PhysicalWorkSubmissionReceipt, ReadyPhysicalWork, ResourceAdmittedPhysicalWork,
};

use super::{
    PreparedCanonicalMetadataRead, PreparedCanonicalRecordRead, RecordReadPartition,
    RecordWorkAdmission,
};

#[derive(Clone)]
pub(in crate::physical_runtime) struct CanonicalRecordReadPort {
    runtime: Weak<PhysicalStoreWorkRuntime>,
    execution: PhysicalWorkExecution,
    submission: PhysicalReadSubmission,
    physical: PhysicalWorkAdmissionAuthority,
    scheduler: PhysicalSchedulerAdmissionOwner,
    record: Arc<RecordWorkAdmission>,
}

struct PreparedPhysicalCommand {
    command: PhysicalExecutorCommand,
    projection_failure: Option<crate::physical_runtime::PhysicalWorkAspectDelta>,
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
    Terminal(crate::physical_runtime::PhysicalWorkTerminalCause),
    SchedulerSettlementRejected,
    SettlementMismatch,
    ProjectionFailureUnavailable,
}

impl CanonicalRecordReadFailure {
    pub(in crate::physical_runtime::record_serving) const fn work_denial(
        self,
    ) -> Option<super::RecordReadWorkDenial> {
        let denial = match self {
            Self::RuntimeReleased => super::RecordReadWorkDenial::RuntimeReleased,
            Self::InvalidCoordinate => super::RecordReadWorkDenial::InvalidCoordinate,
            Self::SubmissionRejected => super::RecordReadWorkDenial::SubmissionRejected,
            Self::PreEffect(_) => super::RecordReadWorkDenial::AdmissionRejected,
            Self::DependencyBlocked => super::RecordReadWorkDenial::DependencyBlocked,
            Self::SchedulerReservation(_) => {
                super::RecordReadWorkDenial::SchedulerReservationRejected
            }
            Self::Scheduler(_) => super::RecordReadWorkDenial::SchedulerRejected,
            Self::Command(_) => super::RecordReadWorkDenial::CommandRejected,
            Self::Backend(_) | Self::Terminal(_) => return None,
            Self::SchedulerSettlementRejected => {
                super::RecordReadWorkDenial::SchedulerSettlementRejected
            }
            Self::SettlementMismatch => super::RecordReadWorkDenial::SettlementMismatch,
            Self::ProjectionFailureUnavailable => super::RecordReadWorkDenial::AdmissionRejected,
        };
        Some(denial)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct CanonicalRecordReadFailureEvidence {
    failure: CanonicalRecordReadFailure,
    identity: Option<PhysicalWorkIdentity>,
}

impl CanonicalRecordReadFailureEvidence {
    const fn before_work(failure: CanonicalRecordReadFailure) -> Self {
        Self {
            failure,
            identity: None,
        }
    }

    pub(super) const fn during_work(
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
    ) -> Result<PreparedCanonicalRecordRead, CanonicalRecordReadFailureEvidence> {
        let runtime = self.runtime.upgrade().ok_or_else(|| {
            CanonicalRecordReadFailureEvidence::before_work(
                CanonicalRecordReadFailure::RuntimeReleased,
            )
        })?;
        let request = PhysicalReadWorkRequest::new(
            PhysicalWorkScope::one(coordinate),
            self.record.read_basis(partition),
            self.record.security(),
        )
        .expect("record coordinates, read basis, and security are admitted together");
        let receipt = match self.submission.submit(request).into_raw() {
            TransitionOutcome::Success(receipt) => receipt,
            _ => {
                return Err(CanonicalRecordReadFailureEvidence::before_work(
                    CanonicalRecordReadFailure::SubmissionRejected,
                ))
            }
        };
        let (ready, identity) = admit_ready(&runtime, receipt, &self.physical)?;
        let prepared = self.prepare_range_command(&runtime, ready, identity, coordinate)?;
        let (command, projection_failure) = require_projection_failure(prepared, identity)?;
        drop(runtime);
        Ok(PreparedCanonicalRecordRead::new(
            self.execution.clone(),
            command,
            identity,
            self.execution.bind_projection_failure(projection_failure),
        ))
    }

    pub(in crate::physical_runtime) fn file_length(
        &self,
        artifact: worth_store_physical_format::RecordArtifactFile,
        partition: RecordReadPartition,
    ) -> Result<
        (
            u64,
            PhysicalWorkIdentity,
            crate::physical_runtime::instance::PhysicalProjectionFailureCapability,
        ),
        CanonicalRecordReadFailureEvidence,
    > {
        self.prepare_metadata(artifact, partition)?.execute()
    }

    fn prepare_metadata(
        &self,
        artifact: worth_store_physical_format::RecordArtifactFile,
        partition: RecordReadPartition,
    ) -> Result<PreparedCanonicalMetadataRead, CanonicalRecordReadFailureEvidence> {
        let runtime = self.runtime.upgrade().ok_or_else(|| {
            CanonicalRecordReadFailureEvidence::before_work(
                CanonicalRecordReadFailure::RuntimeReleased,
            )
        })?;
        let request = PhysicalMetadataReadWorkRequest::new(
            artifact,
            self.record.read_basis(partition),
            self.record.security(),
        )
        .expect("record artifact metadata, read basis, and security are admitted together");
        let receipt = match self.submission.submit_metadata(request).into_raw() {
            TransitionOutcome::Success(receipt) => receipt,
            _ => {
                return Err(CanonicalRecordReadFailureEvidence::before_work(
                    CanonicalRecordReadFailure::SubmissionRejected,
                ))
            }
        };
        let (ready, identity) = admit_ready(&runtime, receipt, &self.physical)?;
        let prepared = self.prepare_metadata_command(&runtime, ready, identity)?;
        let (command, projection_failure) = require_projection_failure(prepared, identity)?;
        Ok(PreparedCanonicalMetadataRead::new(
            self.execution.clone(),
            command,
            identity,
            self.execution.bind_projection_failure(projection_failure),
        ))
    }

    fn prepare_range_command(
        &self,
        runtime: &PhysicalStoreWorkRuntime,
        ready: ReadyPhysicalWork,
        identity: PhysicalWorkIdentity,
        coordinate: RecordFrameCoordinate,
    ) -> Result<PreparedPhysicalCommand, CanonicalRecordReadFailureEvidence> {
        let (reservation, backend) = self
            .scheduler
            .record_read(
                self.record.scheduler_security(),
                u64::from(coordinate.length()),
            )
            .map_err(CanonicalRecordReadFailure::SchedulerReservation)
            .map_err(|failure| {
                CanonicalRecordReadFailureEvidence::during_work(failure, identity)
            })?;
        prepare_command(runtime, ready, identity, reservation, backend, |work| {
            PhysicalExecutorCommand::read(work)
        })
    }

    fn prepare_metadata_command(
        &self,
        runtime: &PhysicalStoreWorkRuntime,
        ready: ReadyPhysicalWork,
        identity: PhysicalWorkIdentity,
    ) -> Result<PreparedPhysicalCommand, CanonicalRecordReadFailureEvidence> {
        let (reservation, backend) = self
            .scheduler
            .record_metadata(self.record.scheduler_security())
            .map_err(CanonicalRecordReadFailure::SchedulerReservation)
            .map_err(|failure| {
                CanonicalRecordReadFailureEvidence::during_work(failure, identity)
            })?;
        prepare_command(runtime, ready, identity, reservation, backend, |work| {
            PhysicalExecutorCommand::metadata(work)
        })
    }
}

fn require_projection_failure(
    prepared: PreparedPhysicalCommand,
    identity: PhysicalWorkIdentity,
) -> Result<
    (
        PhysicalExecutorCommand,
        crate::physical_runtime::PhysicalWorkAspectDelta,
    ),
    CanonicalRecordReadFailureEvidence,
> {
    let projection_failure = prepared.projection_failure.ok_or_else(|| {
        CanonicalRecordReadFailureEvidence::during_work(
            CanonicalRecordReadFailure::ProjectionFailureUnavailable,
            identity,
        )
    })?;
    Ok((prepared.command, projection_failure))
}

fn admit_ready(
    runtime: &PhysicalStoreWorkRuntime,
    receipt: PhysicalWorkSubmissionReceipt,
    physical: &PhysicalWorkAdmissionAuthority,
) -> Result<(ReadyPhysicalWork, PhysicalWorkIdentity), CanonicalRecordReadFailureEvidence> {
    let identity = receipt.identity();
    let admitted =
        PhysicalWorkAdmission::admit(&runtime.submission, receipt, physical, &runtime.health)
            .map_err(CanonicalRecordReadFailure::PreEffect)
            .map_err(|failure| {
                CanonicalRecordReadFailureEvidence::during_work(failure, identity)
            })?;
    match runtime
        .signal
        .request(admitted)
        .map_err(CanonicalRecordReadFailure::PreEffect)
        .map_err(|failure| CanonicalRecordReadFailureEvidence::during_work(failure, identity))?
    {
        PhysicalWorkReadiness::Ready(ready) => Ok((ready, identity)),
        PhysicalWorkReadiness::Blocked(_) => Err(CanonicalRecordReadFailureEvidence::during_work(
            CanonicalRecordReadFailure::DependencyBlocked,
            identity,
        )),
    }
}

fn prepare_command(
    runtime: &PhysicalStoreWorkRuntime,
    ready: ReadyPhysicalWork,
    identity: PhysicalWorkIdentity,
    reservation:
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation,
    backend: worth_store_io_scheduler::IoSchedulerBackendCapabilityAdmission,
    build: impl FnOnce(
        ResourceAdmittedPhysicalWork,
    ) -> Result<PhysicalExecutorCommand, PhysicalExecutorCommandDenial>,
) -> Result<PreparedPhysicalCommand, CanonicalRecordReadFailureEvidence> {
    let demand = PhysicalSchedulerDemand::foreground(ready, reservation, None)
        .map_err(CanonicalRecordReadFailure::Scheduler)
        .map_err(|failure| CanonicalRecordReadFailureEvidence::during_work(failure, identity))?;
    PhysicalWorkAdmission::require_current(&runtime.submission, demand.intent(), &runtime.health)
        .map_err(CanonicalRecordReadFailure::PreEffect)
        .map_err(|failure| CanonicalRecordReadFailureEvidence::during_work(failure, identity))?;
    let policy = super::record_queue_policy::admit_record_queue_policy(&demand.queue_work());
    let work = crate::physical_runtime::PhysicalWorkScheduler::admit(demand, &backend, policy)
        .map_err(CanonicalRecordReadFailure::Scheduler)
        .map_err(|failure| CanonicalRecordReadFailureEvidence::during_work(failure, identity))?;
    debug_assert_eq!(work.intent().identity(), identity);
    let projection_failure = runtime
        .signal
        .admit_projection_failure(&work)
        .map_err(|_| CanonicalRecordReadFailure::ProjectionFailureUnavailable)
        .map_err(|failure| CanonicalRecordReadFailureEvidence::during_work(failure, identity))?;
    let command = build(work)
        .map_err(CanonicalRecordReadFailure::Command)
        .map_err(|failure| CanonicalRecordReadFailureEvidence::during_work(failure, identity))?;
    Ok(PreparedPhysicalCommand {
        command,
        projection_failure,
    })
}
