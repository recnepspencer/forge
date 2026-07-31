use std::sync::{Arc, Weak};

use worth_proof::TransitionOutcome;

use super::{
    PhysicalWalBarrierDeclaration, PhysicalWalBarrierFailureCause, PhysicalWalBarrierOutcome,
    PhysicalWalBarrierSettlement, WalBarrierIndeterminatePhysicalMutation,
};
use crate::physical_runtime::work::PhysicalWorkAdmissionAuthority;
use crate::physical_runtime::{
    instance::{
        PhysicalSchedulerAdmissionOwner, PhysicalStoreWorkRuntime, RecordSchedulerReservationDenial,
    },
    record_serving::RecordWorkAdmission,
    PhysicalDurabilityObservation, PhysicalExecutorCommand, PhysicalMutationWorkRequest,
    PhysicalSchedulerDemand, PhysicalWorkAdmission, PhysicalWorkExecution, PhysicalWorkReadiness,
    PhysicalWorkScheduler, PhysicalWorkSettlementEvidence, WalAppendedPhysicalMutation,
    WalDurablePhysicalMutation,
};

#[derive(Clone)]
pub(in crate::physical_runtime) struct PhysicalWalBarrierPort {
    runtime: Weak<PhysicalStoreWorkRuntime>,
    execution: PhysicalWorkExecution,
    physical: PhysicalWorkAdmissionAuthority,
    scheduler: PhysicalSchedulerAdmissionOwner,
    record: Arc<RecordWorkAdmission>,
    durability: PhysicalDurabilityObservation,
}

impl PhysicalWalBarrierPort {
    pub(in crate::physical_runtime) fn new(
        runtime: &Arc<PhysicalStoreWorkRuntime>,
        generation: crate::physical_runtime::LifecycleGeneration,
        physical: PhysicalWorkAdmissionAuthority,
        scheduler: PhysicalSchedulerAdmissionOwner,
        record: Arc<RecordWorkAdmission>,
        durability: PhysicalDurabilityObservation,
    ) -> Self {
        Self {
            runtime: Arc::downgrade(runtime),
            execution: PhysicalStoreWorkRuntime::execution(runtime, generation),
            physical,
            scheduler,
            record,
            durability,
        }
    }

    pub(in crate::physical_runtime) fn synchronize_appended(
        &self,
        appended: WalAppendedPhysicalMutation,
    ) -> PhysicalWalBarrierOutcome {
        let Some(declaration) =
            PhysicalWalBarrierDeclaration::for_appended(&appended, self.durability)
        else {
            return PhysicalWalBarrierOutcome::BarrierNotStarted {
                appended,
                cause: PhysicalWalBarrierFailureCause::PolicyOrRuntimeMismatch,
            };
        };
        match self.prepare_command(&appended, &declaration) {
            Ok(command) => self.execute(appended, declaration, command),
            Err(cause) => PhysicalWalBarrierOutcome::BarrierNotStarted { appended, cause },
        }
    }

