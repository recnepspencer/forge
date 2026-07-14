use worth_foundational::facade::{AspectKey, FieldKey};
use worth_query::facade::runtime::{WorthQueryAdmittedBooleanPredicateLeaf, WorthQueryGraphReadAdmittedSchemaFieldKind, WorthQueryPredicateOperandOperator, WorthQueryPredicateSelectivityClass};

fn main() {
    let _ = WorthQueryAdmittedBooleanPredicateLeaf {
        aspect: AspectKey::new("status").unwrap(),
        field: FieldKey::new("value").unwrap(),
        family: "equality".to_string(),
        operator: WorthQueryPredicateOperandOperator::Equal,
        normalized_operand_values: vec!["string:active".to_string()],
        field_kind: WorthQueryGraphReadAdmittedSchemaFieldKind::String,
        selectivity_class: WorthQueryPredicateSelectivityClass::ExactAnchor,
    };
}
