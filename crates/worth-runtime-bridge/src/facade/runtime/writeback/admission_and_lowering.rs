use super::*;

impl RuntimeBridge {
    /// Specialist validation entrypoint for writeback declarations.
    ///
    /// Everyday bridge flows should reach writeback through higher-level
    /// promotion or authority workflows, not by assembling declarations by hand.
    pub fn validate_writeback_declaration(
        &self,
        declaration: BridgeWritebackDeclaration,
    ) -> Result<ValidatedBridgeWritebackDeclaration, BridgeWritebackError> {
        ValidatedBridgeWritebackDeclaration::new(declaration)
    }

    /// Admits one writeback declaration against a lowered runtime policy.
    pub fn admit_writeback_declaration(
        &self,
        declaration: BridgeWritebackDeclaration,
        lowered_policy: &LoweredBridgeExecutionPolicy,
    ) -> Result<AdmittedBridgeWritebackContract, BridgeWritebackError> {
        let validated = self.validate_writeback_declaration(declaration)?;
        let authority_inputs = BridgeWritebackAuthorityInputs::new(
            self.policy.allow_replay_artifacts(),
            self.policy.diagnostics_tier(),
        );
        let contract =
            AdmittedBridgeWritebackContract::new(validated, authority_inputs, lowered_policy)?;
        self.diagnostics
            .record_writeback_admission(BridgeWritebackFamilyAdmissionRecord::new(&contract));
        Ok(contract)
    }

    /// Lowers a writeback effect from contract, causality, and effect identity inputs.
    pub fn lower_writeback_effect(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        causality: &BridgeWritebackNativeCausalityInputs,
        effect_identity: BridgeWritebackEffectIdentity,
        effect_intent: BridgeWritebackEffectIntent,
    ) -> BridgeDerivedWritebackEffect {
        let mapped_input = self.map_writeback_family_input(contract, causality, effect_intent);
        BridgeDerivedWritebackEffect::new(effect_identity, &mapped_input)
    }

    /// Produces and records the mapper envelope for a writeback family input.
    pub fn lower_writeback_mapper_envelope(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        causality: &BridgeWritebackNativeCausalityInputs,
        effect_intent: BridgeWritebackEffectIntent,
    ) -> BridgeWritebackMapperEnvelope {
        let envelope = BridgeWritebackMapperEnvelope::new(contract, causality, effect_intent);
        self.diagnostics
            .record_writeback_mapper_envelope(envelope.clone());
        envelope
    }

    /// Maps bridge-native writeback family inputs from mapper-envelope evidence.
    pub fn map_writeback_family_input(
        &self,
        contract: &AdmittedBridgeWritebackContract,
        causality: &BridgeWritebackNativeCausalityInputs,
        effect_intent: BridgeWritebackEffectIntent,
    ) -> BridgeMappedWritebackFamilyInput {
        let envelope = self.lower_writeback_mapper_envelope(contract, causality, effect_intent);
        let mapped_input = BridgeMappedWritebackFamilyInput::from_mapper_envelope(&envelope);
        self.diagnostics
            .record_writeback_mapped_family_input(mapped_input.clone());
        mapped_input
    }

    /// Derives feedback provenance for a lowered writeback effect.
    pub fn derive_writeback_feedback_provenance(
        &self,
        effect: &BridgeDerivedWritebackEffect,
    ) -> BridgeWritebackFeedbackProvenance {
        BridgeWritebackFeedbackProvenance::new(effect)
    }
}
