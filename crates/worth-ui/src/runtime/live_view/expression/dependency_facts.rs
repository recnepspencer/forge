use crate::runtime::{
    WorthUiLiveViewDeclarationReceipt, WorthUiLiveViewStateBindingReceipt, WorthUiRuntimeFactId,
};

use super::{WorthUiLiveViewExpressionDeclaration, WorthUiLiveViewExpressionInput};

pub(super) fn expression_dependency_facts(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> Vec<WorthUiRuntimeFactId> {
    let mut facts = expression_identity_facts(live_view, declaration);
    collect_expression_binding_facts(live_view, declaration, &mut facts);
    facts
}

fn expression_identity_facts(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> Vec<WorthUiRuntimeFactId> {
    vec![
        WorthUiRuntimeFactId::live_view_declaration(live_view.live_view_id()),
        WorthUiRuntimeFactId::live_view_expression_declaration(format!(
            "{}:{}",
            live_view.live_view_id(),
            declaration.expression_id()
        )),
    ]
}

fn collect_expression_binding_facts(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
    facts: &mut Vec<WorthUiRuntimeFactId>,
) {
    for input in declaration.inputs() {
        collect_input_binding_facts(live_view, input, facts);
    }
}

fn collect_input_binding_facts(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    input: &WorthUiLiveViewExpressionInput,
    facts: &mut Vec<WorthUiRuntimeFactId>,
) {
    match input {
        WorthUiLiveViewExpressionInput::BindingReference(binding_id) => {
            if let Some(binding) = live_view.binding(binding_id) {
                push_binding_facts(binding, facts);
            }
        }
        WorthUiLiveViewExpressionInput::BindingSet(binding_ids) => {
            for binding_id in binding_ids {
                if let Some(binding) = live_view.binding(binding_id) {
                    push_binding_facts(binding, facts);
                }
            }
        }
        WorthUiLiveViewExpressionInput::NestedExpression(nested) => {
            collect_expression_binding_facts(live_view, nested, facts);
        }
        WorthUiLiveViewExpressionInput::TextLiteral(_) => {}
    }
}

fn push_binding_facts(
    binding: &WorthUiLiveViewStateBindingReceipt,
    facts: &mut Vec<WorthUiRuntimeFactId>,
) {
    facts.push(WorthUiRuntimeFactId::live_view_state_binding(format!(
        "{}:{}",
        binding.live_view_id(),
        binding.binding_id()
    )));
    facts.push(WorthUiRuntimeFactId::live_view_state_value(
        binding.state_fact().as_str(),
    ));
}
