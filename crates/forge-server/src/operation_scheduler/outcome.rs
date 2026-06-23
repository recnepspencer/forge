use crate::{ForgeServerOperationPlanProof, ForgeServerResponseEnvelope};

use super::{
    ForgeServerOperationExecutionSlot, ForgeServerOperationSchedulerCounters,
    ForgeServerScheduledMutationResult, ForgeServerSchedulerCancellationPosture,
    ForgeServerSchedulerFailurePosture,
};

#[derive(Debug)]
pub struct ForgeServerScheduledOperationOutcome {
    slot: ForgeServerOperationExecutionSlot,
    shared_read_basis_identity: Option<String>,
    execution_digest: Option<String>,
    mutation_result: Option<ForgeServerScheduledMutationResult>,
    response_envelope: Option<ForgeServerResponseEnvelope>,
    failure_posture: Option<ForgeServerSchedulerFailurePosture>,
    cancellation_posture: Option<ForgeServerSchedulerCancellationPosture>,
    scheduler_counters: ForgeServerOperationSchedulerCounters,
}

impl ForgeServerScheduledOperationOutcome {
    pub(crate) fn success(
        slot: ForgeServerOperationExecutionSlot,
        shared_read_basis_identity: String,
        execution_digest: String,
        response_envelope: ForgeServerResponseEnvelope,
        scheduler_counters: ForgeServerOperationSchedulerCounters,
    ) -> Self {
        Self {
            slot,
            shared_read_basis_identity: Some(shared_read_basis_identity),
            execution_digest: Some(execution_digest),
            mutation_result: None,
            response_envelope: Some(response_envelope),
            failure_posture: None,
            cancellation_posture: None,
            scheduler_counters,
        }
    }

    pub(crate) fn failed(
        slot: ForgeServerOperationExecutionSlot,
        failure_posture: ForgeServerSchedulerFailurePosture,
        scheduler_counters: ForgeServerOperationSchedulerCounters,
    ) -> Self {
        Self {
            slot,
            shared_read_basis_identity: None,
            execution_digest: None,
            mutation_result: None,
            response_envelope: None,
            failure_posture: Some(failure_posture),
            cancellation_posture: None,
            scheduler_counters,
        }
    }

    pub(crate) fn cancelled(
        slot: ForgeServerOperationExecutionSlot,
        cancellation_posture: ForgeServerSchedulerCancellationPosture,
        scheduler_counters: ForgeServerOperationSchedulerCounters,
    ) -> Self {
        Self {
            slot,
            shared_read_basis_identity: None,
            execution_digest: None,
            mutation_result: None,
            response_envelope: None,
            failure_posture: None,
            cancellation_posture: Some(cancellation_posture),
            scheduler_counters,
        }
    }

    pub(crate) fn success_without_counters(
        slot: ForgeServerOperationExecutionSlot,
        shared_read_basis_identity: String,
        execution_digest: String,
        response_envelope: ForgeServerResponseEnvelope,
    ) -> Self {
        Self::success(
            slot,
            shared_read_basis_identity,
            execution_digest,
            response_envelope,
            ForgeServerOperationSchedulerCounters::default(),
        )
    }

    pub(crate) fn mutation_success(
        slot: ForgeServerOperationExecutionSlot,
        mutation_result: ForgeServerScheduledMutationResult,
        response_envelope: ForgeServerResponseEnvelope,
        scheduler_counters: ForgeServerOperationSchedulerCounters,
    ) -> Self {
        Self {
            slot,
            shared_read_basis_identity: None,
            execution_digest: Some(mutation_result.result_digest().to_string()),
            mutation_result: Some(mutation_result),
            response_envelope: Some(response_envelope),
            failure_posture: None,
            cancellation_posture: None,
            scheduler_counters,
        }
    }

    pub(crate) fn mutation_success_without_counters(
        slot: ForgeServerOperationExecutionSlot,
        mutation_result: ForgeServerScheduledMutationResult,
        response_envelope: ForgeServerResponseEnvelope,
    ) -> Self {
        Self::mutation_success(
            slot,
            mutation_result,
            response_envelope,
            ForgeServerOperationSchedulerCounters::default(),
        )
    }

    pub(crate) fn failed_without_counters(
        slot: ForgeServerOperationExecutionSlot,
        failure_posture: ForgeServerSchedulerFailurePosture,
    ) -> Self {
        Self::failed(
            slot,
            failure_posture,
            ForgeServerOperationSchedulerCounters::default(),
        )
    }

    pub(crate) fn cancelled_without_counters(
        slot: ForgeServerOperationExecutionSlot,
        cancellation_posture: ForgeServerSchedulerCancellationPosture,
    ) -> Self {
        Self::cancelled(
            slot,
            cancellation_posture,
            ForgeServerOperationSchedulerCounters::default(),
        )
    }

    pub(crate) fn ordinal(&self) -> usize {
        self.slot.ordinal()
    }

    pub(crate) fn attach_scheduler_counters(
        &mut self,
        scheduler_counters: ForgeServerOperationSchedulerCounters,
    ) {
        self.scheduler_counters = scheduler_counters;
    }

    pub fn slot(&self) -> &ForgeServerOperationExecutionSlot {
        &self.slot
    }

    pub fn plan_proof(&self) -> &ForgeServerOperationPlanProof {
        self.slot.plan_proof()
    }

    pub fn shared_read_basis_identity(&self) -> Option<&str> {
        self.shared_read_basis_identity.as_deref()
    }

    pub fn response_envelope(&self) -> Option<&ForgeServerResponseEnvelope> {
        self.response_envelope.as_ref()
    }

    pub fn execution_digest(&self) -> Option<&str> {
        self.execution_digest.as_deref()
    }

    pub fn mutation_result(&self) -> Option<&ForgeServerScheduledMutationResult> {
        self.mutation_result.as_ref()
    }

    pub fn failure_posture(&self) -> Option<&ForgeServerSchedulerFailurePosture> {
        self.failure_posture.as_ref()
    }

    pub fn cancellation_posture(&self) -> Option<ForgeServerSchedulerCancellationPosture> {
        self.cancellation_posture
    }

    pub fn scheduler_counters(&self) -> &ForgeServerOperationSchedulerCounters {
        &self.scheduler_counters
    }
}
