use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::RecordFrameCoordinate;

use crate::physical_runtime::{
    instance::RecordSchedulerReservationDenial, PhysicalEffectIdentity, PhysicalExecutorCommand,
    PhysicalRetryCommand, PhysicalSchedulerDemand, PhysicalSignalSettlementOutcome,
    PhysicalWorkEffectFate, PhysicalWorkIdentity, PhysicalWorkRecoveryDisposition,
    PhysicalWorkScheduler, ReadyPhysicalWork, ResourceAdmittedPhysicalWork,
};

use super::{C6AdmittedDirtyFrame, C6PhysicalResidencyWork, C6PhysicalWorkHandoffFailure};
use crate::physical_runtime::record_serving::residency::scheduled_writeback::{
    PhysicalScheduledWriteback, PhysicalScheduledWritebackAdmissionDenial,
};

mod outcome;

pub use outcome::{
    C6PhysicalWritebackExecution, C6PhysicalWritebackTransitionFailure,
    C6RetryablePhysicalWriteback,
};

#[must_use = "dropping a reservation releases its bounded scheduler capacity"]
pub struct C6PhysicalWritebackReservation {
    identity: PhysicalWorkIdentity,
    reservation:
        worth_store_io_scheduler::foreground_reservation::PhysicalInstanceForegroundReservation,
    backend: worth_store_io_scheduler::IoSchedulerBackendCapabilityAdmission,
    coordinate: RecordFrameCoordinate,
}

#[must_use = "prepared writeback work must be admitted or deliberately dropped"]
pub struct C6PreparedPhysicalWriteback {
    identity: PhysicalWorkIdentity,
    demand: PhysicalSchedulerDemand,
    backend: worth_store_io_scheduler::IoSchedulerBackendCapabilityAdmission,
}

#[must_use = "admitted writeback work must be executed or deliberately dropped"]
pub struct C6AdmittedPhysicalWriteback {
    identity: PhysicalWorkIdentity,
    work: ResourceAdmittedPhysicalWork,
    dirty: C6AdmittedDirtyFrame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct C6PhysicalWorkSettlement {
    identity: PhysicalWorkIdentity,
    effect: Option<PhysicalEffectIdentity>,
    effect_fate: PhysicalWorkEffectFate,
    recovery: PhysicalWorkRecoveryDisposition,
    signal: PhysicalSignalSettlementOutcome,
}

impl C6PhysicalResidencyWork {
    pub fn reserve_writeback(
        &self,
        ready: &ReadyPhysicalWork,
        dirty: &C6AdmittedDirtyFrame,
    ) -> Result<C6PhysicalWritebackReservation, C6PhysicalWorkHandoffFailure> {
        let coordinate = self.require_writeback_intent(ready.intent())?;
        self.require_current(ready.intent())?;
        if dirty.handoff != self.identity
            || dirty.identity != ready.intent().identity()
            || dirty.coordinate != *coordinate
        {
            return Err(C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity);
        }
        let synchronization = matches!(
            ready.intent().durability(),
            crate::physical_runtime::PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(
                ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization
            )
        );
        let (reservation, backend) = self
            .scheduler
            .record_write(
                self.record.scheduler_security(),
                u64::from(coordinate.length()),
                synchronization,
                false,
            )
            .map_err(|failure| match failure {
                RecordSchedulerReservationDenial::Admission(denial) => {
                    C6PhysicalWorkHandoffFailure::SchedulerReservation(denial)
                }
            })?;
        Ok(C6PhysicalWritebackReservation {
            identity: ready.intent().identity(),
            reservation,
            backend,
            coordinate: *coordinate,
        })
    }