    fn prepare_command(
        &self,
        appended: &WalAppendedPhysicalMutation,
        declaration: &PhysicalWalBarrierDeclaration,
    ) -> Result<PhysicalExecutorCommand, PhysicalWalBarrierFailureCause> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or(PhysicalWalBarrierFailureCause::RuntimeReleased)?;
        let request = PhysicalMutationWorkRequest::wal_durability_barrier(
            declaration.scope(appended),
            self.record.wal_barrier_basis(),
            self.record.security(),
        )
        .map_err(PhysicalWalBarrierFailureCause::SubmissionDenied)?;
        let receipt = match runtime
            .submission
            .mutation_submission()
            .submit(request)
            .into_raw()
        {
            TransitionOutcome::Success(receipt) => receipt,
            TransitionOutcome::Denied(denial) => {
                return Err(PhysicalWalBarrierFailureCause::SubmissionDenied(denial))
            }
            TransitionOutcome::Deferred(deferred) => {
                return Err(PhysicalWalBarrierFailureCause::SubmissionDeferred(deferred))
            }
            TransitionOutcome::Stale(stale) => {
                return Err(PhysicalWalBarrierFailureCause::SubmissionStale(stale))
            }
            TransitionOutcome::RebindRequired(rebind) => match rebind {},
            TransitionOutcome::Failed(failure) => {
                return Err(PhysicalWalBarrierFailureCause::SubmissionFailed(failure))
            }
        };
        let admitted = PhysicalWorkAdmission::admit(
            &runtime.submission,
            receipt,
            &self.physical,
            &runtime.health,
        )
        .map_err(PhysicalWalBarrierFailureCause::PreEffect)?;
        let ready = match runtime
            .signal
            .request(admitted)
            .map_err(PhysicalWalBarrierFailureCause::PreEffect)?
        {
            PhysicalWorkReadiness::Ready(ready) => ready,
            PhysicalWorkReadiness::Blocked(blocked) => {
                return Err(PhysicalWalBarrierFailureCause::DependencyBlocked {
                    class: blocked.class(),
                    condition: blocked.condition(),
                })
            }
        };
        let (reservation, backend) = self
            .scheduler
            .wal_durability_barrier(self.record.scheduler_security())
            .map_err(|denial: RecordSchedulerReservationDenial| match denial {
                RecordSchedulerReservationDenial::Admission(denial) => {
                    PhysicalWalBarrierFailureCause::SchedulerReservationDenied(denial)
                }
            })?;
        let demand = PhysicalSchedulerDemand::foreground(ready, reservation, None)
            .map_err(PhysicalWalBarrierFailureCause::Scheduler)?;
        PhysicalWorkAdmission::require_current(
            &runtime.submission,
            demand.intent(),
            &runtime.health,
        )
        .map_err(PhysicalWalBarrierFailureCause::PreEffect)?;
        let policy =
            crate::physical_runtime::record_serving::admit_record_queue_policy(demand.queue_work());
        let work = PhysicalWorkScheduler::admit(demand, &backend, policy)
            .map_err(PhysicalWalBarrierFailureCause::Scheduler)?;
        PhysicalExecutorCommand::wal_barrier(
            work,
            declaration.artifact().clone(),
            declaration.binding_digest(),
        )
        .map_err(PhysicalWalBarrierFailureCause::Command)
    }

    fn execute(
        &self,
        appended: WalAppendedPhysicalMutation,
        declaration: PhysicalWalBarrierDeclaration,
        command: PhysicalExecutorCommand,
    ) -> PhysicalWalBarrierOutcome {
        let expected_work = command.identity();
        let Some((_expected_scope, expected_binding_digest)) =
            command.wal_barrier_completion_binding()
        else {
            return PhysicalWalBarrierOutcome::Indeterminate(
                WalBarrierIndeterminatePhysicalMutation::new(appended),
            );
        };
        let outcome = match self.execution.execute_physical_work(command) {
            Ok(outcome) => outcome,
            Err(cause) => {
                return PhysicalWalBarrierOutcome::BarrierNotStarted {
                    appended,
                    cause: PhysicalWalBarrierFailureCause::PreEffect(cause),
                }
            }
        };
        let settled = outcome.into_settled();
        let work = settled.intent().identity();
        match settled.into_evidence() {
            PhysicalWorkSettlementEvidence::WalBarrier {
                physical,
                scheduler,
            } => {
                let Some(settlement) = PhysicalWalBarrierSettlement::bind_completed(
                    work,
                    &physical,
                    &scheduler,
                    &declaration,
                    expected_work,
                    expected_binding_digest,
                ) else {
                    return PhysicalWalBarrierOutcome::Indeterminate(
                        WalBarrierIndeterminatePhysicalMutation::new(appended),
                    );
                };
                PhysicalWalBarrierOutcome::Durable(WalDurablePhysicalMutation::new(
                    appended, settlement,
                ))
            }
            PhysicalWorkSettlementEvidence::NoEffect(evidence) => {
                PhysicalWalBarrierOutcome::BarrierNotStarted {
                    appended,
                    cause: PhysicalWalBarrierFailureCause::MediaDeniedBeforeEffect(
                        evidence.failure(),
                    ),
                }
            }
            _ => PhysicalWalBarrierOutcome::Indeterminate(
                WalBarrierIndeterminatePhysicalMutation::new(appended),
            ),
        }
    }
}
