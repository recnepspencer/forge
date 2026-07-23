use std::collections::HashMap;

use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::operation_phase_basis;
use crate::domain_installation::WorthQueryCompletedWorkflowTrace;

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

pub(crate) fn compile_workflow_semantic_aspect_dependencies<D, O, F, L: BasisOperationLane>(
    trace: &WorthQueryCompletedWorkflowTrace<D, O, F, L>,
) -> Result<
    WorthQueryCompiledSemanticAspectDependencyClosure,
    WorthQuerySemanticAspectDependencyCompilationDenial,
> {
    let mut counters = WorthQuerySemanticAspectDependencyCompilationCounters::default();
    let authority_basis = operation_phase_basis(trace.bound().authority_proof());
    for conditional in trace.operation_conditional_provenance().iter().chain(
        trace
            .stage_receipts()
            .iter()
            .flat_map(|receipt| receipt.conditional_provenance()),
    ) {
        counters.conditional_authority_checks += 1;
        if operation_phase_basis(&conditional._admission) != authority_basis {
            return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
                WorthQuerySemanticAspectDependencyCompilationDenialKind::RealizedConditionalAuthorityMismatch,
                counters,
            ));
        }
    }
    let mut compilation = SemanticAspectDependencyCompilation::from_bound(trace.bound(), counters)?;
    let mut realized_conditionals = trace
        .operation_conditional_provenance()
        .iter()
        .collect::<Vec<_>>();
    compilation.counters.workflow_trace_checks += 1;
    let worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(workflow) =
        &trace.bound().definition().semantics().workflow
    else {
        return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
            WorthQuerySemanticAspectDependencyCompilationDenialKind::WorkflowTraceMismatch,
            compilation.counters,
        ));
    };
    let mut receipt_by_stage = HashMap::with_capacity(trace.stage_receipts().len());
    for receipt in trace.stage_receipts() {
        compilation.counters.workflow_trace_checks += 1;
        receipt_by_stage.insert(receipt.stage_identity(), receipt);
    }
    compilation.counters.workflow_trace_checks += 1;
    if receipt_by_stage.len() != workflow.stages().len() {
        return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
            WorthQuerySemanticAspectDependencyCompilationDenialKind::WorkflowTraceMismatch,
            compilation.counters,
        ));
    }
    for stage in workflow.stages() {
        compilation.counters.workflow_trace_checks += 1;
        let Some(receipt) = receipt_by_stage.get(stage.identity()).copied() else {
            return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
                WorthQuerySemanticAspectDependencyCompilationDenialKind::WorkflowTraceMismatch,
                compilation.counters,
            ));
        };
        compilation.counters.graph_receipt_checks += 1;
        if !super::graph_calls::realized_calls_match(
            trace.bound(),
            Some(&stage.semantics().graph_read_roles),
            &stage.semantics().touch_roles,
            receipt.graph_receipts(),
        ) {
            return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
                WorthQuerySemanticAspectDependencyCompilationDenialKind::RealizedGraphReceiptMismatch,
                compilation.counters,
            ));
        }
        compilation
            .dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::WorkflowOutput {
                    stage_identity: receipt.stage_identity().to_owned(),
                },
                WorthQuerySemanticDependencyRole::ProjectedValue,
                WorthQuerySemanticAspectDependencySource::RealizedWorkflowOutput {
                    receipt_identity: receipt.identity().to_owned(),
                    semantic_output: receipt.output().semantic_value(),
                    result_state: receipt.result_state(),
                },
            ));
        compilation.counters.realized_workflow_output_edges += 1;
        for (call_ordinal, graph_receipt) in receipt.graph_receipts().iter().enumerate() {
            compilation.push_realized_graph_call(
                WorthQuerySemanticAspectDependencyLocus::WorkflowGraphCall {
                    stage_identity: receipt.stage_identity().to_owned(),
                    call_ordinal,
                },
                graph_receipt,
            );
        }
        push_stage_evidence(&mut compilation, receipt);
        for conditional in receipt.conditional_provenance() {
            realized_conditionals.push(conditional);
        }
    }
    if let Err(kind) = compilation.push_realized_conditionals(&realized_conditionals) {
        return Err(WorthQuerySemanticAspectDependencyCompilationDenial::new(
            kind,
            compilation.counters,
        ));
    }
    compilation.finish(trace.bound())
}

fn push_stage_evidence(
    compilation: &mut SemanticAspectDependencyCompilation,
    receipt: &crate::domain_installation::WorthQueryWorkflowStageReceipt,
) {
    let stage_identity = receipt.stage_identity();
    for (read_ordinal, evidence) in receipt.primary_read_evidence().iter().enumerate() {
        compilation
            .dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::WorkflowPrimaryRead {
                    stage_identity: stage_identity.to_owned(),
                    read_ordinal,
                },
                WorthQuerySemanticDependencyRole::ProjectedValue,
                WorthQuerySemanticAspectDependencySource::RealizedWorkflowRead(evidence.clone()),
            ));
        compilation.counters.realized_workflow_read_edges += 1;
    }
    for (effect_ordinal, evidence) in receipt.effect_evidence().iter().enumerate() {
        compilation
            .dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::WorkflowEffect {
                    stage_identity: stage_identity.to_owned(),
                    effect_ordinal,
                },
                WorthQuerySemanticDependencyRole::SupportAndLifecycle,
                WorthQuerySemanticAspectDependencySource::RealizedWorkflowEffect(evidence.clone()),
            ));
        compilation.counters.realized_effect_edges += 1;
    }
    for (invariant_ordinal, evidence) in receipt.invariant_outcomes().iter().enumerate() {
        compilation
            .dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::WorkflowInvariant {
                    stage_identity: stage_identity.to_owned(),
                    invariant_ordinal,
                },
                WorthQuerySemanticDependencyRole::InstalledDomainInvariant,
                WorthQuerySemanticAspectDependencySource::RealizedWorkflowInvariant(
                    evidence.clone(),
                ),
            ));
        compilation.counters.realized_invariant_edges += 1;
    }
    for (lineage_ordinal, evidence) in receipt.lineage.iter().enumerate() {
        compilation
            .dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                WorthQuerySemanticAspectDependencyLocus::WorkflowLineage {
                    stage_identity: stage_identity.to_owned(),
                    lineage_ordinal,
                },
                WorthQuerySemanticDependencyRole::OperationalIdentity,
                WorthQuerySemanticAspectDependencySource::RealizedWorkflowLineage(evidence.clone()),
            ));
        compilation.counters.realized_lineage_edges += 1;
    }
}
