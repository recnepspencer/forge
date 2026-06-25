use super::super::expression::{
    conditional_expression_declaration, lower_live_view_expression_output,
    WorthUiLiveViewExpressionOutputValue, WorthUiLiveViewExpressionProjectionReceipt,
};
use crate::runtime::{
    WorthUiLiveViewConditionalProjectionGraphPosture, WorthUiLiveViewControlProjectionReceipt,
    WorthUiLiveViewDeclarationReceipt, WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

use super::declaration::{
    WorthUiLiveViewConditionExpression, WorthUiLiveViewConditionalProjectionDeclaration,
    WorthUiLiveViewParticipationPosture,
};
use super::denial::{
    WorthUiLiveViewConditionalProjectionAdmissionReport, WorthUiLiveViewConditionalProjectionDenial,
};
use super::receipt::{
    WorthUiLiveViewConditionalProjectionAdmissionCounters,
    WorthUiLiveViewConditionalProjectionReceipt,
};

impl WorthUiRuntimeHost {
    pub fn admit_live_view_conditional_projections(
        &self,
        live_view: &WorthUiLiveViewDeclarationReceipt,
        controls: &[WorthUiLiveViewControlProjectionReceipt],
        declarations: &[WorthUiLiveViewConditionalProjectionDeclaration],
    ) -> Result<
        Vec<WorthUiLiveViewConditionalProjectionReceipt>,
        WorthUiLiveViewConditionalProjectionAdmissionReport,
    > {
        let denials = conditional_projection_denials(live_view, controls, declarations);
        if !denials.is_empty() {
            return Err(WorthUiLiveViewConditionalProjectionAdmissionReport::denied(
                denials,
            ));
        }
        Ok(lower_live_view_conditional_projection_receipts(
            self,
            live_view,
            controls,
            declarations,
        ))
    }

    pub fn live_view_conditional_projection_admission_counters(
        &self,
        declarations: &[WorthUiLiveViewConditionalProjectionDeclaration],
        denial_count: usize,
    ) -> WorthUiLiveViewConditionalProjectionAdmissionCounters {
        WorthUiLiveViewConditionalProjectionAdmissionCounters::new(
            declarations.len(),
            declarations.len(),
            denial_count,
        )
    }
}

pub(crate) fn conditional_projection_denials(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    controls: &[WorthUiLiveViewControlProjectionReceipt],
    declarations: &[WorthUiLiveViewConditionalProjectionDeclaration],
) -> Vec<WorthUiLiveViewConditionalProjectionDenial> {
    let mut denials = Vec::new();
    for declaration in declarations {
        if !controls
            .iter()
            .any(|control| control.control_id() == declaration.control_id())
        {
            denials.push(WorthUiLiveViewConditionalProjectionDenial::UnknownControl {
                control_id: declaration.control_id().to_owned(),
            });
        }
        append_condition_denials(&mut denials, live_view, declaration);
        append_participation_denials(&mut denials, declaration);
    }
    denials
}

pub(crate) fn lower_live_view_conditional_projection_receipts(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    controls: &[WorthUiLiveViewControlProjectionReceipt],
    declarations: &[WorthUiLiveViewConditionalProjectionDeclaration],
) -> Vec<WorthUiLiveViewConditionalProjectionReceipt> {
    declarations
        .iter()
        .map(|declaration| {
            let control = controls
                .iter()
                .find(|control| control.control_id() == declaration.control_id())
                .expect("conditional control was admitted before lowering")
                .clone();
            let consumed_binding = live_view
                .binding(
                    declaration
                        .condition()
                        .consumed_binding_id()
                        .expect("condition binding was admitted before lowering"),
                )
                .expect("condition binding was admitted before lowering")
                .clone();
            let expression_declaration = conditional_expression_declaration(
                live_view.live_view_id(),
                declaration.control_id(),
                declaration.condition(),
            );
            let mut dependency_facts = conditional_projection_dependency_facts(
                live_view,
                declaration,
                &control,
                &consumed_binding,
            );
            let expression_projection = lower_live_view_expression_output(
                runtime,
                live_view,
                &expression_declaration,
                dependency_facts.clone(),
            )
            .expect("conditional expression was admitted before lowering");
            dependency_facts.push(expression_projection.output_fact().clone());
            dependency_facts.sort();
            dependency_facts.dedup();
            let active_posture = active_participation_posture(declaration, &expression_projection);
            let graph_execution = runtime
                .graph_authority()
                .plan_live_view_conditional_projection_graph_operation(
                    live_view.live_view_id(),
                    declaration.control_id(),
                    dependency_facts,
                    WorthUiLiveViewConditionalProjectionGraphPosture::Admitted,
                )
                .into_execution_receipt();
            WorthUiLiveViewConditionalProjectionReceipt::new(
                live_view.live_view_id(),
                declaration,
                control,
                consumed_binding,
                expression_projection,
                active_posture,
                graph_execution,
            )
        })
        .collect()
}

fn append_condition_denials(
    denials: &mut Vec<WorthUiLiveViewConditionalProjectionDenial>,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewConditionalProjectionDeclaration,
) {
    match declaration.condition() {
        WorthUiLiveViewConditionExpression::BindingEqualsLiteral { binding_id, .. } => {
            if live_view.binding(binding_id).is_none() {
                denials.push(
                    WorthUiLiveViewConditionalProjectionDenial::UnknownConditionBinding {
                        control_id: declaration.control_id().to_owned(),
                        binding_id: binding_id.to_owned(),
                    },
                );
            }
        }
        WorthUiLiveViewConditionExpression::Unsupported(value) => {
            denials.push(
                WorthUiLiveViewConditionalProjectionDenial::UnsupportedCondition {
                    control_id: declaration.control_id().to_owned(),
                    condition: value.to_owned(),
                },
            );
        }
    }
}

fn append_participation_denials(
    denials: &mut Vec<WorthUiLiveViewConditionalProjectionDenial>,
    declaration: &WorthUiLiveViewConditionalProjectionDeclaration,
) {
    for posture in [declaration.when_true(), declaration.when_false()] {
        if !posture.is_supported() {
            denials.push(
                WorthUiLiveViewConditionalProjectionDenial::UnsupportedParticipation {
                    control_id: declaration.control_id().to_owned(),
                    posture: posture.token().to_owned(),
                },
            );
        }
    }
}

fn active_participation_posture(
    declaration: &WorthUiLiveViewConditionalProjectionDeclaration,
    expression_projection: &WorthUiLiveViewExpressionProjectionReceipt,
) -> WorthUiLiveViewParticipationPosture {
    let condition_matches = matches!(
        expression_projection.output().value(),
        WorthUiLiveViewExpressionOutputValue::Boolean(true)
    );
    if condition_matches {
        declaration.when_true()
    } else {
        declaration.when_false()
    }
}

fn conditional_projection_dependency_facts(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewConditionalProjectionDeclaration,
    control: &WorthUiLiveViewControlProjectionReceipt,
    consumed_binding: &crate::runtime::WorthUiLiveViewStateBindingReceipt,
) -> Vec<WorthUiRuntimeFactId> {
    vec![
        WorthUiRuntimeFactId::live_view_declaration(live_view.live_view_id()),
        WorthUiRuntimeFactId::live_view_control_projection(format!(
            "{}:{}",
            live_view.live_view_id(),
            control.control_id()
        )),
        WorthUiRuntimeFactId::live_view_state_binding(format!(
            "{}:{}",
            live_view.live_view_id(),
            consumed_binding.binding_id()
        )),
        WorthUiRuntimeFactId::live_view_state_value(consumed_binding.state_fact().as_str()),
        WorthUiRuntimeFactId::live_view_conditional_projection(format!(
            "{}:{}",
            live_view.live_view_id(),
            declaration.control_id()
        )),
        WorthUiRuntimeFactId::live_view_participation(format!(
            "{}:{}",
            live_view.live_view_id(),
            declaration.control_id()
        )),
    ]
}
