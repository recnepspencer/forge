use super::*;
use crate::intent_admission::dx::non_admitted_runtime_violation;
use crate::intent_admission::{
    ForgeQueryAuthoritativeMutationBatchExecutionBinding,
    ForgeQueryAuthoritativeMutationBatchExecutionHandoff, ForgeQueryIntentAdmissionDecision,
};
use crate::runtime::runtime_writes::ForgeQueryWriteAdmissionExecutionRecord;

impl ForgeQueryRuntime {
    pub fn write_batch_intent(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> crate::intent_admission::ForgeQueryRuntimeWriteBatchIntentAuthoring<'_> {
        crate::intent_admission::ForgeQueryRuntimeWriteBatchIntentAuthoring::new(self, commands)
    }

    pub(crate) fn review_authoritative_runtime_write_batch(
        &self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<
        crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
        ForgeQueryRuntimeError,
    > {
        self.review_authoritative_runtime_write_batch_with_graph_artifacts(
            commands,
            ForgeQueryGraphCompositionBreadth::empty(),
            ForgeQueryGraphCompositionProgram::empty(),
        )
    }

    pub(crate) fn review_authoritative_runtime_write_batch_with_graph_artifacts(
        &self,
        commands: Vec<ForgeQueryWriteCommand>,
        graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
        graph_composition_program: ForgeQueryGraphCompositionProgram,
    ) -> Result<
        crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
        ForgeQueryRuntimeError,
    > {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Write)?;
        let seed = crate::intent_admission::ForgeQueryAuthoritativeMutationBatchIntentSeed::new(
            commands,
            graph_composition_breadth,
            graph_composition_program,
        );
        let request = crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_write_batch_entrypoint(seed)
            .map_err(|violation| ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(violation.message())))?;
        Ok(
            crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
                request,
            ),
        )
    }

    pub(crate) fn resolve_reviewed_admitted_authoritative_write_batch_handoff(
        &self,
        review: crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
    ) -> Result<ForgeQueryAuthoritativeMutationBatchExecutionHandoff, ForgeQueryRuntimeError> {
        let handoff = admitted_authoritative_mutation_batch_handoff_from_review(review)?;
        let obligation_dispatch =
            self.authoritative_mutation_batch_obligation_dispatch(&handoff)?;
        Ok(handoff.with_obligation_dispatch(obligation_dispatch))
    }

    pub(crate) fn resolve_reviewed_admitted_authoritative_write_batch_handoff_with_graph_obligation_execution_context(
        &self,
        review: crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
        execution_context: ForgeQueryGraphObligationExecutionContext,
    ) -> Result<ForgeQueryAuthoritativeMutationBatchExecutionHandoff, ForgeQueryRuntimeError> {
        let handoff = admitted_authoritative_mutation_batch_handoff_from_review(review)?;
        let obligation_dispatch = self
            .authoritative_mutation_batch_obligation_dispatch_with_execution_context(
                &handoff,
                execution_context,
            )?;
        Ok(handoff.with_obligation_dispatch(obligation_dispatch))
    }

    pub(crate) fn resolve_reviewed_admitted_authoritative_write_batch_handoff_with_policy_context(
        &self,
        review: crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
        policy_context: &crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<ForgeQueryAuthoritativeMutationBatchExecutionHandoff, ForgeQueryRuntimeError> {
        let handoff = admitted_authoritative_mutation_batch_handoff_from_review(review)?;
        let obligation_dispatch = self
            .authoritative_mutation_batch_obligation_dispatch_with_policy_context(
                &handoff,
                policy_context,
            )?;
        Ok(handoff.with_obligation_dispatch(obligation_dispatch))
    }

    pub(crate) fn prepare_authoritative_mutation_batch_execution_binding(
        &self,
        handoff: ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    ) -> ForgeQueryAuthoritativeMutationBatchExecutionBinding {
        ForgeQueryAuthoritativeMutationBatchExecutionBinding::from_handoff(handoff)
    }

    pub(crate) fn execute_authoritative_mutation_batch_execution_binding(
        &mut self,
        binding: ForgeQueryAuthoritativeMutationBatchExecutionBinding,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Write)?;
        let handoff = binding.handoff().clone();
        let shared_admission = ForgeQueryWriteAdmissionExecutionRecord {
            family: binding.family(),
            entrypoint: binding.entrypoint(),
            execution_seam: binding.execution_seam(),
            request_detail: format!("batch-write:{}", handoff.commands().len()),
            request_digest: handoff.request_digest().to_string(),
            eligibility_trace: handoff.eligibility_trace().clone(),
            decision_digest: handoff.decision_digest().to_string(),
            handoff_digest: handoff.handoff_digest().to_string(),
            binding_digest: binding.binding_digest().to_string(),
            obligation_dispatch: binding.obligation_dispatch().cloned(),
        };
        self.execute_authoritative_write_batch_direct(
            handoff.commands().to_vec(),
            handoff.graph_composition_breadth().clone(),
            handoff.graph_composition_program().clone(),
            Some(shared_admission),
        )
    }
}

fn admitted_authoritative_mutation_batch_handoff_from_review(
    review: crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData,
) -> Result<ForgeQueryAuthoritativeMutationBatchExecutionHandoff, ForgeQueryRuntimeError> {
    match review.decision().clone() {
        ForgeQueryIntentAdmissionDecision::Admitted(
            crate::intent_admission::ForgeQueryAdmittedIntentPlan::AuthoritativeMutationBatch(plan),
        ) => Ok(ForgeQueryAuthoritativeMutationBatchExecutionHandoff::from_plan(plan)),
        ForgeQueryIntentAdmissionDecision::Admitted(_)
        | ForgeQueryIntentAdmissionDecision::Advisory(_)
        | ForgeQueryIntentAdmissionDecision::Violation(_) => {
            let violation = non_admitted_runtime_violation(&review);
            Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(violation.message()),
            ))
        }
    }
}
