use super::{
    ForgeQueryBatchWriteReceipt, ForgeQueryExistingTruthAssertionDenialKind,
    ForgeQueryExistingTruthBindingDenialKind, ForgeQueryGraphCompositionBuilder,
    ForgeQueryGraphCompositionDenialKind, ForgeQueryGraphCompositionDomainInvariantDenial,
    ForgeQueryGraphCompositionInvariantPackContext,
    ForgeQueryGraphCompositionInvariantPackViolation, ForgeQueryRuntimeError,
    ForgeQuerySymbolicTargetReferenceDenialKind, ForgeQueryWorkspace,
};
use crate::runtime::mutation::graph_composition_error;

impl ForgeQueryWorkspace {
    pub fn compose_graph(
        &mut self,
        declaration: impl FnOnce(
            &mut ForgeQueryGraphCompositionBuilder,
        ) -> Result<(), ForgeQueryRuntimeError>,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.compose_graph_with_invariant_pack(declaration, |_context| Ok(()))
    }

    pub fn compose_graph_with_invariant_pack(
        &mut self,
        declaration: impl FnOnce(
            &mut ForgeQueryGraphCompositionBuilder,
        ) -> Result<(), ForgeQueryRuntimeError>,
        invariant_pack: impl FnOnce(
            &ForgeQueryGraphCompositionInvariantPackContext<'_>,
        )
            -> Result<(), ForgeQueryGraphCompositionInvariantPackViolation>,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.compose_graph_with_invariant_gate(declaration, invariant_pack, |violation, context| {
            ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(
                ForgeQueryGraphCompositionDomainInvariantDenial::from_violation(violation, context),
            )
        })
    }

    pub fn compose_graph_with_domain_invariant_denial(
        &mut self,
        declaration: impl FnOnce(
            &mut ForgeQueryGraphCompositionBuilder,
        ) -> Result<(), ForgeQueryRuntimeError>,
        invariant_denial: impl FnOnce(
            &ForgeQueryGraphCompositionInvariantPackContext<'_>,
        )
            -> Result<(), ForgeQueryGraphCompositionDomainInvariantDenial>,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.compose_graph_with_invariant_gate(declaration, invariant_denial, |denial, _context| {
            ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(denial)
        })
    }

    fn compose_graph_with_invariant_gate<E>(
        &mut self,
        declaration: impl FnOnce(
            &mut ForgeQueryGraphCompositionBuilder,
        ) -> Result<(), ForgeQueryRuntimeError>,
        invariant_gate: impl FnOnce(
            &ForgeQueryGraphCompositionInvariantPackContext<'_>,
        ) -> Result<(), E>,
        map_invariant_error: impl FnOnce(
            E,
            &ForgeQueryGraphCompositionInvariantPackContext<'_>,
        ) -> ForgeQueryRuntimeError,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let mut builder = ForgeQueryGraphCompositionBuilder::new();
        declaration(&mut builder)?;
        let (commands, breadth, program) = builder.finish()?;
        let invariant_context =
            ForgeQueryGraphCompositionInvariantPackContext::new(&commands, &breadth, &program);
        invariant_gate(&invariant_context)
            .map_err(|error| map_invariant_error(error, &invariant_context))?;
        match self.runtime.write_graph_batch(commands, breadth, program) {
            Err(ForgeQueryRuntimeError::ExistingTruthAssertionDenied(denial)) => {
                let kind = match denial.kind() {
                    ForgeQueryExistingTruthAssertionDenialKind::BackendVerificationUnsupported => {
                        ForgeQueryGraphCompositionDenialKind::ExistingTargetBackendVerificationUnsupported
                    }
                    ForgeQueryExistingTruthAssertionDenialKind::ClearAssertionUnsupported => {
                        ForgeQueryGraphCompositionDenialKind::ExistingTargetClearAssertionUnsupported
                    }
                    ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect => {
                        ForgeQueryGraphCompositionDenialKind::ExistingTargetMissingAssertedAspect
                    }
                    ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch => {
                        ForgeQueryGraphCompositionDenialKind::ExistingTargetAssertedValueMismatch
                    }
                };
                Err(graph_composition_error(
                    kind,
                    None,
                    denial.binding().target_collection().map(str::to_string),
                    denial.message().to_string(),
                ))
            }
            Err(ForgeQueryRuntimeError::MutationBindingDenied(denial)) => {
                let kind = match denial.kind() {
                    ForgeQueryExistingTruthBindingDenialKind::UnsupportedFamily => {
                        ForgeQueryGraphCompositionDenialKind::ExistingTargetBindingUnsupported
                    }
                    ForgeQueryExistingTruthBindingDenialKind::ResolvedTargetMissing => {
                        ForgeQueryGraphCompositionDenialKind::ExistingTargetResolvedTargetMissing
                    }
                    ForgeQueryExistingTruthBindingDenialKind::CollectionMismatch => {
                        ForgeQueryGraphCompositionDenialKind::ExistingTargetCollectionMismatch
                    }
                };
                Err(graph_composition_error(
                    kind,
                    None,
                    denial.binding().target_collection().map(str::to_string),
                    denial.message().to_string(),
                ))
            }
            Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(denial)) => {
                let kind = match denial.kind() {
                    ForgeQuerySymbolicTargetReferenceDenialKind::UnresolvedSameBatchTarget => {
                        ForgeQueryGraphCompositionDenialKind::UnresolvedSymbolicReference
                    }
                    ForgeQuerySymbolicTargetReferenceDenialKind::CollectionMismatch => {
                        ForgeQueryGraphCompositionDenialKind::SymbolicCollectionMismatch
                    }
                    ForgeQuerySymbolicTargetReferenceDenialKind::RequiresBatchContext => {
                        return Err(ForgeQueryRuntimeError::MutationTargetReferenceDenied(
                            denial,
                        ));
                    }
                };
                Err(graph_composition_error(
                    kind,
                    Some(denial.reference().symbol().to_string()),
                    denial.reference().target_collection().map(str::to_string),
                    denial.message().to_string(),
                ))
            }
            other => other,
        }
    }
}
