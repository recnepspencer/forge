use super::*;

impl ForgeQueryRuntime {
    pub fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.write_batch_intent(commands).execute()
    }

    pub fn write_batch_with_policy_context(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
        policy_context: crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let review = self.review_authoritative_runtime_write_batch(commands)?;
        let handoff = self
            .resolve_reviewed_admitted_authoritative_write_batch_handoff_with_policy_context(
                review,
                &policy_context,
            )?;
        let binding = self.prepare_authoritative_mutation_batch_execution_binding(handoff);
        self.execute_authoritative_mutation_batch_execution_binding(binding)
    }

    pub fn write_batch_with_graph_obligation_execution_context(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
        execution_context: ForgeQueryGraphObligationExecutionContext,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let review = self.review_authoritative_runtime_write_batch(commands)?;
        let handoff = self
            .resolve_reviewed_admitted_authoritative_write_batch_handoff_with_graph_obligation_execution_context(
                review,
                execution_context,
            )?;
        let binding = self.prepare_authoritative_mutation_batch_execution_binding(handoff);
        self.execute_authoritative_mutation_batch_execution_binding(binding)
    }

    pub fn write_batch_with_graph_obligation_artifact_policy(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
        artifact_policy: ForgeQueryGraphObligationArtifactPolicy,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let execution_context = ForgeQueryGraphObligationExecutionContext::default()
            .with_artifact_policy(artifact_policy);
        self.write_batch_with_graph_obligation_execution_context(commands, execution_context)
    }

    pub(crate) fn write_graph_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
        graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
        graph_composition_program: ForgeQueryGraphCompositionProgram,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let review = self.review_authoritative_runtime_write_batch_with_graph_artifacts(
            commands,
            graph_composition_breadth,
            graph_composition_program,
        )?;
        let handoff = self.resolve_reviewed_admitted_authoritative_write_batch_handoff(review)?;
        let binding = self.prepare_authoritative_mutation_batch_execution_binding(handoff);
        self.execute_authoritative_mutation_batch_execution_binding(binding)
    }

    pub fn write_graph_batch_with_policy_context(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
        graph_composition_breadth: ForgeQueryGraphCompositionBreadth,
        graph_composition_program: ForgeQueryGraphCompositionProgram,
        policy_context: crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let review = self.review_authoritative_runtime_write_batch_with_graph_artifacts(
            commands,
            graph_composition_breadth,
            graph_composition_program,
        )?;
        let handoff = self
            .resolve_reviewed_admitted_authoritative_write_batch_handoff_with_policy_context(
                review,
                &policy_context,
            )?;
        let binding = self.prepare_authoritative_mutation_batch_execution_binding(handoff);
        self.execute_authoritative_mutation_batch_execution_binding(binding)
    }
}
