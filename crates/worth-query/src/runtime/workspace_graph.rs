use super::{
    WorthQueryBatchWriteReceipt, WorthQueryExistingTruthAssertionDenialKind,
    WorthQueryExistingTruthBindingDenialKind, WorthQueryGraphCompositionBuilder,
    WorthQueryGraphCompositionDenialKind, WorthQueryGraphCompositionDomainInvariantDenial,
    WorthQueryGraphCompositionInvariantPackContext,
    WorthQueryGraphCompositionInvariantPackViolation, WorthQueryRuntimeError,
    WorthQuerySymbolicTargetReferenceDenialKind, WorthQueryWorkspace,
};
use crate::runtime::mutation::graph_composition_error;

impl WorthQueryWorkspace {
    pub fn compose_graph(
        &mut self,
        declaration: impl FnOnce(
            &mut WorthQueryGraphCompositionBuilder,
        ) -> Result<(), WorthQueryRuntimeError>,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
        self.compose_graph_with_invariant_pack(declaration, |_context| Ok(()))
    }

    pub fn compose_graph_with_invariant_pack(
        &mut self,
        declaration: impl FnOnce(
            &mut WorthQueryGraphCompositionBuilder,
        ) -> Result<(), WorthQueryRuntimeError>,
        invariant_pack: impl FnOnce(
            &WorthQueryGraphCompositionInvariantPackContext<'_>,
        )
            -> Result<(), WorthQueryGraphCompositionInvariantPackViolation>,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
        self.compose_graph_with_invariant_gate(declaration, invariant_pack, |violation, context| {
            WorthQueryRuntimeError::GraphCompositionDomainInvariantDenied(
                WorthQueryGraphCompositionDomainInvariantDenial::from_violation(violation, context),
            )
        })
    }

    pub fn compose_graph_with_domain_invariant_denial(
        &mut self,
        declaration: impl FnOnce(
            &mut WorthQueryGraphCompositionBuilder,
        ) -> Result<(), WorthQueryRuntimeError>,
        invariant_denial: impl FnOnce(
            &WorthQueryGraphCompositionInvariantPackContext<'_>,
        )
            -> Result<(), WorthQueryGraphCompositionDomainInvariantDenial>,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
        self.compose_graph_with_invariant_gate(declaration, invariant_denial, |denial, _context| {
            WorthQueryRuntimeError::GraphCompositionDomainInvariantDenied(denial)
        })
    }

    fn compose_graph_with_invariant_gate<E>(
        &mut self,
        declaration: impl FnOnce(
            &mut WorthQueryGraphCompositionBuilder,
        ) -> Result<(), WorthQueryRuntimeError>,
        invariant_gate: impl FnOnce(
            &WorthQueryGraphCompositionInvariantPackContext<'_>,
        ) -> Result<(), E>,
        map_invariant_error: impl FnOnce(
            E,
            &WorthQueryGraphCompositionInvariantPackContext<'_>,
        ) -> WorthQueryRuntimeError,
    ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
        let mut builder = WorthQueryGraphCompositionBuilder::new();
        declaration(&mut builder)?;
        let (commands, breadth, program) = builder.finish()?;
        let invariant_context =
            WorthQueryGraphCompositionInvariantPackContext::new(&commands, &breadth, &program);
        invariant_gate(&invariant_context)
            .map_err(|error| map_invariant_error(error, &invariant_context))?;
        match self.runtime.write_graph_batch(commands, breadth, program) {
            Err(WorthQueryRuntimeError::ExistingTruthAssertionDenied(denial)) => {
                let kind = match denial.kind() {
                    WorthQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported => {
                        WorthQueryGraphCompositionDenialKind::ExistingTargetBackendVerificationUnsupported
                    }
                    WorthQueryExistingTruthAssertionDenialKind::ClearAssertionUnsupported => {
                        WorthQueryGraphCompositionDenialKind::ExistingTargetClearAssertionUnsupported
                    }
                    WorthQueryExistingTruthAssertionDenialKind::MissingAssertedAspect => {
                        WorthQueryGraphCompositionDenialKind::ExistingTargetMissingAssertedAspect
                    }
                    WorthQueryExistingTruthAssertionDenialKind::AssertedValueMismatch => {
                        WorthQueryGraphCompositionDenialKind::ExistingTargetAssertedValueMismatch
                    }
                };
                Err(graph_composition_error(
                    kind,
                    None,
                    denial.binding().target_collection_identity().cloned(),
                    denial.message().to_string(),
                ))
            }
            Err(WorthQueryRuntimeError::MutationBindingDenied(denial)) => {
                let kind = match denial.kind() {
                    WorthQueryExistingTruthBindingDenialKind::UnsupportedFamily => {
                        WorthQueryGraphCompositionDenialKind::ExistingTargetBindingUnsupported
                    }
                    WorthQueryExistingTruthBindingDenialKind::ResolvedTargetMissing => {
                        WorthQueryGraphCompositionDenialKind::ExistingTargetResolvedTargetMissing
                    }
                    WorthQueryExistingTruthBindingDenialKind::CollectionMismatch => {
                        WorthQueryGraphCompositionDenialKind::ExistingTargetCollectionMismatch
                    }
                };
                Err(graph_composition_error(
                    kind,
                    None,
                    denial.binding().target_collection_identity().cloned(),
                    denial.message().to_string(),
                ))
            }
            Err(WorthQueryRuntimeError::MutationTargetReferenceDenied(denial)) => {
                let kind = match denial.kind() {
                    WorthQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget => {
                        WorthQueryGraphCompositionDenialKind::UnresolvedSymbolicReference
                    }
                    WorthQuerySymbolicTargetReferenceDenialKind::CollectionMismatch => {
                        WorthQueryGraphCompositionDenialKind::SymbolicCollectionMismatch
                    }
                    WorthQuerySymbolicTargetReferenceDenialKind::RequiresBatchContext => {
                        return Err(WorthQueryRuntimeError::MutationTargetReferenceDenied(
                            denial,
                        ));
                    }
                    WorthQuerySymbolicTargetReferenceDenialKind::NonEntityReferenceTarget => {
                        return Err(WorthQueryRuntimeError::MutationTargetReferenceDenied(
                            denial,
                        ));
                    }
                };
                Err(graph_composition_error(
                    kind,
                    Some(denial.reference().symbol().to_string()),
                    denial.reference().target_collection_identity().cloned(),
                    denial.message().to_string(),
                ))
            }
            other => other,
        }
    }
}
