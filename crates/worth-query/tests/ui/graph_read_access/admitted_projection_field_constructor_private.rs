use worth_foundational::facade::{AspectKey, FieldKey};
use worth_query::facade::runtime::{
    WorthQueryAdmittedGraphReadProjectionField, WorthQueryGraphReadAdmittedSchemaFieldKind,
};

fn main() {
    let _ = WorthQueryAdmittedGraphReadProjectionField {
        aspect: AspectKey::new("identity").unwrap(),
        field: FieldKey::new("id").unwrap(),
        delivered_name: "id".to_string(),
        kind: WorthQueryGraphReadAdmittedSchemaFieldKind::String,
    };
}
