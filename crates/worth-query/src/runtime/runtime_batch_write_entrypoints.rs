use super::*;

impl WorthQueryRuntime {
    pub fn write_batch(
        &mut self,
        commands: Vec<WorthQueryWriteCommand>,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
        self.write_batch_intent(commands).execute()
    }

    pub fn write_batch_with_policy_context(
        &mut self,
        commands: Vec<WorthQueryWriteCommand>,
        policy_context: crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
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
        commands: Vec<WorthQueryWriteCommand>,
        execution_context: WorthQueryGraphObligationExecutionContext,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
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
        commands: Vec<WorthQueryWriteCommand>,
        artifact_policy: WorthQueryGraphObligationArtifactPolicy,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
        let execution_context = WorthQueryGraphObligationExecutionContext::default()
            .with_artifact_policy(artifact_policy);
        self.write_batch_with_graph_obligation_execution_context(commands, execution_context)
    }

    pub(crate) fn write_graph_batch(
        &mut self,
        commands: Vec<WorthQueryWriteCommand>,
        graph_composition_breadth: WorthQueryGraphCompositionBreadth,
        graph_composition_program: WorthQueryGraphCompositionProgram,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
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
        commands: Vec<WorthQueryWriteCommand>,
        graph_composition_breadth: WorthQueryGraphCompositionBreadth,
        graph_composition_program: WorthQueryGraphCompositionProgram,
        policy_context: crate::policy_basis::AdmittedPolicyTenantContext,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
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
