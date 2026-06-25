use crate::runtime::{WorthUiLiveViewDeclarationReceipt, WorthUiRuntimeFactId, WorthUiRuntimeHost};

use super::admission::{admit_live_view_expression, WorthUiLiveViewExpressionAdmissionReport};
use super::dependency_facts::expression_dependency_facts;
use super::operator_evaluator::evaluate_expression;
use super::{WorthUiLiveViewExpressionDeclaration, WorthUiLiveViewExpressionProjectionReceipt};

pub(crate) fn lower_live_view_expression_output(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
    extra_facts: Vec<WorthUiRuntimeFactId>,
) -> Result<WorthUiLiveViewExpressionProjectionReceipt, WorthUiLiveViewExpressionAdmissionReport> {
    let admitted = admit_live_view_expression(live_view, declaration)?;
    let consumed_facts = consumed_expression_projection_facts(live_view, declaration, extra_facts);
    let output = evaluate_expression(runtime, live_view, declaration);
    let graph_execution = runtime
        .graph_authority()
        .plan_live_view_expression_projection_graph_operation(
            live_view.live_view_id(),
            declaration.expression_id(),
            consumed_facts.clone(),
        )
        .into_execution_receipt();
    Ok(WorthUiLiveViewExpressionProjectionReceipt::new(
        live_view.live_view_id(),
        admitted.declaration(),
        admitted.descriptor().clone(),
        consumed_facts,
        output,
        graph_execution,
    ))
}

fn consumed_expression_projection_facts(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
    extra_facts: Vec<WorthUiRuntimeFactId>,
) -> Vec<WorthUiRuntimeFactId> {
    let mut consumed_facts = expression_dependency_facts(live_view, declaration);
    consumed_facts.extend(extra_facts);
    consumed_facts.sort();
    consumed_facts.dedup();
    consumed_facts
}
