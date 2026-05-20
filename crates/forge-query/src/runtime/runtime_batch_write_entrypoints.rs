use super::*;

impl ForgeQueryRuntime {
    pub fn write_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.write_batch_intent(commands).execute()
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
}
