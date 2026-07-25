use worth_store_io_scheduler::QueueExecutionOutcome;

use crate::physical_runtime::{
    instance::PhysicalProjectionFailureCapability, PhysicalExecutorCommand, PhysicalWorkExecution,
    PhysicalWorkIdentity, PhysicalWorkSettlementEvidence,
};

use super::{read_work_port::CanonicalRecordReadFailureEvidence, CanonicalRecordReadFailure};

pub(in crate::physical_runtime) struct PreparedCanonicalRecordRead {
    execution: PhysicalWorkExecution,
    command: PhysicalExecutorCommand,
    identity: PhysicalWorkIdentity,
    projection_failure: PhysicalProjectionFailureCapability,
}

pub(super) struct PreparedCanonicalMetadataRead {
    execution: PhysicalWorkExecution,
    command: PhysicalExecutorCommand,
    identity: PhysicalWorkIdentity,
    projection_failure: PhysicalProjectionFailureCapability,
}

impl PreparedCanonicalRecordRead {
    pub(super) const fn new(
        execution: PhysicalWorkExecution,
        command: PhysicalExecutorCommand,
        identity: PhysicalWorkIdentity,
        projection_failure: PhysicalProjectionFailureCapability,
    ) -> Self {
        Self {
            execution,
            command,
            identity,
            projection_failure,
        }
    }

    pub(in crate::physical_runtime) const fn identity(&self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub(in crate::physical_runtime) fn execute(
        self,
    ) -> Result<(Box<[u8]>, PhysicalProjectionFailureCapability), CanonicalRecordReadFailure> {
        let outcome = self
            .execution
            .execute_physical_work(self.command)
            .map_err(CanonicalRecordReadFailure::PreEffect)?;
        match outcome.into_settled().into_evidence() {
            PhysicalWorkSettlementEvidence::Read {
                bytes,
                scheduler: QueueExecutionOutcome::Executed(_),
                ..
            } => Ok((bytes, self.projection_failure)),
            PhysicalWorkSettlementEvidence::Read { .. } => {
                Err(CanonicalRecordReadFailure::SchedulerSettlementRejected)
            }
            PhysicalWorkSettlementEvidence::NoEffect(evidence) => {
                Err(CanonicalRecordReadFailure::Backend(evidence.failure()))
            }
            PhysicalWorkSettlementEvidence::TerminalFailure(failure) => {
                Err(CanonicalRecordReadFailure::Terminal(failure.cause()))
            }
            _ => Err(CanonicalRecordReadFailure::SettlementMismatch),
        }
    }
}

impl PreparedCanonicalMetadataRead {
    pub(super) const fn new(
        execution: PhysicalWorkExecution,
        command: PhysicalExecutorCommand,
        identity: PhysicalWorkIdentity,
        projection_failure: PhysicalProjectionFailureCapability,
    ) -> Self {
        Self {
            execution,
            command,
            identity,
            projection_failure,
        }
    }

    pub(super) fn execute(
        self,
    ) -> Result<
        (
            u64,
            PhysicalWorkIdentity,
            PhysicalProjectionFailureCapability,
        ),
        CanonicalRecordReadFailureEvidence,
    > {
        let identity = self.identity;
        let outcome = self
            .execution
            .execute_physical_work(self.command)
            .map_err(CanonicalRecordReadFailure::PreEffect)
            .map_err(|failure| {
                CanonicalRecordReadFailureEvidence::during_work(failure, identity)
            })?;
        let result = match outcome.into_settled().into_evidence() {
            PhysicalWorkSettlementEvidence::Metadata {
                physical,
                scheduler: QueueExecutionOutcome::Executed(_),
            } => Ok((physical.file_length(), identity, self.projection_failure)),
            PhysicalWorkSettlementEvidence::Metadata { .. } => {
                Err(CanonicalRecordReadFailure::SchedulerSettlementRejected)
            }
            PhysicalWorkSettlementEvidence::NoEffect(evidence) => {
                Err(CanonicalRecordReadFailure::Backend(evidence.failure()))
            }
            PhysicalWorkSettlementEvidence::TerminalFailure(failure) => {
                Err(CanonicalRecordReadFailure::Terminal(failure.cause()))
            }
            _ => Err(CanonicalRecordReadFailure::SettlementMismatch),
        };
        result.map_err(|failure| CanonicalRecordReadFailureEvidence::during_work(failure, identity))
    }
}
