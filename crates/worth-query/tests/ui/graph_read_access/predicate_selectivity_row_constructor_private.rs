use worth_foundational::facade::{AspectKey, FieldKey};
use worth_query::facade::runtime::{WorthQueryBooleanPredicateSelectivityRow, WorthQueryPredicateOperandOperator, WorthQueryPredicateSelectivityClass};

#[allow(unreachable_code)]
fn main() {
    let _ = WorthQueryBooleanPredicateSelectivityRow {
        aspect: AspectKey::new("status").unwrap(),
        field: FieldKey::new("value").unwrap(),
        family: "equality".to_string(),
        operand_identity: "eq:string:active".to_string(),
        operator: WorthQueryPredicateOperandOperator::Equal,
        normalized_operand_values: vec!["string:active".to_string()],
        field_kind: panic!("compile-fail fixture must not construct admitted evidence"),
        selectivity_class: WorthQueryPredicateSelectivityClass::ExactAnchor,
        pre_traversal_eligible: true,
    };
}
