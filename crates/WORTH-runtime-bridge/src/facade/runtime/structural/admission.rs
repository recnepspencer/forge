use super::*;

impl RuntimeBridge {
    /// Specialist validation entrypoint for structural identity declarations.
    pub fn validate_structural_declaration(
        &self,
        declaration: StructuralIdentityDeclaration,
    ) -> Result<ValidatedStructuralIdentityDeclaration, BridgeDeliveryError> {
        let contract = self.admit_structural_comparison(declaration)?;
        Ok(ValidatedStructuralIdentityDeclaration::from_contract(
            &contract,
        ))
    }

    /// Admits a structural comparison declaration against the runtime registry.
    pub fn admit_structural_comparison(
        &self,
        declaration: StructuralIdentityDeclaration,
    ) -> Result<AdmittedStructuralComparisonContract, BridgeDeliveryError> {
        self.structural_registry
            .contract_for_declaration(&declaration)
            .cloned()
            .ok_or_else(|| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralContractMismatch,
                    format!(
                        "Structural declaration `{}` was not admitted by the runtime structural registry.",
                        declaration.declaration_identity().as_str()
                    ),
                )
            })
    }
}
