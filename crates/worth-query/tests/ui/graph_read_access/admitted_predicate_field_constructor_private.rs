use worth_foundational::facade::{AspectKey, FieldKey};
use worth_query::facade::runtime::{WorthQueryAdmittedGraphReadPredicateField, WorthQueryGraphReadAdmittedSchemaFieldKind};

fn main() {
    let _ = WorthQueryAdmittedGraphReadPredicateField {
        aspect: AspectKey::new("status").unwrap(),
        field: FieldKey::new("value").unwrap(),
        family: "equality".to_string(),
        kind: WorthQueryGraphReadAdmittedSchemaFieldKind::String,
    };
}
