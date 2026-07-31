use std::sync::{Arc, Weak};

use worth_proof::TransitionOutcome;
use worth_signal::facade::{AsyncNodeAdmissionClass, AsyncNodeConditionBlockClass};
use worth_store_io_scheduler::QueueExecutionOutcome;
use worth_store_physical_backend::ArtifactTreeFailure;

use super::PhysicalWalRuntimeOwner;
use crate::physical_runtime::work::PhysicalWorkAdmissionAuthority;
use crate::physical_runtime::{
    instance::{
        PhysicalSchedulerAdmissionOwner, PhysicalStoreWorkRuntime, RecordSchedulerReservationDenial,
    },
    record_serving::{PreparedPhysicalMutation, RecordWorkAdmission},
    PhysicalExecutorCommand, PhysicalExecutorCommandDenial, PhysicalMutationWorkRequest,
    PhysicalSchedulerDemand, PhysicalSchedulerDenial, PhysicalWalAppendScope,
    PhysicalWalAppendSettlement, PhysicalWalReservationDenial, PhysicalWorkAdmission,
    PhysicalWorkExecution, PhysicalWorkPreEffectDenial, PhysicalWorkReadiness,
    PhysicalWorkScheduler, PhysicalWorkSettlementEvidence, WalAppendedPhysicalMutation,
    WalRangeReservedPhysicalMutation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWalAppendFailureCause {
    RuntimeReleased,
    SubmissionDenied(crate::physical_runtime::PhysicalWorkSubmissionDenial),
    SubmissionDeferred(crate::physical_runtime::PhysicalWorkSubmissionDeferred),
    SubmissionStale(crate::physical_runtime::PhysicalWorkSubmissionStale),
    SubmissionFailed(crate::physical_runtime::PhysicalWorkSubmissionFailure),
    PreEffect(PhysicalWorkPreEffectDenial),
    DependencyBlocked {
        class: AsyncNodeAdmissionClass,
        condition: Option<AsyncNodeConditionBlockClass>,
    },
    SchedulerReservationDenied(
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundAdmissionDenial,
    ),
    Scheduler(PhysicalSchedulerDenial),
    Command(PhysicalExecutorCommandDenial),
    MediaDeniedBeforeEffect(ArtifactTreeFailure),
}

pub enum PhysicalWalAppendOutcome {
    Appended(WalAppendedPhysicalMutation),
    ReservationDenied {
        prepared: PreparedPhysicalMutation,
        cause: PhysicalWalReservationDenial,
    },
    ProvenNoEffect {
        prepared: PreparedPhysicalMutation,
        cause: PhysicalWalAppendFailureCause,
    },
    Indeterminate {
        reserved: WalRangeReservedPhysicalMutation,
    },
}

#[derive(Clone)]
pub(in crate::physical_runtime) struct PhysicalWalAppendPort {
    runtime: Weak<PhysicalStoreWorkRuntime>,
    execution: PhysicalWorkExecution,
    physical: PhysicalWorkAdmissionAuthority,
    scheduler: PhysicalSchedulerAdmissionOwner,
    record: Arc<RecordWorkAdmission>,
    owner: PhysicalWalRuntimeOwner,
}

impl PhysicalWalAppendPort {
    pub(in crate::physical_runtime) fn new(
        runtime: &Arc<PhysicalStoreWorkRuntime>,
        generation: crate::physical_runtime::LifecycleGeneration,
        physical: PhysicalWorkAdmissionAuthority,
        scheduler: PhysicalSchedulerAdmissionOwner,
        record: Arc<RecordWorkAdmission>,
        owner: PhysicalWalRuntimeOwner,
    ) -> Self {
        Self {
            runtime: Arc::downgrade(runtime),
            execution: PhysicalStoreWorkRuntime::execution(runtime, generation),
            physical,
            scheduler,
            record,
            owner,
        }
    }

    pub(in crate::physical_runtime) fn append_prepared(
        &self,
        prepared: PreparedPhysicalMutation,
    ) -> PhysicalWalAppendOutcome {
        let prepared = match self.owner.admit_preparation(prepared) {
            Ok(prepared) => prepared,
            Err((prepared, cause)) => {
                return PhysicalWalAppendOutcome::ReservationDenied { prepared, cause }
            }
        };
        let reserved = match self.owner.reserve(prepared) {
            Ok(reserved) => reserved,
            Err((prepared, cause)) => {
                return PhysicalWalAppendOutcome::ReservationDenied { prepared, cause }
            }
        };
        match self.prepare_command(&reserved) {
            Ok(command) => self.execute(reserved, command),
            Err(cause) => {
                self.owner.release_before_effect();
                PhysicalWalAppendOutcome::ProvenNoEffect {
                    prepared: reserved.into_prepared_after_no_effect(),
                    cause,
                }
            }
        }
    }

    pub(in crate::physical_runtime) fn observation(&self) -> super::PhysicalWalObservation {
        self.owner.observation()
    }

    fn prepare_command(
        &self,
        reserved: &WalRangeReservedPhysicalMutation,
    ) -> Result<PhysicalExecutorCommand, PhysicalWalAppendFailureCause> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or(PhysicalWalAppendFailureCause::RuntimeReleased)?;
        let declaration = reserved.declaration();
        let artifact_range = declaration.artifact_range();
        let scope = PhysicalWalAppendScope::new(
            declaration.segment().get(),
            declaration.generation().get(),
            artifact_range.offset(),
            artifact_range.byte_count(),
        )
        .expect("reserved WAL declarations carry one valid append scope");
        let request = PhysicalMutationWorkRequest::wal_append(
            scope,
            self.record.wal_append_basis(),
            self.record.security(),
        )
        .map_err(PhysicalWalAppendFailureCause::SubmissionDenied)?;
        let receipt = match runtime
            .submission
            .mutation_submission()
            .submit(request)
            .into_raw()
        {
            TransitionOutcome::Success(receipt) => receipt,
            TransitionOutcome::Denied(denial) => {
                return Err(PhysicalWalAppendFailureCause::SubmissionDenied(denial))
            }
            TransitionOutcome::Deferred(deferred) => {
                return Err(PhysicalWalAppendFailureCause::SubmissionDeferred(deferred))
            }
            TransitionOutcome::Stale(stale) => {
                return Err(PhysicalWalAppendFailureCause::SubmissionStale(stale))
            }
            TransitionOutcome::RebindRequired(rebind) => match rebind {},
            TransitionOutcome::Failed(failure) => {
                return Err(PhysicalWalAppendFailureCause::SubmissionFailed(failure))
            }
        };
        let admitted = PhysicalWorkAdmission::admit(
            &runtime.submission,
            receipt,
            &self.physical,
            &runtime.health,
        )
        .map_err(PhysicalWalAppendFailureCause::PreEffect)?;
        let ready = match runtime
            .signal
            .request(admitted)
            .map_err(PhysicalWalAppendFailureCause::PreEffect)?
        {
            PhysicalWorkReadiness::Ready(ready) => ready,
            PhysicalWorkReadiness::Blocked(blocked) => {
                return Err(PhysicalWalAppendFailureCause::DependencyBlocked {
                    class: blocked.class(),
                    condition: blocked.condition(),
                })
            }
        };
        let (reservation, backend) = self
            .scheduler
            .wal_append(
                self.record.scheduler_security(),
                artifact_range.byte_count(),
            )
            .map_err(|denial: RecordSchedulerReservationDenial| match denial {
                RecordSchedulerReservationDenial::Admission(denial) => {
                    PhysicalWalAppendFailureCause::SchedulerReservationDenied(denial)
                }
            })?;
        let demand = PhysicalSchedulerDemand::foreground(ready, reservation, None)
            .map_err(PhysicalWalAppendFailureCause::Scheduler)?;
        PhysicalWorkAdmission::require_current(
            &runtime.submission,
            demand.intent(),
            &runtime.health,
        )
        .map_err(PhysicalWalAppendFailureCause::PreEffect)?;
        let policy =
            crate::physical_runtime::record_serving::admit_record_queue_policy(demand.queue_work());
        let work = PhysicalWorkScheduler::admit(demand, &backend, policy)
            .map_err(PhysicalWalAppendFailureCause::Scheduler)?;
        PhysicalExecutorCommand::wal_append(
            work,
            reserved.artifact().clone(),
            reserved.encoded_frame().to_vec().into_boxed_slice(),
        )
        .map_err(PhysicalWalAppendFailureCause::Command)
    }

    fn execute(
        &self,
        reserved: WalRangeReservedPhysicalMutation,
        command: PhysicalExecutorCommand,
    ) -> PhysicalWalAppendOutcome {
        let expected_work = command.identity();
        let (expected_range, expected_digest) = command
            .wal_append_completion_binding()
            .expect("the WAL port can execute only WAL append commands");
        let outcome = match self.execution.execute_physical_work(command) {
            Ok(outcome) => outcome,
            Err(cause) => {
                self.owner.release_before_effect();
                return PhysicalWalAppendOutcome::ProvenNoEffect {
                    prepared: reserved.into_prepared_after_no_effect(),
                    cause: PhysicalWalAppendFailureCause::PreEffect(cause),
                };
            }
        };
        let settled = outcome.into_settled();
        let work = settled.intent().identity();
        match settled.into_evidence() {
            PhysicalWorkSettlementEvidence::WalAppend {
                physical,
                scheduler: QueueExecutionOutcome::Executed(_),
            } => {
                let settlement = PhysicalWalAppendSettlement::completed(work, &physical);
                let Some(settlement) =
                    settlement.bind_completion(expected_work, expected_range, expected_digest)
                else {
                    self.owner.seal_for_inspection();
                    return PhysicalWalAppendOutcome::Indeterminate { reserved };
                };
                self.owner
                    .complete(reserved.resulting_frontier(), physical.range().byte_count());
                PhysicalWalAppendOutcome::Appended(WalAppendedPhysicalMutation::new(
                    reserved, settlement,
                ))
            }
            PhysicalWorkSettlementEvidence::NoEffect(evidence) => {
                self.owner.release_before_effect();
                PhysicalWalAppendOutcome::ProvenNoEffect {
                    prepared: reserved.into_prepared_after_no_effect(),
                    cause: PhysicalWalAppendFailureCause::MediaDeniedBeforeEffect(
                        evidence.failure(),
                    ),
                }
            }
            _ => {
                self.owner.seal_for_inspection();
                PhysicalWalAppendOutcome::Indeterminate { reserved }
            }
        }
    }
}
