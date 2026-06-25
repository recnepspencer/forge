use crate::capability::{
    AND_OPERATOR, DATA_PAYLOAD_OBJECT_OPERATOR, EMPTY_OPERATOR, EQUALS_OPERATOR, FIELD_OPERATOR,
    LITERAL_TEXT_OPERATOR, NON_EMPTY_OPERATOR, NORMALIZE_TRIM_OPERATOR, NOT_OPERATOR,
    ONE_OF_OPERATOR, OR_OPERATOR, PAYLOAD_OBJECT_OPERATOR, PRESENT_OPERATOR,
};
use crate::runtime::{
    WorthUiLiveViewDeclarationReceipt, WorthUiLiveViewStateValue, WorthUiRuntimeHost,
};

use super::{
    WorthUiLiveViewExpressionDeclaration, WorthUiLiveViewExpressionInput,
    WorthUiLiveViewExpressionOutputReceipt, WorthUiLiveViewExpressionOutputValue,
};

pub(super) fn evaluate_expression(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> WorthUiLiveViewExpressionOutputReceipt {
    match declaration.operator_id().as_str() {
        id if id == FIELD_OPERATOR.as_str() => {
            text_output(binding_text(runtime, live_view, declaration))
        }
        id if id == LITERAL_TEXT_OPERATOR.as_str() => text_output(literal_text(declaration)),
        id if id == PRESENT_OPERATOR.as_str() => {
            boolean_output(binding_is_present(runtime, live_view, declaration))
        }
        id if id == EQUALS_OPERATOR.as_str() => {
            boolean_output(binding_equals_literal(runtime, live_view, declaration))
        }
        id if id == AND_OPERATOR.as_str() => boolean_output(nested_boolean_outputs_all_match(
            runtime,
            live_view,
            declaration,
        )),
        id if id == OR_OPERATOR.as_str() => boolean_output(nested_boolean_outputs_any_match(
            runtime,
            live_view,
            declaration,
        )),
        id if id == NOT_OPERATOR.as_str() => boolean_output(negated_nested_boolean_output(
            runtime,
            live_view,
            declaration,
        )),
        id if id == ONE_OF_OPERATOR.as_str() => {
            boolean_output(binding_matches_any_literal(runtime, live_view, declaration))
        }
        id if id == EMPTY_OPERATOR.as_str() => {
            boolean_output(!binding_is_present(runtime, live_view, declaration))
        }
        id if id == NON_EMPTY_OPERATOR.as_str() => {
            boolean_output(binding_is_present(runtime, live_view, declaration))
        }
        id if id == NORMALIZE_TRIM_OPERATOR.as_str() => text_output(
            binding_text(runtime, live_view, declaration)
                .trim()
                .to_owned(),
        ),
        id if id == PAYLOAD_OBJECT_OPERATOR.as_str() => payload_output("payload"),
        id if id == DATA_PAYLOAD_OBJECT_OPERATOR.as_str() => payload_output("data"),
        _ => unreachable!("expression operator was admitted before evaluation"),
    }
}

fn binding_equals_literal(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> bool {
    binding_text(runtime, live_view, declaration) == literal_text(declaration)
}

fn nested_boolean_outputs_all_match(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> bool {
    declaration
        .inputs()
        .iter()
        .all(|input| nested_boolean_output(runtime, live_view, input).unwrap_or(false))
}

fn nested_boolean_outputs_any_match(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> bool {
    declaration
        .inputs()
        .iter()
        .any(|input| nested_boolean_output(runtime, live_view, input).unwrap_or(false))
}

fn negated_nested_boolean_output(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> bool {
    declaration
        .inputs()
        .iter()
        .find_map(|input| nested_boolean_output(runtime, live_view, input))
        .map(|value| !value)
        .unwrap_or(false)
}

fn nested_boolean_output(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    input: &WorthUiLiveViewExpressionInput,
) -> Option<bool> {
    match input {
        WorthUiLiveViewExpressionInput::NestedExpression(nested) => {
            evaluate_expression(runtime, live_view, nested)
                .value()
                .as_boolean()
        }
        _ => None,
    }
}

fn binding_matches_any_literal(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> bool {
    let value = binding_text(runtime, live_view, declaration);
    declaration
        .inputs()
        .iter()
        .any(|input| matches!(input, WorthUiLiveViewExpressionInput::TextLiteral(literal) if literal == &value))
}

fn binding_text(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> String {
    declaration
        .inputs()
        .iter()
        .find_map(|input| match input {
            WorthUiLiveViewExpressionInput::BindingReference(binding_id) => live_view
                .binding(binding_id)
                .and_then(|binding| runtime.live_view_state_value(binding))
                .map(WorthUiLiveViewStateValue::as_display_text),
            _ => None,
        })
        .unwrap_or_default()
}

fn binding_is_present(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> bool {
    !binding_text(runtime, live_view, declaration)
        .trim()
        .is_empty()
}

fn literal_text(declaration: &WorthUiLiveViewExpressionDeclaration) -> String {
    declaration
        .inputs()
        .iter()
        .find_map(|input| match input {
            WorthUiLiveViewExpressionInput::TextLiteral(value) => Some(value.clone()),
            _ => None,
        })
        .unwrap_or_default()
}

fn boolean_output(value: bool) -> WorthUiLiveViewExpressionOutputReceipt {
    WorthUiLiveViewExpressionOutputReceipt::new(
        WorthUiLiveViewExpressionOutputValue::Boolean(value),
        crate::capability::WorthUiExpressionOutputKind::Boolean,
    )
}

fn payload_output(shape: &str) -> WorthUiLiveViewExpressionOutputReceipt {
    WorthUiLiveViewExpressionOutputReceipt::new(
        WorthUiLiveViewExpressionOutputValue::PayloadShape(shape.to_owned()),
        crate::capability::WorthUiExpressionOutputKind::PayloadObject,
    )
}

fn text_output(value: String) -> WorthUiLiveViewExpressionOutputReceipt {
    WorthUiLiveViewExpressionOutputReceipt::new(
        WorthUiLiveViewExpressionOutputValue::Text(value),
        crate::capability::WorthUiExpressionOutputKind::Text,
    )
}

impl WorthUiLiveViewExpressionOutputValue {
    fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            Self::PayloadShape(_) | Self::Text(_) => None,
        }
    }
}
