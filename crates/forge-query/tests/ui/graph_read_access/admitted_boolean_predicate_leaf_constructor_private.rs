use forge_foundational::facade::{AspectKey, FieldKey};
use forge_query::facade::runtime::{
    ForgeQueryAdmittedBooleanPredicateLeaf, ForgeQueryGraphReadAdmittedSchemaFieldKind,
    ForgeQueryPredicateOperandOperator, ForgeQueryPredicateSelectivityClass,
};

fn main() {
    let _ = ForgeQueryAdmittedBooleanPredicateLeaf {
        aspect: AspectKey::new("status").unwrap(),
        field: FieldKey::new("value").unwrap(),
        family: "equality".to_string(),
        operator: ForgeQueryPredicateOperandOperator::Equal,
        normalized_operand_values: vec!["string:active".to_string()],
        field_kind: ForgeQueryGraphReadAdmittedSchemaFieldKind::String,
        selectivity_class: ForgeQueryPredicateSelectivityClass::ExactAnchor,
    };
}
