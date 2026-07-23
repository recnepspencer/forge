use super::*;

impl RuntimeBridge {
    /// Executes one mutation writeback and returns Bridge-owned authority only
    /// when the exact effect, causality, execution record, and commit outcome
    /// form one successful artifact chain.
    pub fn execute_writeback_mutation_authority(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
        causality: &BridgeWritebackNativeCausalityInputs,
    ) -> Result<BridgeMutationAuthorityBundle, BridgeWritebackError> {
        let feedback = BridgeWritebackFeedbackProvenance::new(effect);
        let execution = self.execute_writeback_authority_artifacts_with_feedback_context(
            contract,
            effect,
            idempotence,
            None,
        )?;
        BridgeMutationAuthorityBundle::from_successful_writeback_artifacts(
            crate::writeback::SuccessfulWritebackArtifactChain {
                causality,
                effect,
                feedback: &feedback,
                execution_record: execution.execution_record(),
                outcome: execution.outcome(),
            },
        )
        .map_err(|error| {
            BridgeWritebackError::new(
                BridgeWritebackErrorKind::InvariantRejected,
                format!("bridge mutation authority artifact chain rejected: {error}"),
            )
        })
    }
}
