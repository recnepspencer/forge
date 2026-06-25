use super::{
    WorthUiExpressionArity, WorthUiExpressionCostPosture, WorthUiExpressionDependencyContract,
    WorthUiExpressionDiagnosticsPosture, WorthUiExpressionInputKind,
    WorthUiExpressionOperatorDescriptor, WorthUiExpressionOperatorId, WorthUiExpressionOutputKind,
};
use crate::runtime::WorthUiSemanticSliceId;

pub const FIELD_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.field");
pub const LITERAL_TEXT_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.literal.text");
pub const PRESENT_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.present");
pub const EQUALS_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.equals");
pub const AND_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.and");
pub const OR_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.or");
pub const NOT_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.not");
pub const ONE_OF_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.one-of");
pub const EMPTY_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.empty");
pub const NON_EMPTY_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.non-empty");
pub const NORMALIZE_TRIM_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.normalize.trim");
pub const PAYLOAD_OBJECT_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.payload.object");
pub const DATA_PAYLOAD_OBJECT_OPERATOR: WorthUiExpressionOperatorId =
    WorthUiExpressionOperatorId::new("worth.expression.payload.data-object");

pub fn standard_expression_operator_descriptors() -> Vec<WorthUiExpressionOperatorDescriptor> {
    [
        standard_expression_operator_descriptor(FIELD_OPERATOR),
        standard_expression_operator_descriptor(LITERAL_TEXT_OPERATOR),
        standard_expression_operator_descriptor(PRESENT_OPERATOR),
        standard_expression_operator_descriptor(EQUALS_OPERATOR),
        standard_expression_operator_descriptor(AND_OPERATOR),
        standard_expression_operator_descriptor(OR_OPERATOR),
        standard_expression_operator_descriptor(NOT_OPERATOR),
        standard_expression_operator_descriptor(ONE_OF_OPERATOR),
        standard_expression_operator_descriptor(EMPTY_OPERATOR),
        standard_expression_operator_descriptor(NON_EMPTY_OPERATOR),
        standard_expression_operator_descriptor(NORMALIZE_TRIM_OPERATOR),
        standard_expression_operator_descriptor(PAYLOAD_OBJECT_OPERATOR),
        standard_expression_operator_descriptor(DATA_PAYLOAD_OBJECT_OPERATOR),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn standard_expression_operator_descriptor(
    operator_id: WorthUiExpressionOperatorId,
) -> Option<WorthUiExpressionOperatorDescriptor> {
    match operator_id.as_str() {
        "worth.expression.field" => Some(WorthUiExpressionOperatorDescriptor::new(
            FIELD_OPERATOR,
            vec![WorthUiExpressionInputKind::BindingReference],
            WorthUiExpressionOutputKind::Text,
            WorthUiExpressionArity::exact(1),
            WorthUiExpressionDependencyContract::BindingReference,
            WorthUiExpressionCostPosture::SingleBindingLookup,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionProjection,
        )),
        "worth.expression.literal.text" => Some(WorthUiExpressionOperatorDescriptor::new(
            LITERAL_TEXT_OPERATOR,
            vec![WorthUiExpressionInputKind::TextLiteral],
            WorthUiExpressionOutputKind::Text,
            WorthUiExpressionArity::exact(1),
            WorthUiExpressionDependencyContract::NoRuntimeFacts,
            WorthUiExpressionCostPosture::Constant,
            WorthUiExpressionDiagnosticsPosture::SchemaReferenced,
            WorthUiSemanticSliceId::ExpressionProjection,
        )),
        "worth.expression.present" => Some(WorthUiExpressionOperatorDescriptor::new(
            PRESENT_OPERATOR,
            vec![WorthUiExpressionInputKind::BindingReference],
            WorthUiExpressionOutputKind::Boolean,
            WorthUiExpressionArity::exact(1),
            WorthUiExpressionDependencyContract::BindingReference,
            WorthUiExpressionCostPosture::SingleBindingLookup,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.equals" => Some(WorthUiExpressionOperatorDescriptor::new(
            EQUALS_OPERATOR,
            vec![
                WorthUiExpressionInputKind::BindingReference,
                WorthUiExpressionInputKind::TextLiteral,
            ],
            WorthUiExpressionOutputKind::Boolean,
            WorthUiExpressionArity::exact(2),
            WorthUiExpressionDependencyContract::BindingReferenceAndLiteral,
            WorthUiExpressionCostPosture::SingleBindingLookup,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.and" => Some(WorthUiExpressionOperatorDescriptor::new(
            AND_OPERATOR,
            vec![WorthUiExpressionInputKind::BooleanExpression],
            WorthUiExpressionOutputKind::Boolean,
            WorthUiExpressionArity::at_least(1),
            WorthUiExpressionDependencyContract::NestedBooleanExpressions,
            WorthUiExpressionCostPosture::NestedExpressionLinear,
            WorthUiExpressionDiagnosticsPosture::SchemaReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.or" => Some(WorthUiExpressionOperatorDescriptor::new(
            OR_OPERATOR,
            vec![WorthUiExpressionInputKind::BooleanExpression],
            WorthUiExpressionOutputKind::Boolean,
            WorthUiExpressionArity::at_least(1),
            WorthUiExpressionDependencyContract::NestedBooleanExpressions,
            WorthUiExpressionCostPosture::NestedExpressionLinear,
            WorthUiExpressionDiagnosticsPosture::SchemaReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.not" => Some(WorthUiExpressionOperatorDescriptor::new(
            NOT_OPERATOR,
            vec![WorthUiExpressionInputKind::BooleanExpression],
            WorthUiExpressionOutputKind::Boolean,
            WorthUiExpressionArity::exact(1),
            WorthUiExpressionDependencyContract::NestedBooleanExpressions,
            WorthUiExpressionCostPosture::NestedExpressionLinear,
            WorthUiExpressionDiagnosticsPosture::SchemaReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.one-of" => Some(WorthUiExpressionOperatorDescriptor::new(
            ONE_OF_OPERATOR,
            vec![
                WorthUiExpressionInputKind::BindingReference,
                WorthUiExpressionInputKind::TextLiteral,
            ],
            WorthUiExpressionOutputKind::Boolean,
            WorthUiExpressionArity::at_least(2),
            WorthUiExpressionDependencyContract::BindingReferenceAndLiteral,
            WorthUiExpressionCostPosture::SingleBindingLookup,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.empty" => Some(WorthUiExpressionOperatorDescriptor::new(
            EMPTY_OPERATOR,
            vec![WorthUiExpressionInputKind::BindingReference],
            WorthUiExpressionOutputKind::Boolean,
            WorthUiExpressionArity::exact(1),
            WorthUiExpressionDependencyContract::BindingReference,
            WorthUiExpressionCostPosture::SingleBindingLookup,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.non-empty" => Some(WorthUiExpressionOperatorDescriptor::new(
            NON_EMPTY_OPERATOR,
            vec![WorthUiExpressionInputKind::BindingReference],
            WorthUiExpressionOutputKind::Boolean,
            WorthUiExpressionArity::exact(1),
            WorthUiExpressionDependencyContract::BindingReference,
            WorthUiExpressionCostPosture::SingleBindingLookup,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.normalize.trim" => Some(WorthUiExpressionOperatorDescriptor::new(
            NORMALIZE_TRIM_OPERATOR,
            vec![WorthUiExpressionInputKind::BindingReference],
            WorthUiExpressionOutputKind::Text,
            WorthUiExpressionArity::exact(1),
            WorthUiExpressionDependencyContract::BindingReference,
            WorthUiExpressionCostPosture::SingleBindingLookup,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.payload.object" => Some(WorthUiExpressionOperatorDescriptor::new(
            PAYLOAD_OBJECT_OPERATOR,
            vec![WorthUiExpressionInputKind::BindingSet],
            WorthUiExpressionOutputKind::PayloadObject,
            WorthUiExpressionArity::exact(1),
            WorthUiExpressionDependencyContract::BindingSet,
            WorthUiExpressionCostPosture::BindingSetLinear,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        "worth.expression.payload.data-object" => Some(WorthUiExpressionOperatorDescriptor::new(
            DATA_PAYLOAD_OBJECT_OPERATOR,
            vec![WorthUiExpressionInputKind::BindingSet],
            WorthUiExpressionOutputKind::PayloadObject,
            WorthUiExpressionArity::exact(1),
            WorthUiExpressionDependencyContract::BindingSet,
            WorthUiExpressionCostPosture::BindingSetLinear,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        )),
        _ => None,
    }
}
