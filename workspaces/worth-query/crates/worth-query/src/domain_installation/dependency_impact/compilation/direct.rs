use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::operation_phase_basis;
use crate::domain_installation::{
    WorthQueryBoundDomainOperation, WorthQueryBoundGraphExecutionReceipt,
    WorthQueryConditionalProvenance,
};

use super::super::compiled::{
    WorthQueryCompiledSemanticAspectDependency, WorthQueryCompiledSemanticAspectDependencyClosure,
    WorthQuerySemanticAspectDependencyCompilationCounters, WorthQuerySemanticAspectDependencyLocus,
    WorthQuerySemanticAspectDependencySource, WorthQuerySemanticDependencyRole,
};
use super::operation_definition::SemanticAspectDependencyCompilation;
use super::{
    WorthQuerySemanticAspectDependencyCompilationDenial,
    WorthQuerySemanticAspectDependencyCompilationDenialKind,
};

pub(crate) fn compile_direct_semantic_aspect_dependencies<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    graph_receipts: &[WorthQueryBoundGraphExecutionReceipt],
    conditionals: &[WorthQueryConditionalProvenance],
    execution: &crate::domain_installation::WorthQueryBoundExecutionReceipt,
    publication: &crate::domain_installation::WorthQueryDerivedPublicationReceipt,
) -> Result<
    WorthQueryCompiledSemanticAspectDependencyClosure,
    WorthQuerySemanticAspectDependencyCompilationDenial,
> {
    let mut counters = WorthQuerySemanticAspectDependencyCompilationCounters::default();
    let semantics = bound.definition().semantics();
    counters.semantic_contract_checks += 1;
    if !matches!(
        semantics.effects,
        worth_query_installation::facade::WorthQueryOperationEffectContract::NotRequired
    ) || !matches!(
        semantics.invariants,
        worth_query_installation::facade::WorthQueryOperationInvariantContract::NotRequired
    ) || !matches!(
        semantics.lineage,
        worth_query_installation::facade::WorthQueryOperationLineageContract::NotRequired
    ) {
        return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
            WorthQuerySemanticAspectDependencyCompilationDenialKind::DirectExecutionCannotRealizeSemanticContract,
            counters,
        ));
    }
    counters.execution_receipt_checks += 1;
    if execution.binding_identity() != bound.binding_identity() {
        return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
            WorthQuerySemanticAspectDependencyCompilationDenialKind::DirectExecutionReceiptMismatch,
            counters,
        ));
    }
    counters.execution_receipt_checks += 1;
    if publication.execution_identity() != execution.identity() {
        return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
            WorthQuerySemanticAspectDependencyCompilationDenialKind::DirectExecutionReceiptMismatch,
            counters,
        ));
    }
    let touch_roles = match &semantics.touches {
        worth_query_installation::facade::WorthQueryOperationTouchContract::Declared {
            graph_roles,
            ..
        } => graph_roles.as_slice(),
        worth_query_installation::facade::WorthQueryOperationTouchContract::NotRequired => &[],
    };
    counters.graph_receipt_checks += 1;
    if !super::graph_calls::realized_calls_match(bound, None, touch_roles, graph_receipts) {
        return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
            WorthQuerySemanticAspectDependencyCompilationDenialKind::RealizedGraphReceiptMismatch,
            counters,
        ));
    }
    let authority_basis = operation_phase_basis(bound.authority_proof());
    for conditional in conditionals {
        counters.conditional_authority_checks += 1;
        if operation_phase_basis(&conditional._admission) != authority_basis {
            return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
                WorthQuerySemanticAspectDependencyCompilationDenialKind::RealizedConditionalAuthorityMismatch,
                counters,
            ));
        }
    }
    let mut compilation = SemanticAspectDependencyCompilation::from_bound(bound, counters)?;
    for (call_ordinal, receipt) in graph_receipts.iter().enumerate() {
        compilation.push_realized_graph_call(
            WorthQuerySemanticAspectDependencyLocus::DirectGraphCall { call_ordinal },
            receipt,
        );
    }
    if let Err(kind) =
        compilation.push_realized_conditionals(&conditionals.iter().collect::<Vec<_>>())
    {
        return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
            kind,
            compilation.counters,
        ));
    }
    compilation
        .dependencies
        .push(WorthQueryCompiledSemanticAspectDependency::new(
            WorthQuerySemanticAspectDependencyLocus::DirectOutput,
            WorthQuerySemanticDependencyRole::ProjectedValue,
            WorthQuerySemanticAspectDependencySource::RealizedDirectOutput {
                result_state: execution.result_state(),
                output_identity: execution.output_identity().to_owned(),
                publication: publication.clone(),
            },
        ));
    compilation.counters.realized_direct_output_edges += 1;
    compilation.finish(bound)
}
