use forge_foundational::facade::{AspectKey, FieldKey};
use forge_query::facade::runtime::{
    ForgeQueryAdmittedGraphReadPredicateField, ForgeQueryGraphReadAdmittedSchemaFieldKind,
};

fn main() {
    let _ = ForgeQueryAdmittedGraphReadPredicateField {
        aspect: AspectKey::new("status").unwrap(),
        field: FieldKey::new("value").unwrap(),
        family: "equality".to_string(),
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind::String,
    };
}
