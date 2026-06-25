use crate::capability::{
    AND_OPERATOR, DATA_PAYLOAD_OBJECT_OPERATOR, EQUALS_OPERATOR, PAYLOAD_OBJECT_OPERATOR,
    PRESENT_OPERATOR,
};
use crate::runtime::{
    WorthUiLiveViewConditionExpression, WorthUiLiveViewPayloadProjectionDeclaration,
    WorthUiLiveViewPayloadShape, WorthUiLiveViewReadinessProjectionDeclaration,
};

use super::{WorthUiLiveViewExpressionDeclaration, WorthUiLiveViewExpressionInput};

pub(crate) fn conditional_expression_declaration(
    live_view_id: &str,
    control_id: &str,
    condition: &WorthUiLiveViewConditionExpression,
) -> WorthUiLiveViewExpressionDeclaration {
    match condition {
        WorthUiLiveViewConditionExpression::BindingEqualsLiteral {
            binding_id,
            literal,
        } => WorthUiLiveViewExpressionDeclaration::new(
            format!("{live_view_id}:{control_id}:condition"),
            EQUALS_OPERATOR,
            vec![
                WorthUiLiveViewExpressionInput::BindingReference(binding_id.to_owned()),
                WorthUiLiveViewExpressionInput::TextLiteral(literal.to_owned()),
            ],
        ),
        WorthUiLiveViewConditionExpression::Unsupported(value) => {
            WorthUiLiveViewExpressionDeclaration::new(
                format!("{live_view_id}:{control_id}:unsupported-condition:{value}"),
                EQUALS_OPERATOR,
                Vec::new(),
            )
        }
    }
}

pub(crate) fn readiness_expression_declaration(
    live_view_id: &str,
    readiness: &WorthUiLiveViewReadinessProjectionDeclaration,
) -> WorthUiLiveViewExpressionDeclaration {
    let nested = readiness
        .required_bindings()
        .iter()
        .map(|binding_id| {
            WorthUiLiveViewExpressionInput::NestedExpression(Box::new(
                WorthUiLiveViewExpressionDeclaration::new(
                    format!(
                        "{live_view_id}:{}:present:{binding_id}",
                        readiness.readiness_id()
                    ),
                    PRESENT_OPERATOR,
                    vec![WorthUiLiveViewExpressionInput::BindingReference(
                        binding_id.to_owned(),
                    )],
                ),
            ))
        })
        .collect();
    WorthUiLiveViewExpressionDeclaration::new(
        format!("{live_view_id}:{}:requiredness", readiness.readiness_id()),
        AND_OPERATOR,
        nested,
    )
}

pub(crate) fn payload_expression_declaration<'a>(
    live_view_id: &str,
    payload: &WorthUiLiveViewPayloadProjectionDeclaration,
    consumed_binding_ids: impl IntoIterator<Item = &'a str>,
) -> WorthUiLiveViewExpressionDeclaration {
    let operator = match payload.shape() {
        WorthUiLiveViewPayloadShape::PayloadValues => PAYLOAD_OBJECT_OPERATOR,
        WorthUiLiveViewPayloadShape::DataPayloadValues => DATA_PAYLOAD_OBJECT_OPERATOR,
        WorthUiLiveViewPayloadShape::Unsupported(_) => PAYLOAD_OBJECT_OPERATOR,
    };
    WorthUiLiveViewExpressionDeclaration::new(
        format!("{live_view_id}:{}:payload-shape", payload.payload_id()),
        operator,
        vec![WorthUiLiveViewExpressionInput::BindingSet(
            consumed_binding_ids
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
        )],
    )
}
