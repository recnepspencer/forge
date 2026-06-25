use worth_ui::facade::{
    standard_expression_operator_descriptor, standard_expression_operator_descriptors,
    WorthUiExpressionCostPosture, WorthUiExpressionDependencyContract,
    WorthUiExpressionDiagnosticsPosture, WorthUiExpressionOutputKind, WorthUiSemanticSliceId,
    AND_OPERATOR, DATA_PAYLOAD_OBJECT_OPERATOR, EMPTY_OPERATOR, EQUALS_OPERATOR, FIELD_OPERATOR,
    LITERAL_TEXT_OPERATOR, NON_EMPTY_OPERATOR, NORMALIZE_TRIM_OPERATOR, NOT_OPERATOR,
    ONE_OF_OPERATOR, OR_OPERATOR, PAYLOAD_OBJECT_OPERATOR, PRESENT_OPERATOR,
};

#[test]
fn standard_expression_catalog_registers_bounded_pure_operators() {
    let descriptors = standard_expression_operator_descriptors();
    let operator_ids = descriptors
        .iter()
        .map(|descriptor| descriptor.operator_id().as_str())
        .collect::<Vec<_>>();

    for expected in [
        FIELD_OPERATOR,
        LITERAL_TEXT_OPERATOR,
        PRESENT_OPERATOR,
        EQUALS_OPERATOR,
        AND_OPERATOR,
        OR_OPERATOR,
        NOT_OPERATOR,
        ONE_OF_OPERATOR,
        EMPTY_OPERATOR,
        NON_EMPTY_OPERATOR,
        NORMALIZE_TRIM_OPERATOR,
        PAYLOAD_OBJECT_OPERATOR,
        DATA_PAYLOAD_OBJECT_OPERATOR,
    ] {
        let descriptor = standard_expression_operator_descriptor(expected)
            .expect("standard operator must be registered");
        assert!(operator_ids.contains(&expected.as_str()));
        assert!(descriptor.is_bounded());
        assert!(descriptor.is_pure());
        assert!(descriptor.descriptor_digest() > 0);
        assert!(descriptor.arity().admits(descriptor.arity().min()));
        assert!(matches!(
            descriptor.diagnostics_posture(),
            WorthUiExpressionDiagnosticsPosture::SchemaReferenced
                | WorthUiExpressionDiagnosticsPosture::DependencyReferenced
        ));
        assert!(matches!(
            descriptor.semantic_slice(),
            WorthUiSemanticSliceId::ExpressionOutput | WorthUiSemanticSliceId::ExpressionProjection
        ));
    }
}

#[test]
fn standard_expression_payload_operators_return_payload_objects() {
    for operator in [PAYLOAD_OBJECT_OPERATOR, DATA_PAYLOAD_OBJECT_OPERATOR] {
        let descriptor =
            standard_expression_operator_descriptor(operator).expect("payload operator registered");

        assert_eq!(
            descriptor.output_kind(),
            WorthUiExpressionOutputKind::PayloadObject
        );
        assert_eq!(
            descriptor.dependency_contract(),
            WorthUiExpressionDependencyContract::BindingSet
        );
        assert_eq!(
            descriptor.cost_posture(),
            WorthUiExpressionCostPosture::BindingSetLinear
        );
    }
}

#[test]
fn unknown_standard_expression_operator_does_not_fabricate_descriptor() {
    let descriptor = standard_expression_operator_descriptor(
        worth_ui::facade::WorthUiExpressionOperatorId::new("worth.expression.nope"),
    );

    assert!(descriptor.is_none());
}
