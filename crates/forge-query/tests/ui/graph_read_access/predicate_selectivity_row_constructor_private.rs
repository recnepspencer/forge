use forge_foundational::facade::{AspectKey, FieldKey};
use forge_query::facade::runtime::{
    ForgeQueryBooleanPredicateSelectivityRow, ForgeQueryGraphReadAdmittedSchemaFieldKind,
    ForgeQueryPredicateOperandOperator, ForgeQueryPredicateSelectivityClass,
};

fn main() {
    let _ = ForgeQueryBooleanPredicateSelectivityRow {
        aspect: AspectKey::new("status").unwrap(),
        field: FieldKey::new("value").unwrap(),
        family: "equality".to_string(),
        operand_identity: "eq:string:active".to_string(),
        operator: ForgeQueryPredicateOperandOperator::Equal,
        normalized_operand_values: vec!["string:active".to_string()],
        field_kind: ForgeQueryGraphReadAdmittedSchemaFieldKind::String,
        selectivity_class: ForgeQueryPredicateSelectivityClass::ExactAnchor,
        pre_traversal_eligible: true,
    };
}
