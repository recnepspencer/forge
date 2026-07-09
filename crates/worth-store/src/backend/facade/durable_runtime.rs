use crate::failure::StoreError;
use crate::recovery::{DurableRecoveryOutcome, DurableRecoveryPlan, DurableRetryResolution};
use crate::wal::{DurableMutationId, DurablePublicationPhase};
use worth_relational::facade::history::CommitId;

use super::{dispatch_mut, dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn admit_durable_mutation(
        &mut self,
        runtime_session_id: &str,
        operation_name: &str,
    ) -> Result<DurableMutationId, StoreError> {
        dispatch_mut!(self, |backend| backend
            .admit_durable_mutation(runtime_session_id, operation_name))
    }
    pub fn record_hosted_runtime_commit_result(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        envelope: worth_relational::facade::replay::CanonicalCommitEnvelope,
    ) -> Result<(), StoreError> {
        dispatch_mut!(self, |backend| backend.record_hosted_runtime_commit_result(
            runtime_session_id,
            durable_mutation_id,
            envelope,
        ))
    }
    pub fn record_publication_phase(
        &mut self,
        runtime_session_id: &str,
        durable_mutation_id: DurableMutationId,
        phase: DurablePublicationPhase,
        commit_id: Option<CommitId>,
    ) -> Result<(), StoreError> {
        dispatch_mut!(self, |backend| backend.record_publication_phase(
            runtime_session_id,
            durable_mutation_id,
            phase,
            commit_id
        ))
    }
    pub fn resolve_retry(
        &self,
        durable_mutation_id: DurableMutationId,
    ) -> Result<DurableRetryResolution, StoreError> {
        dispatch_ref!(self, |backend| backend.resolve_retry(durable_mutation_id))
    }
    pub fn recover_durable_runtime(
        &mut self,
        runtime_session_id: &str,
    ) -> Result<DurableRecoveryOutcome, StoreError> {
        dispatch_mut!(self, |backend| backend
            .recover_durable_runtime(runtime_session_id))
    }
    pub fn plan_durable_recovery(&self) -> DurableRecoveryPlan {
        dispatch_ref!(self, |backend| backend.plan_durable_recovery())
    }
}