    pub fn prepare_writeback(
        &self,
        ready: ReadyPhysicalWork,
        reservation: C6PhysicalWritebackReservation,
        flush_epoch: u64,
        resource_shape: worth_store_contracts::QueueProducerResourceShape,
    ) -> Result<C6PreparedPhysicalWriteback, C6PhysicalWorkHandoffFailure> {
        let identity = ready.intent().identity();
        let coordinate = self.require_writeback_intent(ready.intent())?;
        if reservation.identity != identity || reservation.coordinate != *coordinate {
            return Err(C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity);
        }
        self.require_current(ready.intent())?;
        let grouping = worth_store_buffer_pool::BufferPoolQueueGroupingScope::new(
            reservation.reservation.receipt().security_scope_identity(),
        );
        let declaration = self
            .frame_ports
            .writeback_declaration(*coordinate, grouping, flush_epoch, resource_shape)
            .map_err(C6PhysicalWorkHandoffFailure::Residency)?;
        let secure_io = worth_store_io_scheduler::admit_secure_io_scope_for_scheduler(
            worth_store_io_scheduler::SecureIoPreservationRequest::new(
                worth_store_io_scheduler::SecureIoOperation::WriteBack,
                self.record.scheduler_security(),
                &reservation.backend,
            ),
        )
        .map_err(C6PhysicalWorkHandoffFailure::SecureIo)?;
        let demand = PhysicalSchedulerDemand::residency_writeback(
            ready,
            declaration,
            reservation.reservation,
            Some(secure_io),
        )
        .map_err(C6PhysicalWorkHandoffFailure::Scheduler)?;
        Ok(C6PreparedPhysicalWriteback {
            identity,
            demand,
            backend: reservation.backend,
        })
    }

    pub fn admit_writeback(
        &self,
        prepared: C6PreparedPhysicalWriteback,
        dirty: C6AdmittedDirtyFrame,
    ) -> Result<C6AdmittedPhysicalWriteback, C6PhysicalWritebackTransitionFailure> {
        self.admit_writeback_attempt(prepared, dirty, None)
    }

    pub fn admit_writeback_retry(
        &self,
        prepared: C6PreparedPhysicalWriteback,
        dirty: C6AdmittedDirtyFrame,
        retry: PhysicalRetryCommand,
    ) -> Result<C6AdmittedPhysicalWriteback, C6PhysicalWritebackTransitionFailure> {
        self.admit_writeback_attempt(prepared, dirty, Some(retry))
    }

    fn admit_writeback_attempt(
        &self,
        prepared: C6PreparedPhysicalWriteback,
        dirty: C6AdmittedDirtyFrame,
        retry: Option<PhysicalRetryCommand>,
    ) -> Result<C6AdmittedPhysicalWriteback, C6PhysicalWritebackTransitionFailure> {
        if let Err(cause) = self.require_current(prepared.demand.intent()) {
            return Err(C6PhysicalWritebackTransitionFailure::new(cause, dirty));
        }
        let coordinate = match self.require_writeback_intent(prepared.demand.intent()) {
            Ok(coordinate) => coordinate,
            Err(cause) => {
                return Err(C6PhysicalWritebackTransitionFailure::new(cause, dirty));
            }
        };
        if dirty.handoff != self.identity
            || dirty.identity != prepared.identity
            || dirty.coordinate != *coordinate
        {
            return Err(C6PhysicalWritebackTransitionFailure::new(
                C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity,
                dirty,
            ));
        }
        let policy =
            crate::physical_runtime::record_serving::record_queue_policy::admit_record_queue_policy(
                &prepared.demand.queue_work(),
            );
        let work = match PhysicalWorkScheduler::admit(prepared.demand, &prepared.backend, policy) {
            Ok(work) => work,
            Err(denial) => {
                return Err(C6PhysicalWritebackTransitionFailure::new(
                    C6PhysicalWorkHandoffFailure::Scheduler(denial),
                    dirty,
                ));
            }
        };
        if let Some(retry) = retry {
            if let Err(denial) = retry.admit_residency_retry(&work) {
                return Err(C6PhysicalWritebackTransitionFailure::new(
                    C6PhysicalWorkHandoffFailure::WritebackAdmission(
                        PhysicalScheduledWritebackAdmissionDenial::Retry(denial),
                    ),
                    dirty,
                ));
            }
        }
        Ok(C6AdmittedPhysicalWriteback {
            identity: prepared.identity,
            work,
            dirty,
        })
    }

    pub fn execute_writeback(
        &self,
        admitted: C6AdmittedPhysicalWriteback,
    ) -> Result<C6PhysicalWritebackExecution, C6PhysicalWritebackTransitionFailure> {
        let C6AdmittedPhysicalWriteback {
            identity,
            work,
            dirty,
        } = admitted;
        let coordinate = match self.validate_admitted_writeback(identity, &work, &dirty) {
            Ok(coordinate) => coordinate,
            Err(cause) => {
                return Err(C6PhysicalWritebackTransitionFailure::new(cause, dirty));
            }
        };
        let claim = match self.admit_writeback_claim(coordinate, &work) {
            Ok(claim) => claim,
            Err(cause) => {
                return Err(C6PhysicalWritebackTransitionFailure::new(cause, dirty));
            }
        };
        self.dispatch_writeback_effect(identity, work, dirty, claim)
    }

