use super::*;

impl RuntimeBridge {
    /// Specialist validation entrypoint for source declarations.
    pub fn validate_source_declaration(
        &self,
        declaration: SourceDeclaration,
    ) -> Result<ValidatedSourceDeclaration, BridgeDeliveryError> {
        let contract = self.admit_source(declaration)?;
        Ok(ValidatedSourceDeclaration::from_contract(&contract))
    }

    /// Admits a source declaration against the runtime source registry.
    pub fn admit_source(
        &self,
        declaration: SourceDeclaration,
    ) -> Result<AdmittedSourceContract, BridgeDeliveryError> {
        self.source_registry
            .contract_for_declaration(&declaration)
            .cloned()
            .ok_or_else(|| {
                let error = BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::SourceContractMismatch,
                    format!(
                        "Source declaration `{}` was not admitted by the runtime source registry.",
                        declaration.declaration_identity().as_str()
                    ),
                );
                self.record_source_failure(
                    &declaration,
                    SourceFailureClass::SourceContractMismatch,
                    error.kind(),
                    error.to_string(),
                );
                error
            })
    }

    pub(super) fn record_source_failure(
        &self,
        declaration: &SourceDeclaration,
        failure_class: SourceFailureClass,
        delivery_error_kind: BridgeDeliveryErrorKind,
        detail: impl Into<Arc<str>>,
    ) {
        self.diagnostics
            .record_source_failure(SourceFailureRecord::new(
                declaration.declaration_identity().clone(),
                declaration.selector(),
                declaration.required_capabilities(),
                failure_class,
                delivery_error_kind,
                detail,
            ));
    }
}