    fn validate_admitted_writeback(
        &self,
        identity: PhysicalWorkIdentity,
        work: &ResourceAdmittedPhysicalWork,
        dirty: &C6AdmittedDirtyFrame,
    ) -> Result<RecordFrameCoordinate, C6PhysicalWorkHandoffFailure> {
        if work.intent().identity() != identity || dirty.identity != identity {
            return Err(C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity);
        }
        let coordinate = *self.require_writeback_intent(work.intent())?;
        if dirty.handoff != self.identity || dirty.coordinate != coordinate {
            return Err(C6PhysicalWorkHandoffFailure::StaleOrForeignIdentity);
        }
        self.require_current(work.intent())?;
        Ok(coordinate)
    }

    fn admit_writeback_claim(
        &self,
        coordinate: RecordFrameCoordinate,
        work: &ResourceAdmittedPhysicalWork,
    ) -> Result<worth_store_buffer_pool::PhysicalWritebackClaim, C6PhysicalWorkHandoffFailure> {
        let claim = self
            .frame_ports
            .claim_writeback(coordinate)
            .map_err(C6PhysicalWorkHandoffFailure::Residency)?;
        PhysicalScheduledWriteback::validate(&claim, work.queue_plan())
            .map_err(C6PhysicalWorkHandoffFailure::WritebackAdmission)?;
        Ok(claim)
    }

    fn dispatch_writeback_effect(
        &self,
        identity: PhysicalWorkIdentity,
        work: ResourceAdmittedPhysicalWork,
        dirty: C6AdmittedDirtyFrame,
        claim: worth_store_buffer_pool::PhysicalWritebackClaim,
    ) -> Result<C6PhysicalWritebackExecution, C6PhysicalWritebackTransitionFailure> {
        let (dirty_binding, dirty_guard) = dirty.into_parts();
        let command = PhysicalExecutorCommand::residency_writeback(work, claim);
        let outcome = match self.execution.execute_physical_work(command) {
            Ok(outcome) => outcome,
            Err(denial) => {
                return Err(C6PhysicalWritebackTransitionFailure::new(
                    C6PhysicalWorkHandoffFailure::PreEffect(denial),
                    C6AdmittedDirtyFrame::from_parts(dirty_binding, dirty_guard),
                ));
            }
        };
        let signal = outcome.signal();
        if outcome.settled().retry_is_physically_safe() {
            return Ok(C6PhysicalWritebackExecution::Retryable(Box::new(
                C6RetryablePhysicalWriteback::new(
                    outcome.into_settled(),
                    signal,
                    C6AdmittedDirtyFrame::from_parts(dirty_binding, dirty_guard),
                ),
            )));
        }
        drop(dirty_guard);
        debug_assert_eq!(outcome.settled().intent().identity(), identity);
        Ok(C6PhysicalWritebackExecution::Settled(
            C6PhysicalWorkSettlement::from_execution(outcome),
        ))
    }
}

impl C6PhysicalWorkSettlement {
    fn from_execution(outcome: crate::physical_runtime::PhysicalWorkExecutionOutcome) -> Self {
        let signal = outcome.signal();
        let settled = outcome.into_settled();
        Self {
            identity: settled.intent().identity(),
            effect: settled.effect_identity(),
            effect_fate: settled.evidence().fate(),
            recovery: settled.recovery_disposition(),
            signal,
        }
    }

    pub const fn identity(self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn effect(self) -> Option<PhysicalEffectIdentity> {
        self.effect
    }

    pub const fn effect_fate(self) -> PhysicalWorkEffectFate {
        self.effect_fate
    }

    pub const fn recovery(self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }

    pub const fn signal(self) -> PhysicalSignalSettlementOutcome {
        self.signal
    }
}

impl C6PhysicalWritebackReservation {
    pub const fn identity(&self) -> PhysicalWorkIdentity {
        self.identity
    }
}

impl C6PreparedPhysicalWriteback {
    pub const fn identity(&self) -> PhysicalWorkIdentity {
        self.identity
    }
}

impl C6AdmittedPhysicalWriteback {
    pub const fn identity(&self) -> PhysicalWorkIdentity {
        self.identity
    }
}
